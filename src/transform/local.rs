use crate::kernel::node::NodeState;

// Turing reaction-diffusion parameters
const ALPHA: f32 = 0.12;  // phi self-activation
const BETA: f32 = 0.08;   // phi inhibition by tau
const GAMMA: f32 = 0.10;  // tau production by phi^2
const DELTA: f32 = 0.05;  // tau decay
const D_PHI: f32 = 0.15;  // phi diffusion coefficient
const D_TAU: f32 = 0.30;  // tau diffusion (MUST > D_PHI for Turing instability)
const D_OMEGA: f32 = 0.05; // omega diffusion

pub fn transform(state: &NodeState, neighbors: &[NodeState]) -> NodeState {
    if neighbors.is_empty() {
        return NodeState {
            phi: (state.phi * 0.99).tanh(),
            tau: (state.tau * 0.995).tanh(),
            omega: (state.omega * 0.97).tanh(),
        };
    }

    let n = neighbors.len() as f32;
    let avg_phi: f32 = neighbors.iter().map(|nb| nb.phi).sum::<f32>() / n;
    let avg_tau: f32 = neighbors.iter().map(|nb| nb.tau).sum::<f32>() / n;
    let avg_omega: f32 = neighbors.iter().map(|nb| nb.omega).sum::<f32>() / n;

    // Discrete Laplacian: avg_neighbor - self
    let lap_phi = avg_phi - state.phi;
    let lap_tau = avg_tau - state.tau;
    let lap_omega = avg_omega - state.omega;

    // PHI reaction: activates itself, inhibited by tau
    let phi_react = ALPHA * state.phi * (1.0 - state.phi * state.phi)
                  - BETA * state.phi * state.tau;
    let new_phi = (state.phi + phi_react + D_PHI * lap_phi).tanh();

    // TAU reaction: produced by phi^2, decays slowly, diffuses fast
    let tau_react = GAMMA * state.phi * state.phi - DELTA * state.tau;
    let new_tau = (state.tau + tau_react + D_TAU * lap_tau).tanh();

    // OMEGA: driven by local phi gradient magnitude
    let phi_var: f32 = neighbors.iter()
        .map(|nb| (nb.phi - state.phi).powi(2))
        .sum::<f32>() / n;
    let phi_grad = phi_var.sqrt();
    let omega_drive = 0.15 * phi_grad - 0.04 * state.omega.abs();
    let new_omega = (state.omega * 0.80 + omega_drive + D_OMEGA * lap_omega).tanh();

    NodeState { phi: new_phi, tau: new_tau, omega: new_omega }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_isolated_node_decays() {
        let state = NodeState { phi: 0.8, tau: 0.3, omega: 0.5 };
        let result = transform(&state, &[]);
        // Isolated nodes decay toward zero
        assert!(result.phi.abs() < state.phi.abs());
        assert!(result.tau.abs() < state.tau.abs());
        assert!(result.omega.abs() < state.omega.abs());
    }

    #[test]
    fn test_transform_output_in_range() {
        let state = NodeState { phi: 0.5, tau: -0.3, omega: 0.2 };
        let neighbors = vec![
            NodeState { phi: 0.4, tau: -0.2, omega: 0.1 },
            NodeState { phi: 0.6, tau: -0.4, omega: 0.3 },
            NodeState { phi: 0.3, tau: 0.1, omega: -0.1 },
        ];
        let result = transform(&state, &neighbors);
        assert!(result.is_valid(), "transform output must be valid");
        assert!(result.phi.abs() <= 1.0);
        assert!(result.tau.abs() <= 1.0);
        assert!(result.omega.abs() <= 1.0);
    }

    #[test]
    fn test_transform_deterministic() {
        let state = NodeState { phi: 0.3, tau: 0.1, omega: -0.2 };
        let neighbors = vec![
            NodeState { phi: 0.5, tau: 0.0, omega: 0.4 },
            NodeState { phi: -0.1, tau: 0.2, omega: -0.3 },
        ];
        let r1 = transform(&state, &neighbors);
        let r2 = transform(&state, &neighbors);
        assert_eq!(r1, r2, "transform must be deterministic");
    }

    #[test]
    fn test_transform_tau_increases_with_phi() {
        // High phi should drive tau upward (GAMMA * phi^2 > DELTA * tau when tau is small)
        let state = NodeState { phi: 0.9, tau: 0.0, omega: 0.0 };
        let neighbors = vec![
            NodeState { phi: 0.9, tau: 0.0, omega: 0.0 },
            NodeState { phi: 0.9, tau: 0.0, omega: 0.0 },
        ];
        let result = transform(&state, &neighbors);
        // tau should increase since phi^2 > 0 and tau is 0
        assert!(result.tau > state.tau, "tau should increase with high phi");
    }

    #[test]
    fn test_transform_turing_diffusion_asymmetry() {
        // D_TAU > D_PHI means tau diffuses faster than phi
        // Verify the constants are correct for Turing instability
        assert!(D_TAU > D_PHI, "Turing instability requires D_TAU > D_PHI");
    }

    #[test]
    fn test_transform_phi_inhibited_by_tau() {
        // High tau should slow down phi growth
        let state_low_tau = NodeState { phi: 0.5, tau: 0.0, omega: 0.0 };
        let state_high_tau = NodeState { phi: 0.5, tau: 0.9, omega: 0.0 };
        let neighbors_low = vec![state_low_tau; 2];
        let neighbors_high = vec![state_high_tau; 2];

        let result_low = transform(&state_low_tau, &neighbors_low);
        let result_high = transform(&state_high_tau, &neighbors_high);

        // Higher tau should produce lower (or slower growing) phi
        assert!(result_high.phi <= result_low.phi,
            "high tau should inhibit phi: {} vs {}", result_high.phi, result_low.phi);
    }
}
