use clap::Parser;
use std::time::{Duration, Instant};

mod kernel;
mod topology;
mod transform;
mod observer;
mod visualization;
mod replay;
mod web;

use kernel::universe::Universe;
use observer::Observer;
use replay::recorder::Recorder;
use visualization::ascii::{render_psi_grid, render_chi_grid, render_report, render_histogram};

#[derive(Parser, Debug)]
#[command(name = "reality-engine", version = "0.2.0")]
#[command(about = "Deterministic emergent reality substrate — V0.2")]
struct Args {
    /// Universe seed
    #[arg(short, long, default_value_t = 42)]
    seed: u64,

    /// Number of nodes (rounded to nearest perfect square)
    #[arg(short, long, default_value_t = 4096)]
    nodes: usize,

    /// Number of ticks to run (0 = infinite, ignored in --web mode)
    #[arg(short, long, default_value_t = 0)]
    ticks: u64,

    /// Observe and render every N ticks
    #[arg(short, long, default_value_t = 100)]
    observe_interval: u64,

    /// Show ψ field ASCII grid
    #[arg(long)]
    viz: bool,

    /// Show χ metric grid
    #[arg(long)]
    chi_viz: bool,

    /// Show ψ histogram
    #[arg(long)]
    histogram: bool,

    /// Record snapshots
    #[arg(long)]
    record: bool,

    /// Max snapshots to keep
    #[arg(long, default_value_t = 10)]
    max_snapshots: usize,

    /// Target ticks per second (0 = unlimited, terminal mode only)
    #[arg(long, default_value_t = 0)]
    tps: u64,

    /// Launch browser visualization server
    #[arg(long)]
    web: bool,

    /// Initialize as Big Bang (singularity at center)
    #[arg(long)]
    big_bang: bool,

    /// Web server port
    #[arg(long, default_value_t = 8080)]
    port: u16,

    /// Big Bang energy scale
    #[arg(long, default_value_t = 1.5)]
    energy: f32,
}

fn main() {
    let args = Args::parse();

    let mut universe = Universe::new(args.seed, args.nodes);

    if args.big_bang {
        universe.init_big_bang(args.energy);
    } else {
        universe.init();
    }

    if args.web {
        web::server::start(universe, args.big_bang, args.port);
        return;
    }

    // Terminal mode (unchanged from before)
    let actual_nodes = universe.node_count;
    let width = universe.grid_width;

    println!("Reality Engine V0.2 — Hamiltonian Field Dynamics");
    println!("══════════════════════════════════════════════════════════");
    println!("Seed: {}  Grid: {}×{}={}  Mode: {}",
        args.seed, width, width, actual_nodes,
        if args.big_bang { "Big Bang" } else { "Normal" });
    println!("══════════════════════════════════════════════════════════\n");

    let mut observer = Observer::new(actual_nodes);
    let mut recorder = if args.record { Some(Recorder::new(args.max_snapshots)) } else { None };

    let tick_duration = if args.tps > 0 {
        Some(Duration::from_micros(1_000_000 / args.tps))
    } else {
        None
    };

    let mut last_tick_time = Instant::now();
    let start_time = Instant::now();

    loop {
        universe.tick();
        let tick = universe.tick;

        if tick % args.observe_interval == 0 {
            let report = observer.observe(
                tick,
                universe.states(),
                &universe.graph,
                universe.last_mutation.created,
                universe.last_mutation.destroyed,
            );

            if let Some(ref mut rec) = recorder {
                rec.record(tick, universe.states(), Some(report.clone()));
            }

            let elapsed = start_time.elapsed().as_secs_f64();
            let tps_actual = tick as f64 / elapsed;

            print!("\x1b[2J\x1b[H");
            println!("Reality Engine V0.2  [{:.1}s  {:.0} ticks/s]", elapsed, tps_actual);
            println!("──────────────────────────────────────────────────────");

            if args.viz { print!("{}", render_psi_grid(universe.states(), width)); }
            if args.chi_viz { print!("{}", render_chi_grid(universe.states(), width)); }
            if args.histogram { print!("{}", render_histogram(universe.states())); }

            println!("{}", render_report(&report));

            if report.symmetry_broken && report.breaking_tick == Some(tick) {
                println!("  *** SPONTANEOUS SYMMETRY BREAKING at tick {} ***", tick);
            }
            if report.max_stability > 200 {
                println!("  [EMERGENCE] Stable structure: {} ticks", report.max_stability);
            }
            if report.compression_ratio > 0.7 {
                println!("  [EMERGENCE] Strong compression: {} macro-regions", report.macro_regions);
            }
        }

        if let Some(dur) = tick_duration {
            let elapsed = last_tick_time.elapsed();
            if elapsed < dur { std::thread::sleep(dur - elapsed); }
            last_tick_time = Instant::now();
        }

        if args.ticks > 0 && tick >= args.ticks {
            println!("\nSimulation complete at tick {}.", tick);
            break;
        }
    }
}
