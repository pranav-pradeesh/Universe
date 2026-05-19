use crate::kernel::node::{NodeId, NodeState};

/// Tracks how long each node's phi-sign has remained stable.
pub struct PersistenceTracker {
    prev_phi_sign: Vec<i8>, // -1, 0, +1
    stability: Vec<u32>,    // ticks since last sign flip
    pub max_stability: u32,
    pub avg_stability: f32,
}

impl PersistenceTracker {
    pub fn new(node_count: usize) -> Self {
        Self {
            prev_phi_sign: vec![0i8; node_count],
            stability: vec![0u32; node_count],
            max_stability: 0,
            avg_stability: 0.0,
        }
    }

    pub fn update(&mut self, states: &[NodeState]) {
        let n = states.len();
        let mut total = 0u64;
        let mut max = 0u32;

        for i in 0..n {
            let sign = if states[i].phi > 0.05 { 1i8 }
                       else if states[i].phi < -0.05 { -1i8 }
                       else { 0i8 };

            if sign == self.prev_phi_sign[i] && sign != 0 {
                self.stability[i] = self.stability[i].saturating_add(1);
            } else {
                self.stability[i] = 0;
            }
            self.prev_phi_sign[i] = sign;
            total += self.stability[i] as u64;
            if self.stability[i] > max { max = self.stability[i]; }
        }

        self.max_stability = max;
        self.avg_stability = total as f32 / n as f32;
    }

    pub fn stability_at(&self, node: NodeId) -> u32 {
        self.stability[node as usize]
    }
}
