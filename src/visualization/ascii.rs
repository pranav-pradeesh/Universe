use crate::kernel::node::NodeState;
use crate::observer::ObservationReport;

const PHI_CHARS: &[char] = &[' ', '·', '░', '▒', '▓', '█'];
const NEG_CHARS: &[char] = &[' ', '·', '╌', '─', '═', '■'];

pub fn render_phi_grid(states: &[NodeState], width: usize) -> String {
    let n = states.len();
    let height = (n + width - 1) / width;
    let mut out = String::with_capacity((width + 1) * height + 256);

    // Border
    out.push('┌');
    for _ in 0..width { out.push('─'); }
    out.push('┐');
    out.push('\n');

    for row in 0..height {
        out.push('│');
        for col in 0..width {
            let idx = row * width + col;
            if idx < n {
                let phi = states[idx].phi;
                let c = phi_to_char(phi);
                out.push(c);
            } else {
                out.push(' ');
            }
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

fn phi_to_char(phi: f32) -> char {
    if phi >= 0.0 {
        let idx = (phi * (PHI_CHARS.len() - 1) as f32) as usize;
        PHI_CHARS[idx.min(PHI_CHARS.len() - 1)]
    } else {
        let idx = ((-phi) * (NEG_CHARS.len() - 1) as f32) as usize;
        NEG_CHARS[idx.min(NEG_CHARS.len() - 1)]
    }
}

pub fn render_report(report: &ObservationReport) -> String {
    format!(
        "Tick {:>8} │ Edges {:>6} │ Φ-var {:.4} │ Φ-entropy {:.3} │ Active {:.1}%\n\
         Persist  max={:>6}  avg={:.2} │ Compress {:.3} ({} regions, largest={})\n\
         Mutations  +{} −{}\n",
        report.tick,
        report.edge_count,
        report.phi_variance,
        report.phi_entropy,
        report.active_fraction * 100.0,
        report.max_stability,
        report.avg_stability,
        report.compression_ratio,
        report.macro_regions,
        report.largest_region,
        report.mutations_created,
        report.mutations_destroyed,
    )
}

pub fn render_histogram(states: &[NodeState]) -> String {
    let bins = 20usize;
    let mut counts = vec![0u32; bins];
    for s in states {
        let bin = ((s.phi + 1.0) / 2.0 * bins as f32) as usize;
        let bin = bin.min(bins - 1);
        counts[bin] += 1;
    }
    let max_count = *counts.iter().max().unwrap_or(&1).max(&1);
    let bar_height = 8usize;

    let mut out = String::from("phi distribution [-1 .. +1]\n");
    for row in (0..bar_height).rev() {
        out.push('│');
        for &c in &counts {
            let bar = (c as f32 / max_count as f32 * bar_height as f32) as usize;
            if bar > row { out.push('█'); } else { out.push(' '); }
        }
        out.push_str("│\n");
    }
    out.push('└');
    for _ in 0..bins { out.push('─'); }
    out.push('┘');
    out.push('\n');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phi_to_char_positive() {
        // phi=0.0 should map to first char (space)
        assert_eq!(phi_to_char(0.0), ' ');
        // phi=1.0 should map to last char
        assert_eq!(phi_to_char(1.0), '█');
    }

    #[test]
    fn test_phi_to_char_negative() {
        // phi=-1.0 should map to last negative char
        assert_eq!(phi_to_char(-1.0), '■');
        // phi=-0.0 same as 0.0 (non-negative)
        assert_eq!(phi_to_char(-0.0), ' ');
    }

    #[test]
    fn test_render_phi_grid_dimensions() {
        let states = vec![NodeState { phi: 0.5, tau: 0.0, omega: 0.0 }; 16];
        let grid = render_phi_grid(&states, 4);
        // Should have border rows + 4 data rows
        let lines: Vec<&str> = grid.lines().collect();
        assert_eq!(lines.len(), 6, "4 data rows + top border + bottom border");
    }

    #[test]
    fn test_render_report_contains_tick() {
        let report = ObservationReport {
            tick: 42,
            max_stability: 10,
            avg_stability: 5.0,
            phi_entropy: 0.5,
            phi_variance: 0.1,
            active_fraction: 0.8,
            compression_ratio: 0.3,
            macro_regions: 5,
            largest_region: 20,
            edge_count: 100,
            mutations_created: 3,
            mutations_destroyed: 1,
        };
        let output = render_report(&report);
        assert!(output.contains("42"), "report should contain the tick number");
        assert!(output.contains("100"), "report should contain edge count");
    }

    #[test]
    fn test_render_histogram_has_bars() {
        let states: Vec<NodeState> = (0..20).map(|i| NodeState {
            phi: (i as f32 / 20.0) * 2.0 - 1.0,
            tau: 0.0,
            omega: 0.0,
        }).collect();
        let hist = render_histogram(&states);
        assert!(hist.contains('█'), "histogram should have bar characters");
        assert!(hist.contains("phi distribution"), "histogram should have label");
    }
}
