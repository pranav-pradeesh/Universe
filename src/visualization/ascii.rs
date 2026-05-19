use crate::kernel::node::NodeState;
use crate::observer::ObservationReport;

/// Maps ψ value to ASCII character.
/// Positive ψ (+ vacuum): block characters (dark)
/// Negative ψ (- vacuum): dash characters (light)
/// Near zero (domain wall): dot/pipe
fn psi_to_char(psi: f32) -> char {
    if psi > 0.7 { '█' }
    else if psi > 0.4 { '▓' }
    else if psi > 0.1 { '▒' }
    else if psi > -0.1 { '·' }
    else if psi > -0.4 { '░' }
    else if psi > -0.7 { '─' }
    else { ' ' }
}

fn chi_to_char(chi: f32) -> char {
    // chi: [0.2, 2.5] → chars for geometry/curvature heatmap
    if chi > 2.0 { '◉' }
    else if chi > 1.6 { '●' }
    else if chi > 1.3 { '○' }
    else if chi > 1.1 { '·' }
    else { ' ' }
}

pub fn render_psi_grid(states: &[NodeState], width: usize) -> String {
    let n = states.len();
    let height = (n + width - 1) / width;
    let mut out = String::with_capacity((width + 3) * (height + 2) + 256);

    out.push('┌');
    for _ in 0..width { out.push('─'); }
    out.push_str("┐  ψ field  [█=+vac  ·=wall  ' '=-vac]\n");

    for row in 0..height {
        out.push('│');
        for col in 0..width {
            let idx = row * width + col;
            out.push(if idx < n { psi_to_char(states[idx].psi) } else { ' ' });
        }
        out.push('│');
        out.push('\n');
    }

    out.push('└');
    for _ in 0..width { out.push('─'); }
    out.push('┘');
    out.push('\n');
    out
}

pub fn render_chi_grid(states: &[NodeState], width: usize) -> String {
    let n = states.len();
    let height = (n + width - 1) / width;
    let mut out = String::with_capacity((width + 3) * (height + 2) + 128);

    out.push('┌');
    for _ in 0..width { out.push('─'); }
    out.push_str("┐  χ metric (proto-curvature)\n");

    for row in 0..height {
        out.push('│');
        for col in 0..width {
            let idx = row * width + col;
            out.push(if idx < n { chi_to_char(states[idx].chi) } else { ' ' });
        }
        out.push('│');
        out.push('\n');
    }

    out.push('└');
    for _ in 0..width { out.push('─'); }
    out.push('┘');
    out.push('\n');
    out
}

pub fn render_report(r: &ObservationReport) -> String {
    let sym_status = if r.symmetry_broken {
        format!("BROKEN (tick {})", r.breaking_tick.unwrap_or(0))
    } else {
        "intact".to_string()
    };

    format!(
        "Tick {:>8} │ Edges {:>6} │ Mutations +{} −{}\n\
         ψ  var={:.4}  entropy={:.3}  active={:.1}%\n\
         E  total={:>9.3}  mean={:.4}  var={:.4}  KE%={:.1}%\n\
         Persist  max={:>6}  avg={:.2}\n\
         Compress  ratio={:.3}  regions={}  largest={}\n\
         Symmetry  {}  +={:.1}%  −={:.1}%  walls={}\n",
        r.tick,
        r.edge_count,
        r.mutations_created, r.mutations_destroyed,
        r.psi_variance, r.psi_entropy, r.active_fraction * 100.0,
        r.total_energy, r.energy_mean, r.energy_variance, r.kinetic_fraction * 100.0,
        r.max_stability, r.avg_stability,
        r.compression_ratio, r.macro_regions, r.largest_region,
        sym_status, r.plus_fraction * 100.0, r.minus_fraction * 100.0, r.domain_wall_edges,
    )
}

pub fn render_histogram(states: &[NodeState]) -> String {
    let bins = 20usize;
    let mut counts = vec![0u32; bins];
    for s in states {
        let bin = ((s.psi + 1.0) / 2.0 * bins as f32) as usize;
        counts[bin.min(bins - 1)] += 1;
    }
    let max_count = counts.iter().copied().max().unwrap_or(1).max(1);
    let bar_height = 6usize;

    let mut out = String::from("ψ distribution [-1 ··· +1]\n");
    for row in (0..bar_height).rev() {
        out.push('│');
        for &c in &counts {
            let bar = (c as f32 / max_count as f32 * bar_height as f32) as usize;
            out.push(if bar > row { '█' } else { ' ' });
        }
        out.push_str("│\n");
    }
    out.push('└');
    for _ in 0..bins { out.push('─'); }
    out.push_str("┘  (bimodal → symmetry broken)\n");
    out
}
