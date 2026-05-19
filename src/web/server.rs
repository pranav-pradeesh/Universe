use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::broadcast;

use crate::kernel::universe::Universe;
use crate::observer::Observer;

static INDEX_HTML: &str = include_str!("../../static/index.html");

#[derive(Clone)]
struct AppState {
    frame_tx: broadcast::Sender<Arc<Vec<u8>>>,
    paused: Arc<AtomicBool>,
    ticks_per_frame: Arc<AtomicUsize>,
    reset_tx: std::sync::mpsc::Sender<ResetCmd>,
}

#[derive(Debug)]
enum ResetCmd {
    Normal { seed: u64 },
    BigBang { seed: u64, energy: f32 },
}

pub fn start(universe: Universe, big_bang: bool, port: u16) {
    let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
    rt.block_on(run(universe, big_bang, port));
}

async fn run(universe: Universe, big_bang: bool, port: u16) {
    let (frame_tx, _) = broadcast::channel::<Arc<Vec<u8>>>(8);
    let paused = Arc::new(AtomicBool::new(false));
    let ticks_per_frame = Arc::new(AtomicUsize::new(5));
    let (reset_tx, reset_rx) = std::sync::mpsc::channel::<ResetCmd>();

    let state = AppState {
        frame_tx: frame_tx.clone(),
        paused: paused.clone(),
        ticks_per_frame: ticks_per_frame.clone(),
        reset_tx,
    };

    // Spawn CPU-bound simulation on a dedicated OS thread
    std::thread::spawn(move || {
        sim_loop(universe, big_bang, frame_tx, paused, ticks_per_frame, reset_rx);
    });

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/ws", get(ws_handler))
        .with_state(state);

    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("bind port");

    println!("┌──────────────────────────────────────────────────────┐");
    println!("│  Reality Engine V0.2 — Universe Visualization        │");
    println!("│  Open in browser: http://localhost:{port:<26}│");
    println!("│  Mode: {}                                  │", if big_bang { "Big Bang 🌌  " } else { "Normal Emergence" });
    println!("└──────────────────────────────────────────────────────┘");

    axum::serve(listener, app).await.expect("serve");
}

async fn serve_index() -> impl IntoResponse {
    Html(INDEX_HTML)
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, state))
}

async fn handle_ws(mut socket: WebSocket, state: AppState) {
    let mut frame_rx = state.frame_tx.subscribe();

    loop {
        tokio::select! {
            // Forward simulation frames to client
            Ok(frame) = frame_rx.recv() => {
                if socket.send(Message::Binary((*frame).clone())).await.is_err() {
                    break;
                }
            }
            // Receive control messages from client
            Some(Ok(msg)) = socket.recv() => {
                match msg {
                    Message::Text(txt) => handle_control(&txt, &state),
                    Message::Close(_) => break,
                    _ => {}
                }
            }
            else => break,
        }
    }
}

fn handle_control(txt: &str, state: &AppState) {
    let Ok(v) = serde_json::from_str::<serde_json::Value>(txt) else { return };
    match v["action"].as_str() {
        Some("pause") => { state.paused.store(true, Ordering::Relaxed); }
        Some("resume") => { state.paused.store(false, Ordering::Relaxed); }
        Some("speed") => {
            if let Some(tpf) = v["ticks_per_frame"].as_u64() {
                state.ticks_per_frame.store(tpf as usize, Ordering::Relaxed);
            }
        }
        Some("reset") => {
            let seed = v["seed"].as_u64().unwrap_or(42);
            let energy = v["energy"].as_f64().unwrap_or(1.5) as f32;
            let cmd = match v["mode"].as_str() {
                Some("big_bang") => ResetCmd::BigBang { seed, energy },
                _ => ResetCmd::Normal { seed },
            };
            let _ = state.reset_tx.send(cmd);
        }
        _ => {}
    }
}

fn sim_loop(
    mut universe: Universe,
    big_bang: bool,
    frame_tx: broadcast::Sender<Arc<Vec<u8>>>,
    paused: Arc<AtomicBool>,
    ticks_per_frame: Arc<AtomicUsize>,
    reset_rx: std::sync::mpsc::Receiver<ResetCmd>,
) {
    let node_count = universe.node_count;
    let mut observer = Observer::new(node_count);

    loop {
        // Non-blocking check for reset commands
        while let Ok(cmd) = reset_rx.try_recv() {
            let (seed, new_big_bang, energy) = match cmd {
                ResetCmd::Normal { seed } => (seed, false, 1.5f32),
                ResetCmd::BigBang { seed, energy } => (seed, true, energy),
            };
            universe = Universe::new(seed, node_count);
            if new_big_bang {
                universe.init_big_bang(energy);
            } else {
                universe.init();
            }
            observer = Observer::new(node_count);
        }

        if paused.load(Ordering::Relaxed) {
            std::thread::sleep(std::time::Duration::from_millis(16));
            continue;
        }

        let tpf = ticks_per_frame.load(Ordering::Relaxed).max(1);
        for _ in 0..tpf {
            universe.tick();
        }

        // Build binary frame
        let frame = build_frame(&universe, &mut observer);

        // Broadcast to all connected clients (ignore if no subscribers)
        let _ = frame_tx.send(Arc::new(frame));

        // Small sleep to maintain ~60 frame/sec broadcast rate and not pin the CPU
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

fn build_frame(universe: &Universe, observer: &mut Observer) -> Vec<u8> {
    let states = universe.states();
    let n = universe.node_count;
    let w = universe.grid_width;
    let h = w;

    let report = observer.observe(
        universe.tick,
        states,
        &universe.graph,
        universe.last_mutation.created,
        universe.last_mutation.destroyed,
    );

    // Binary layout:
    // [0..8]        tick u64 LE
    // [8..12]       width u32 LE
    // [12..16]      height u32 LE
    // [16..16+N]    psi u8 (quantized [-1,1]→[0,255])
    // [16+N..16+2N] chi u8 (quantized [0.2,2.5]→[0,255])
    // [16+2N..16+3N]omega u8 (quantized [-1,1]→[0,255])
    // [16+3N+0..4]  total_energy f32 LE
    // [16+3N+4..8]  plus_fraction f32 LE
    // [16+3N+8..12] minus_fraction f32 LE
    // [16+3N+12..16]neutral_fraction f32 LE
    // [16+3N+16..20]domain_wall_edges u32 LE
    // [16+3N+20]    symmetry_broken u8
    // [16+3N+21..25]max_stability u32 LE
    // [16+3N+25..29]kinetic_fraction f32 LE
    // [16+3N+29..33]energy_variance f32 LE

    let header = 16usize;
    let fields = 3 * n;
    let stats = 33usize;
    let total = header + fields + stats;
    let mut buf = vec![0u8; total];

    // Header
    buf[0..8].copy_from_slice(&universe.tick.to_le_bytes());
    buf[8..12].copy_from_slice(&(w as u32).to_le_bytes());
    buf[12..16].copy_from_slice(&(h as u32).to_le_bytes());

    // Field data
    for i in 0..n {
        let s = &states[i];
        buf[header + i] = quantize_signed(s.psi);
        buf[header + n + i] = quantize_chi(s.chi);
        buf[header + 2 * n + i] = quantize_signed(s.omega);
    }

    // Stats
    let so = header + fields;
    buf[so..so+4].copy_from_slice(&report.total_energy.to_le_bytes());
    buf[so+4..so+8].copy_from_slice(&report.plus_fraction.to_le_bytes());
    buf[so+8..so+12].copy_from_slice(&report.minus_fraction.to_le_bytes());
    let neutral = report.neutral_fraction();
    buf[so+12..so+16].copy_from_slice(&neutral.to_le_bytes());
    buf[so+16..so+20].copy_from_slice(&(report.domain_wall_edges as u32).to_le_bytes());
    buf[so+20] = report.symmetry_broken as u8;
    buf[so+21..so+25].copy_from_slice(&report.max_stability.to_le_bytes());
    buf[so+25..so+29].copy_from_slice(&report.kinetic_fraction.to_le_bytes());
    buf[so+29..so+33].copy_from_slice(&report.energy_variance.to_le_bytes());

    buf
}

fn quantize_signed(v: f32) -> u8 {
    ((v.clamp(-1.0, 1.0) + 1.0) * 0.5 * 255.0).round() as u8
}

fn quantize_chi(v: f32) -> u8 {
    ((v.clamp(0.2, 2.5) - 0.2) / 2.3 * 255.0).round() as u8
}
