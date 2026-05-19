pub mod compression;
pub mod entropy;
pub mod persistence;
pub mod symmetry;

use crate::kernel::node::NodeState;
use crate::topology::graph::SparseGraph;

pub use compression::CompressionDetector;
pub use entropy::EntropyAnalyzer;
pub use persistence::PersistenceTracker;
pub use symmetry::SymmetryTracker;

pub struct Observer {
    pub persistence: PersistenceTracker,
    pub entropy: EntropyAnalyzer,
    pub compression: CompressionDetector,
    pub symmetry: SymmetryTracker,
}

#[derive(Debug, Clone)]
pub struct ObservationReport {
    pub tick: u64,
    // Topology
    pub edge_count: usize,
    pub mutations_created: u32,
    pub mutations_destroyed: u32,
    // Field statistics
    pub psi_entropy: f32,
    pub psi_variance: f32,
    pub active_fraction: f32,
    // Energy
    pub total_energy: f32,
    pub energy_mean: f32,
    pub energy_variance: f32,
    pub kinetic_fraction: f32,
    // Emergence
    pub max_stability: u32,
    pub avg_stability: f32,
    pub compression_ratio: f32,
    pub macro_regions: usize,
    pub largest_region: usize,
    // Symmetry breaking
    pub symmetry_broken: bool,
    pub plus_fraction: f32,
    pub minus_fraction: f32,
    pub domain_wall_edges: usize,
    pub breaking_tick: Option<u64>,
}

impl ObservationReport {
    pub fn neutral_fraction(&self) -> f32 {
        (1.0 - self.plus_fraction - self.minus_fraction).max(0.0)
    }
}

impl Observer {
    pub fn new(node_count: usize) -> Self {
        Self {
            persistence: PersistenceTracker::new(node_count),
            entropy: EntropyAnalyzer::new(),
            compression: CompressionDetector::new(),
            symmetry: SymmetryTracker::new(),
        }
    }

    pub fn observe(
        &mut self,
        tick: u64,
        states: &[NodeState],
        graph: &SparseGraph,
        mutations_created: u32,
        mutations_destroyed: u32,
    ) -> ObservationReport {
        self.persistence.update(states);
        self.entropy.update(states);
        self.compression.update(graph, states);
        self.symmetry.update(states, graph, tick);

        ObservationReport {
            tick,
            edge_count: graph.edge_count(),
            mutations_created,
            mutations_destroyed,
            psi_entropy: self.entropy.psi_entropy,
            psi_variance: self.entropy.psi_variance,
            active_fraction: self.entropy.active_fraction,
            total_energy: self.entropy.total_energy,
            energy_mean: self.entropy.energy_mean,
            energy_variance: self.entropy.energy_variance,
            kinetic_fraction: self.entropy.kinetic_fraction,
            max_stability: self.persistence.max_stability,
            avg_stability: self.persistence.avg_stability,
            compression_ratio: self.compression.compression_ratio,
            macro_regions: self.compression.macro_region_count,
            largest_region: self.compression.largest_region_size,
            symmetry_broken: self.symmetry.symmetry_broken,
            plus_fraction: self.symmetry.plus_fraction,
            minus_fraction: self.symmetry.minus_fraction,
            domain_wall_edges: self.symmetry.domain_wall_edges,
            breaking_tick: self.symmetry.breaking_tick,
        }
    }
}
