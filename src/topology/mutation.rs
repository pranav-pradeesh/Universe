use crate::kernel::node::{NodeId, NodeState};
use crate::topology::graph::SparseGraph;

const OMEGA_CREATE_THRESHOLD: f32 = 0.65;
const OMEGA_DESTROY_THRESHOLD: f32 = 0.70;
const MAX_DEGREE: usize = 8;
const MIN_DEGREE_FOR_DESTROY: usize = 2;
const COOLDOWN_TICKS: u64 = 8;

pub struct MutationEngine {
    cooldown: Vec<u64>, // last mutation tick per node
}

impl MutationEngine {
    pub fn new(node_count: usize) -> Self {
        Self {
            cooldown: vec![0u64; node_count],
        }
    }

    pub fn apply(
        &mut self,
        graph: &mut SparseGraph,
        states: &[NodeState],
        tick: u64,
    ) -> MutationStats {
        let mut created = 0u32;
        let mut destroyed = 0u32;

        // Edge destruction: scan existing edges
        let edges: Vec<(NodeId, NodeId)> = graph.iter_edges().collect();
        for (a, b) in edges {
            let sa = &states[a as usize];
            let sb = &states[b as usize];

            // High omega on both endpoints with diverging phi: destroy
            let phi_divergence = (sa.phi - sb.phi).abs();
            let avg_omega = (sa.omega.abs() + sb.omega.abs()) * 0.5;

            let on_cooldown = tick.saturating_sub(self.cooldown[a as usize]) < COOLDOWN_TICKS
                || tick.saturating_sub(self.cooldown[b as usize]) < COOLDOWN_TICKS;

            if !on_cooldown
                && avg_omega > OMEGA_DESTROY_THRESHOLD
                && phi_divergence > 0.6
                && graph.degree(a) > MIN_DEGREE_FOR_DESTROY
                && graph.degree(b) > MIN_DEGREE_FOR_DESTROY
            {
                graph.remove_edge(a, b);
                self.cooldown[a as usize] = tick;
                self.cooldown[b as usize] = tick;
                destroyed += 1;
            }
        }

        // Edge creation: scan nodes with high omega, find viable partners
        let node_count = graph.node_count;
        for a in 0..node_count as NodeId {
            let sa = &states[a as usize];
            if sa.omega.abs() < OMEGA_CREATE_THRESHOLD { continue; }
            if graph.degree(a) >= MAX_DEGREE { continue; }
            if tick.saturating_sub(self.cooldown[a as usize]) < COOLDOWN_TICKS * 2 { continue; }

            // Look for a neighbor-of-neighbor to connect to (triadic closure tendency)
            let current_neighbors: Vec<NodeId> = graph.neighbors(a).to_vec();
            'outer: for &nb in &current_neighbors {
                let nb_neighbors = graph.neighbors(nb).to_vec();
                for &candidate in &nb_neighbors {
                    if candidate == a { continue; }
                    if graph.has_edge(a, candidate) { continue; }
                    if graph.degree(candidate) >= MAX_DEGREE { continue; }
                    if tick.saturating_sub(self.cooldown[candidate as usize]) < COOLDOWN_TICKS * 2 { continue; }

                    let sc = &states[candidate as usize];
                    // Similar phi and both high omega: attractive
                    let phi_affinity = 1.0 - (sa.phi - sc.phi).abs();
                    let omega_product = sa.omega.abs() * sc.omega.abs();

                    if phi_affinity > 0.5 && omega_product > 0.4 {
                        graph.add_edge(a, candidate);
                        self.cooldown[a as usize] = tick;
                        self.cooldown[candidate as usize] = tick;
                        created += 1;
                        break 'outer;
                    }
                }
            }
        }

        MutationStats { created, destroyed }
    }
}

#[derive(Default, Debug, Clone)]
pub struct MutationStats {
    pub created: u32,
    pub destroyed: u32,
}
