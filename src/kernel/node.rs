pub type NodeId = u32;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NodeState {
    pub phi: f32,   // Potential: attraction/repulsion/clustering [-1, 1]
    pub tau: f32,   // Inertia: persistence, delayed adaptation [-1, 1]
    pub omega: f32, // Flux: topology mutation pressure [-1, 1]
}

impl NodeState {
    pub fn new(phi: f32, tau: f32, omega: f32) -> Self {
        Self {
            phi: phi.tanh(),
            tau: tau.tanh(),
            omega: omega.tanh(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.phi.is_finite()
            && self.tau.is_finite()
            && self.omega.is_finite()
            && self.phi.abs() <= 1.0 + f32::EPSILON
            && self.tau.abs() <= 1.0 + f32::EPSILON
            && self.omega.abs() <= 1.0 + f32::EPSILON
    }

    pub fn clamp(&self) -> Self {
        Self {
            phi: self.phi.clamp(-1.0, 1.0),
            tau: self.tau.clamp(-1.0, 1.0),
            omega: self.omega.clamp(-1.0, 1.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_state_new_clamps_via_tanh() {
        // tanh saturates at ±1, so large inputs should stay in range
        let s = NodeState::new(100.0, -100.0, 50.0);
        assert!(s.phi > 0.99 && s.phi <= 1.0);
        assert!(s.tau < -0.99 && s.tau >= -1.0);
        assert!(s.is_valid());
    }

    #[test]
    fn test_node_state_is_valid() {
        let s = NodeState { phi: 0.5, tau: -0.3, omega: 0.1 };
        assert!(s.is_valid());

        let bad = NodeState { phi: f32::NAN, tau: 0.0, omega: 0.0 };
        assert!(!bad.is_valid());

        let bad2 = NodeState { phi: 2.0, tau: 0.0, omega: 0.0 };
        assert!(!bad2.is_valid());
    }

    #[test]
    fn test_node_state_clamp() {
        let s = NodeState { phi: 1.5, tau: -1.5, omega: 0.5 };
        let c = s.clamp();
        assert_eq!(c.phi, 1.0);
        assert_eq!(c.tau, -1.0);
        assert_eq!(c.omega, 0.5);
    }

    #[test]
    fn test_node_state_default() {
        let s = NodeState::default();
        assert_eq!(s.phi, 0.0);
        assert_eq!(s.tau, 0.0);
        assert_eq!(s.omega, 0.0);
        assert!(s.is_valid());
    }
}
