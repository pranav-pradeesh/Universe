use crate::kernel::node::{NodeId, NodeState};
use crate::topology::graph::SparseGraph;

const OMEGA_CREATE_THRESHOLD: f32 = 0.65;
const OMEGA_DESTROY_THRESHOLD: f32 = 0.70;
const MAX_DEGREE: usize = 8;
const MIN_DEGREE_FOR_DESTROY: usize = 2;
const COOLDOWN_TICKS: u64 = 10;

pub struct MutationEngine {
    cooldown: Vec<u64>,
}

impl MutationEngine {
    pub fn new(node_count: usize) -> Self {
        Self { cooldown: vec![0u64; node_count] }
    }

    pub fn apply(
        &mut self,
        graph: &mut SparseGraph,
        states: &[NodeState],
        tick: u64,
    ) -> MutationStats {
        let mut created = 0u32;
        let mut destroyed = 0u32;

        // Edge destruction: high omega on both endpoints + diverging psi
        let edges: Vec<(NodeId, NodeId)> = graph.iter_edges().collect();
        for (a, b) in edges {
            let sa = &states[a as usize];
            let sb = &states[b as usize];
            let psi_divergence = (sa.psi - sb.psi).abs();
            let avg_omega = (sa.omega.abs() + sb.omega.abs()) * 0.5;
            let on_cooldown = tick - self.cooldown[a as usize] < COOLDOWN_TICKS
                || tick - self.cooldown[b as usize] < COOLDOWN_TICKS;

            if !on_cooldown
                && avg_omega > OMEGA_DESTROY_THRESHOLD
                && psi_divergence > 0.8
                && graph.degree(a) > MIN_DEGREE_FOR_DESTROY
                && graph.degree(b) > MIN_DEGREE_FOR_DESTROY
            {
                graph.remove_edge(a, b);
                self.cooldown[a as usize] = tick;
                self.cooldown[b as usize] = tick;
                destroyed += 1;
            }
        }

        // Edge creation: triadic closure among high-omega compatible nodes
        let node_count = graph.node_count;
        'node_loop: for a in 0..node_count as NodeId {
            let sa = &states[a as usize];
            if sa.omega.abs() < OMEGA_CREATE_THRESHOLD { continue; }
            if graph.degree(a) >= MAX_DEGREE { continue; }
            if tick - self.cooldown[a as usize] < COOLDOWN_TICKS * 2 { continue; }

            let current_neighbors: Vec<NodeId> = graph.neighbors(a).to_vec();
            for &nb in &current_neighbors {
                let nb_neighbors = graph.neighbors(nb).to_vec();
                for &candidate in &nb_neighbors {
                    if candidate == a { continue; }
                    if graph.has_edge(a, candidate) { continue; }
                    if graph.degree(candidate) >= MAX_DEGREE { continue; }
                    if tick - self.cooldown[candidate as usize] < COOLDOWN_TICKS * 2 { continue; }

                    let sc = &states[candidate as usize];
                    let psi_affinity = 1.0 - (sa.psi - sc.psi).abs();
                    let omega_product = sa.omega.abs() * sc.omega.abs();

                    if psi_affinity > 0.5 && omega_product > 0.4 {
                        graph.add_edge(a, candidate);
                        self.cooldown[a as usize] = tick;
                        self.cooldown[candidate as usize] = tick;
                        created += 1;
                        continue 'node_loop;
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
