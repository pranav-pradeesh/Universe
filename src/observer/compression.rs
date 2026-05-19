use crate::kernel::node::{NodeId, NodeState};
use crate::topology::graph::SparseGraph;

/// Measures how much the universe has self-organized into compressible macro-states.
pub struct CompressionDetector {
    pub macro_region_count: usize,
    pub compression_ratio: f32,
    pub largest_region_size: usize,
}

impl CompressionDetector {
    pub fn new() -> Self {
        Self {
            macro_region_count: 0,
            compression_ratio: 0.0,
            largest_region_size: 0,
        }
    }

    /// A macro-region is a connected subgraph where all nodes have similar phi (within threshold).
    pub fn update(&mut self, graph: &SparseGraph, states: &[NodeState]) {
        let n = graph.node_count;
        let threshold = 0.15f32;
        let mut visited = vec![false; n];
        let mut regions = Vec::new();

        for start in 0..n as NodeId {
            if visited[start as usize] { continue; }
            visited[start as usize] = true;

            let phi0 = states[start as usize].phi;
            let mut region = vec![start];
            let mut queue = vec![start];

            while let Some(node) = queue.pop() {
                for &nb in graph.neighbors(node) {
                    if !visited[nb as usize] {
                        let phi_nb = states[nb as usize].phi;
                        if (phi_nb - phi0).abs() < threshold {
                            visited[nb as usize] = true;
                            region.push(nb);
                            queue.push(nb);
                        }
                    }
                }
            }

            if region.len() >= 3 {
                regions.push(region);
            }
        }

        self.macro_region_count = regions.len();
        self.largest_region_size = regions.iter().map(|r| r.len()).max().unwrap_or(0);
        // Compression ratio: how many macro-regions vs nodes
        // A high ratio means good compression (few macro-states describe many nodes)
        self.compression_ratio = if n > 0 {
            1.0 - (self.macro_region_count as f32 / n as f32)
        } else {
            0.0
        };
    }
}

impl Default for CompressionDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::graph::SparseGraph;

    #[test]
    fn test_compression_all_similar() {
        // All nodes have same phi => one big region
        let n = 8;
        let mut graph = SparseGraph::new(n);
        // Ring topology
        for i in 0..n as NodeId {
            graph.add_edge(i, (i + 1) % n as NodeId);
        }
        let states = vec![NodeState { phi: 0.5, tau: 0.0, omega: 0.0 }; n];
        let mut detector = CompressionDetector::new();
        detector.update(&graph, &states);
        // Should find one large region
        assert_eq!(detector.largest_region_size, n);
        assert!(detector.compression_ratio > 0.0);
    }

    #[test]
    fn test_compression_all_different() {
        // All nodes have very different phi => many small/no regions
        let n = 8;
        let mut graph = SparseGraph::new(n);
        for i in 0..n as NodeId {
            graph.add_edge(i, (i + 1) % n as NodeId);
        }
        // Alternating +1 and -1 phi values
        let states: Vec<NodeState> = (0..n).map(|i| NodeState {
            phi: if i % 2 == 0 { 0.9 } else { -0.9 },
            tau: 0.0,
            omega: 0.0,
        }).collect();
        let mut detector = CompressionDetector::new();
        detector.update(&graph, &states);
        // No large regions (alternating means neighbors always differ by ~1.8 > threshold 0.15)
        assert!(detector.largest_region_size < n / 2);
    }

    #[test]
    fn test_compression_ratio_bounds() {
        let n = 16;
        let mut graph = SparseGraph::new(n);
        for i in 0..n as NodeId {
            graph.add_edge(i, (i + 1) % n as NodeId);
        }
        let states = vec![NodeState { phi: 0.3, tau: 0.0, omega: 0.0 }; n];
        let mut detector = CompressionDetector::new();
        detector.update(&graph, &states);
        assert!(detector.compression_ratio >= 0.0 && detector.compression_ratio <= 1.0);
    }
}
