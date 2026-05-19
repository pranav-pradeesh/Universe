use crate::kernel::node::NodeState;
use crate::topology::graph::SparseGraph;

/// Detects and tracks spontaneous symmetry breaking in the ψ field.
///
/// The ψ field starts near 0 (unstable vacuum) and breaks into regions of ψ≈+1 and ψ≈-1.
/// The boundaries between these regions are domain walls — the first particle-like structures.
pub struct SymmetryTracker {
    pub plus_fraction: f32,       // fraction of nodes in + vacuum
    pub minus_fraction: f32,      // fraction of nodes in - vacuum
    pub neutral_fraction: f32,    // fraction near ψ=0 (domain wall / transitional)
    pub domain_wall_edges: usize, // edges crossing ψ sign boundary (domain wall "length")
    pub symmetry_broken: bool,    // true once clear bimodal distribution forms
    pub breaking_tick: Option<u64>,
}

impl SymmetryTracker {
    pub fn new() -> Self {
        Self {
            plus_fraction: 0.0,
            minus_fraction: 0.0,
            neutral_fraction: 0.0,
            domain_wall_edges: 0,
            symmetry_broken: false,
            breaking_tick: None,
        }
    }

    pub fn update(&mut self, states: &[NodeState], graph: &SparseGraph, tick: u64) {
        let n = states.len() as f32;
        let threshold = 0.4f32; // consider "committed" to a vacuum above this

        let mut plus = 0usize;
        let mut minus = 0usize;
        let mut neutral = 0usize;

        for s in states {
            if s.psi > threshold {
                plus += 1;
            } else if s.psi < -threshold {
                minus += 1;
            } else {
                neutral += 1;
            }
        }

        self.plus_fraction = plus as f32 / n;
        self.minus_fraction = minus as f32 / n;
        self.neutral_fraction = neutral as f32 / n;

        // Count domain wall edges: edges where ψ changes sign
        let committed = self.plus_fraction + self.minus_fraction;
        self.domain_wall_edges = graph
            .iter_edges()
            .filter(|&(a, b)| {
                states[a as usize].psi * states[b as usize].psi < -0.1
            })
            .count();

        // Symmetry is broken when most nodes have committed to a vacuum
        let newly_broken = committed > 0.6 && !self.symmetry_broken;
        if newly_broken {
            self.symmetry_broken = true;
            self.breaking_tick = Some(tick);
        }
    }
}
