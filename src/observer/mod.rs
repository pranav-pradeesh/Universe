pub mod persistence;
pub mod entropy;
pub mod compression;

use crate::kernel::node::NodeState;
use crate::topology::graph::SparseGraph;

pub use persistence::PersistenceTracker;
pub use entropy::EntropyAnalyzer;
pub use compression::CompressionDetector;

pub struct Observer {
    pub persistence: PersistenceTracker,
    pub entropy: EntropyAnalyzer,
    pub compression: CompressionDetector,
}

#[derive(Debug, Clone)]
pub struct ObservationReport {
    pub tick: u64,
    pub max_stability: u32,
    pub avg_stability: f32,
    pub phi_entropy: f32,
    pub phi_variance: f32,
    pub active_fraction: f32,
    pub compression_ratio: f32,
    pub macro_regions: usize,
    pub largest_region: usize,
    pub edge_count: usize,
    pub mutations_created: u32,
    pub mutations_destroyed: u32,
}

impl Observer {
    pub fn new(node_count: usize) -> Self {
        Self {
            persistence: PersistenceTracker::new(node_count),
            entropy: EntropyAnalyzer::new(),
            compression: CompressionDetector::new(),
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

        ObservationReport {
            tick,
            max_stability: self.persistence.max_stability,
            avg_stability: self.persistence.avg_stability,
            phi_entropy: self.entropy.global_phi_entropy,
            phi_variance: self.entropy.phi_variance,
            active_fraction: self.entropy.active_fraction,
            compression_ratio: self.compression.compression_ratio,
            macro_regions: self.compression.macro_region_count,
            largest_region: self.compression.largest_region_size,
            edge_count: graph.edge_count(),
            mutations_created,
            mutations_destroyed,
        }
    }
}
