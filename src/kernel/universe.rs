use crate::kernel::constants::*;
use crate::kernel::node::{NodeId, NodeState};
use crate::topology::graph::SparseGraph;
use crate::topology::mutation::{MutationEngine, MutationStats};
use crate::transform::local::transform;

pub struct Universe {
    pub tick: u64,
    pub seed: u64,
    pub node_count: usize,
    pub grid_width: usize,    // sqrt(node_count) for 2D layout
    states: Vec<NodeState>,
    next_states: Vec<NodeState>,
    pub graph: SparseGraph,
    mutation_engine: MutationEngine,
    pub last_mutation: MutationStats,
}

struct Xorshift64(u64);
impl Xorshift64 {
    fn new(seed: u64) -> Self {
        Self(if seed == 0 { 0xdeadbeef_cafebabe } else { seed })
    }
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
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
        let width = (node_count as f64).sqrt() as usize;
        let node_count = width * width; // ensure perfect square
        assert!(node_count >= 4, "need at least 4 nodes");

        Universe {
            tick: 0,
            seed,
            node_count,
            grid_width: width,
            states: vec![NodeState::default(); node_count],
            next_states: vec![NodeState::default(); node_count],
            graph: SparseGraph::new(node_count),
            mutation_engine: MutationEngine::new(node_count),
            last_mutation: MutationStats::default(),
        }
    }

    /// Initialize universe on a 2D torus grid with small noise near ψ=0 (unstable equilibrium).
    /// The double-well potential will drive spontaneous symmetry breaking.
    pub fn init(&mut self) {
        let mut rng = Xorshift64::new(self.seed);
        let w = self.grid_width;
        let n = self.node_count;

        // Build 2D torus topology: each node connects right and down (periodic)
        for row in 0..w {
            for col in 0..w {
                let node = (row * w + col) as NodeId;
                let right = (row * w + (col + 1) % w) as NodeId;
                let down = (((row + 1) % w) * w + col) as NodeId;
                self.graph.add_edge(node, right);
                self.graph.add_edge(node, down);
            }
        }

        // Seed with small perturbations near ψ=0.
        // A few "seeds" of opposite sign create tension — forcing the universe to break symmetry.
        for i in 0..n {
            let psi_init = rng.f32_signed() * INIT_NOISE;
            let phi_init = rng.f32_signed() * INIT_NOISE * 0.5;
            // Slightly perturbed flat metric
            let chi_init = 1.0 + rng.f32_signed() * 0.01;

            self.states[i] = NodeState {
                psi: psi_init,
                pi: 0.0,
                phi: phi_init,
                rho: 0.0,
                chi: chi_init.max(METRIC_MIN).min(METRIC_MAX),
                omega: 0.0,
            };
        }

        // Add a few strong seeds to guarantee interesting symmetry breaking
        let seed_count = (n / 64).max(4);
        for k in 0..seed_count {
            let idx = (rng.f32_unit() * n as f32) as usize % n;
            let sign = if k % 2 == 0 { 1.0f32 } else { -1.0f32 };
            self.states[idx].psi = sign * 0.3;
        }
    }

    pub fn states(&self) -> &[NodeState] {
        &self.states
    }

    /// One full deterministic tick.
    ///
    /// Phases:
    ///   1. Read neighborhoods
    ///   2. Compute Hamiltonian update (symplectic leapfrog)
    ///   3. Commit scalar updates
    ///   4. Apply topology mutations
    pub fn tick(&mut self) {
        self.tick += 1;

        let n = self.node_count;

        // Phases 1–3: Hamiltonian update
        for i in 0..n {
            let neighbors: Vec<NodeState> = self.graph
                .neighbors(i as NodeId)
                .iter()
                .map(|&nb| self.states[nb as usize])
                .collect();
            self.next_states[i] = transform(&self.states[i], &neighbors);
        }
        self.states.copy_from_slice(&self.next_states);

        // Phase 4: Topology mutations (omega-driven)
        self.last_mutation =
            self.mutation_engine
                .apply(&mut self.graph, &self.states, self.tick);
    }

    pub fn total_energy(&self) -> f32 {
        self.states.iter().map(|s| s.local_energy()).sum()
    }
}
