use crate::kernel::node::{NodeId, NodeState};
use crate::topology::graph::SparseGraph;
use crate::topology::mutation::{MutationEngine, MutationStats};
use crate::transform::local::transform;

pub struct Universe {
    pub tick: u64,
    pub seed: u64,
    pub node_count: usize,
    states: Vec<NodeState>,
    next_states: Vec<NodeState>,
    pub graph: SparseGraph,
    mutation_engine: MutationEngine,
    pub last_mutation: MutationStats,
}

// Simple deterministic xorshift64 for initialization only
struct Xorshift64(u64);
impl Xorshift64 {
    fn new(seed: u64) -> Self { Self(if seed == 0 { 0xdeadbeef_cafebabe } else { seed }) }
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13; x ^= x >> 7; x ^= x << 17;
        self.0 = x; x
    }
    fn f32_signed(&mut self) -> f32 {
        let raw = self.next();
        ((raw as f32) / (u64::MAX as f32)) * 2.0 - 1.0
    }
    fn f32_unit(&mut self) -> f32 {
        (self.next() as f32) / (u64::MAX as f32)
    }
}

impl Universe {
    pub fn new(seed: u64, node_count: usize) -> Self {
        assert!(node_count >= 4, "need at least 4 nodes");
        let graph = SparseGraph::new(node_count);
        let states = vec![NodeState::default(); node_count];
        let next_states = vec![NodeState::default(); node_count];
        let mutation_engine = MutationEngine::new(node_count);
        Universe {
            tick: 0,
            seed,
            node_count,
            states,
            next_states,
            graph,
            mutation_engine,
            last_mutation: MutationStats::default(),
        }
    }

    /// Initialize with structured gradients and a ring topology.
    /// Initial topology: ring with K=4 neighbors per node.
    pub fn init(&mut self) {
        let mut rng = Xorshift64::new(self.seed);
        let n = self.node_count;
        let ring_k = 4usize; // each node connected to K nearest in ring

        // Build ring topology
        for i in 0..n {
            for k in 1..=ring_k {
                let j = (i + k) % n;
                self.graph.add_edge(i as NodeId, j as NodeId);
            }
        }

        // Initialize states with structured gradients
        // Use several overlapping sinusoidal gradients seeded by rng
        // This creates "ordered tension" and asymmetric initial conditions
        let freq1 = rng.f32_unit() * 3.0 + 1.0;
        let freq2 = rng.f32_unit() * 5.0 + 2.0;
        let phase1 = rng.f32_unit() * std::f32::consts::TAU;
        let phase2 = rng.f32_unit() * std::f32::consts::TAU;
        let amp_phi = 0.6 + rng.f32_unit() * 0.3;
        let amp_tau = 0.2 + rng.f32_unit() * 0.2;

        for i in 0..n {
            let t = (i as f32) / (n as f32) * std::f32::consts::TAU;
            let phi_init = amp_phi * (freq1 * t + phase1).sin()
                         + 0.3 * (freq2 * t + phase2).cos();
            let tau_init = amp_tau * (freq1 * t * 0.7 + phase2).cos();
            // Small omega perturbation
            let omega_init = rng.f32_signed() * 0.05;

            self.states[i] = NodeState::new(phi_init, tau_init, omega_init);
        }
    }

    pub fn states(&self) -> &[NodeState] {
        &self.states
    }

    /// Execute one full deterministic tick.
    pub fn tick(&mut self) {
        self.tick += 1;

        // Phase 1+2+3: Read neighborhoods, compute updates, stage results
        let n = self.node_count;
        // Collect neighbor states for each node (borrowck: read from states, write to next_states)
        // We build neighbor snapshots before transform to avoid borrow issues
        for i in 0..n {
            let neighbors: Vec<NodeState> = self.graph
                .neighbors(i as NodeId)
                .iter()
                .map(|&nb| self.states[nb as usize])
                .collect();
            self.next_states[i] = transform(&self.states[i], &neighbors);
        }

        // Phase 4: Commit scalar updates
        self.states.copy_from_slice(&self.next_states);

        // Phase 5: Evaluate and apply topology mutations
        self.last_mutation = self.mutation_engine.apply(&mut self.graph, &self.states, self.tick);

        // (Phase 6: observation layer runs externally, driven by caller)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universe_init_valid_states() {
        let mut u = Universe::new(42, 16);
        u.init();
        for s in u.states() {
            assert!(s.is_valid(), "all states must be valid after init");
        }
    }

    #[test]
    fn test_universe_tick_advances() {
        let mut u = Universe::new(42, 16);
        u.init();
        assert_eq!(u.tick, 0);
        u.tick();
        assert_eq!(u.tick, 1);
        u.tick();
        assert_eq!(u.tick, 2);
    }

    #[test]
    fn test_universe_tick_deterministic() {
        let mut u1 = Universe::new(123, 32);
        let mut u2 = Universe::new(123, 32);
        u1.init();
        u2.init();
        for _ in 0..10 {
            u1.tick();
            u2.tick();
        }
        let s1 = u1.states();
        let s2 = u2.states();
        for (a, b) in s1.iter().zip(s2.iter()) {
            assert_eq!(a, b, "same seed must produce same state");
        }
    }

    #[test]
    fn test_universe_different_seeds() {
        let mut u1 = Universe::new(1, 32);
        let mut u2 = Universe::new(2, 32);
        u1.init();
        u2.init();
        for _ in 0..5 {
            u1.tick();
            u2.tick();
        }
        // Different seeds should produce at least some different states
        let differs = u1.states().iter().zip(u2.states().iter()).any(|(a, b)| a != b);
        assert!(differs, "different seeds should produce different outcomes");
    }

    #[test]
    fn test_universe_states_remain_valid_after_ticks() {
        let mut u = Universe::new(99, 64);
        u.init();
        for _ in 0..50 {
            u.tick();
        }
        for s in u.states() {
            assert!(s.is_valid(), "states must remain valid after 50 ticks");
        }
    }
}
