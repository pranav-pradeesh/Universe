mod kernel;
mod topology;
mod transform;
mod observer;
mod visualization;
mod replay;

use clap::Parser;
use std::time::{Duration, Instant};

use kernel::universe::Universe;
use observer::Observer;
use replay::recorder::Recorder;
use visualization::ascii::{render_histogram, render_phi_grid, render_report};

#[derive(Parser, Debug)]
#[command(name = "reality-engine", version = "0.1.0")]
#[command(about = "Deterministic emergent reality substrate — V0.1")]
#[command(long_about = None)]
struct Args {
    /// Universe seed (determines all initial conditions deterministically)
    #[arg(short, long, default_value_t = 42)]
    seed: u64,

    /// Number of nodes in the universe
    #[arg(short, long, default_value_t = 512)]
    nodes: usize,

    /// Number of ticks to run (0 = run indefinitely)
    #[arg(short, long, default_value_t = 0)]
    ticks: u64,

    /// Run the observer every N ticks
    #[arg(short, long, default_value_t = 50)]
    observe_interval: u64,

    /// Show ASCII phi-field grid visualization
    #[arg(long)]
    viz: bool,

    /// Width of the ASCII visualization grid
    #[arg(long, default_value_t = 32)]
    grid_width: usize,

    /// Show phi histogram alongside the grid
    #[arg(long)]
    histogram: bool,

    /// Enable snapshot recording for replay
    #[arg(long)]
    record: bool,

    /// Maximum snapshots to keep in the rolling window
    #[arg(long, default_value_t = 10)]
    max_snapshots: usize,

    /// Target ticks per second for rate-limiting (0 = unlimited)
    #[arg(long, default_value_t = 0)]
    tps: u64,
}

fn main() {
    let args = Args::parse();

    println!("Reality Engine V0.1");
    println!("══════════════════════════════════════════════════════════");
    println!(
        "  Seed:  {}   Nodes: {}   Observe every: {} ticks",
        args.seed, args.nodes, args.observe_interval
    );
    println!(
        "  TPS limit: {}   Viz: {}   Histogram: {}",
        if args.tps == 0 {
            "unlimited".to_string()
        } else {
            args.tps.to_string()
        },
        args.viz,
        args.histogram,
    );
    println!("══════════════════════════════════════════════════════════\n");

    let mut universe = Universe::new(args.seed, args.nodes);
    universe.init();

    let mut observer = Observer::new(args.nodes);
    let mut recorder = args.record.then(|| Recorder::new(args.max_snapshots));

    let tick_period = (args.tps > 0).then(|| Duration::from_micros(1_000_000 / args.tps));
    let mut last_tick = Instant::now();

    println!(
        "Universe initialized with {} nodes and {} edges. Running...\n",
        args.nodes,
        universe.graph.edge_count()
    );

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

            // Clear terminal and render
            print!("\x1b[2J\x1b[H");
            println!("Reality Engine V0.1  [Tick {}]", tick);
            println!("──────────────────────────────────────────────────────");

            if args.viz {
                print!("{}", render_phi_grid(universe.states(), args.grid_width));
            }

            if args.histogram {
                print!("{}", render_histogram(universe.states()));
            }

            println!("{}", render_report(&report));
            print_emergence_signals(&report, args.nodes);
        }

        if let Some(period) = tick_period {
            let elapsed = last_tick.elapsed();
            if elapsed < period {
                std::thread::sleep(period - elapsed);
            }
            last_tick = Instant::now();
        }

        if args.ticks > 0 && tick >= args.ticks {
            println!("\nSimulation complete at tick {}.", tick);
            break;
        }
    }
}

fn print_emergence_signals(report: &observer::ObservationReport, node_count: usize) {
    if report.max_stability > 100 {
        println!(
            "  [EMERGENCE] Stable macro-structures: max persistence = {} ticks",
            report.max_stability
        );
    }
    if report.compression_ratio > 0.7 {
        println!(
            "  [EMERGENCE] High macro-compressibility: ratio = {:.3}",
            report.compression_ratio
        );
    }
    if report.largest_region > node_count / 4 {
        println!(
            "  [EMERGENCE] Large coherent domain: {} / {} nodes",
            report.largest_region, node_count
        );
    }
    if report.phi_variance < 0.05 && report.phi_entropy < 0.2 {
        println!("  [WARNING]   Low variance — universe may be collapsing to equilibrium");
    }
}
