use crate::kernel::node::{NodeId, NodeState};
use crate::topology::graph::SparseGraph;

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

    pub fn update(&mut self, graph: &SparseGraph, states: &[NodeState]) {
        let n = graph.node_count;
        let threshold = 0.15f32;
        let mut visited = vec![false; n];
        let mut regions = Vec::new();

        for start in 0..n as NodeId {
            if visited[start as usize] {
                continue;
            }
            visited[start as usize] = true;
            let psi0 = states[start as usize].psi;
            let mut region = vec![start];
            let mut queue = vec![start];

            while let Some(node) = queue.pop() {
                for &nb in graph.neighbors(node) {
                    if !visited[nb as usize] {
                        let psi_nb = states[nb as usize].psi;
                        if (psi_nb - psi0).abs() < threshold {
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
        self.compression_ratio = if n > 0 {
            1.0 - (self.macro_region_count as f32 / n as f32)
        } else {
            0.0
        };
    }
}
