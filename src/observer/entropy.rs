use crate::kernel::node::NodeState;

/// Measures local order (low entropy) vs disorder (high entropy) across the universe.
pub struct EntropyAnalyzer {
    pub global_phi_entropy: f32,
    pub global_tau_entropy: f32,
    pub phi_mean: f32,
    pub phi_variance: f32,
    pub tau_mean: f32,
    pub active_fraction: f32,
}

impl EntropyAnalyzer {
    pub fn new() -> Self {
        Self {
            global_phi_entropy: 0.0,
            global_tau_entropy: 0.0,
            phi_mean: 0.0,
            phi_variance: 0.0,
            tau_mean: 0.0,
            active_fraction: 0.0,
        }
    }

    pub fn update(&mut self, states: &[NodeState]) {
        let n = states.len() as f32;
        let phi_sum: f32 = states.iter().map(|s| s.phi).sum();
        let tau_sum: f32 = states.iter().map(|s| s.tau).sum();
        self.phi_mean = phi_sum / n;
        self.tau_mean = tau_sum / n;

        let phi_var: f32 = states.iter()
            .map(|s| (s.phi - self.phi_mean).powi(2))
            .sum::<f32>() / n;
        self.phi_variance = phi_var;

        // Approximate differential entropy using variance (Gaussian assumption)
        // H ≈ 0.5 * ln(2*pi*e*variance)
        // We normalize to [0,1] range approximation
        let tau_var: f32 = states.iter()
            .map(|s| (s.tau - self.tau_mean).powi(2))
            .sum::<f32>() / n;

        self.global_phi_entropy = entropy_from_variance(phi_var);
        self.global_tau_entropy = entropy_from_variance(tau_var);

        let active_count = states.iter().filter(|s| s.phi.abs() > 0.1).count();
        self.active_fraction = active_count as f32 / n;
    }
}

impl Default for EntropyAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

fn entropy_from_variance(variance: f32) -> f32 {
    if variance <= 0.0 { return 0.0; }
    // Map variance to [0,1]: max variance for uniform on [-1,1] is 1/3
    (variance * 3.0).min(1.0).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_zero_variance() {
        // All identical states => zero variance => zero entropy
        let states = vec![NodeState { phi: 0.5, tau: 0.2, omega: 0.1 }; 10];
        let mut analyzer = EntropyAnalyzer::new();
        analyzer.update(&states);
        assert_eq!(analyzer.global_phi_entropy, 0.0);
        assert_eq!(analyzer.phi_variance, 0.0);
    }

    #[test]
    fn test_entropy_uniform_distribution() {
        // Spread states evenly across range => high entropy
        let mut states = Vec::new();
        let n = 100;
        for i in 0..n {
            let phi = (i as f32 / n as f32) * 2.0 - 1.0;
            states.push(NodeState { phi, tau: 0.0, omega: 0.0 });
        }
        let mut analyzer = EntropyAnalyzer::new();
        analyzer.update(&states);
        // Variance should be non-trivial, entropy > 0
        assert!(analyzer.global_phi_entropy > 0.0);
        assert!(analyzer.phi_variance > 0.0);
    }

    #[test]
    fn test_entropy_active_fraction() {
        // Half states above 0.1 in phi
        let mut states = vec![NodeState { phi: 0.5, tau: 0.0, omega: 0.0 }; 50];
        states.extend(vec![NodeState { phi: 0.0, tau: 0.0, omega: 0.0 }; 50]);
        let mut analyzer = EntropyAnalyzer::new();
        analyzer.update(&states);
        assert!((analyzer.active_fraction - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_entropy_mean_calculation() {
        let states = vec![
            NodeState { phi: 1.0, tau: 0.0, omega: 0.0 },
            NodeState { phi: -1.0, tau: 0.0, omega: 0.0 },
        ];
        let mut analyzer = EntropyAnalyzer::new();
        analyzer.update(&states);
        assert!((analyzer.phi_mean).abs() < 1e-6, "mean should be 0");
    }

    #[test]
    fn test_entropy_output_bounded() {
        // Entropy should always be in [0, 1]
        let states = vec![
            NodeState { phi: 0.9, tau: -0.8, omega: 0.3 },
            NodeState { phi: -0.7, tau: 0.6, omega: -0.2 },
            NodeState { phi: 0.1, tau: 0.0, omega: 0.8 },
            NodeState { phi: -0.5, tau: 0.4, omega: -0.6 },
        ];
        let mut analyzer = EntropyAnalyzer::new();
        analyzer.update(&states);
        assert!(analyzer.global_phi_entropy >= 0.0 && analyzer.global_phi_entropy <= 1.0);
        assert!(analyzer.global_tau_entropy >= 0.0 && analyzer.global_tau_entropy <= 1.0);
    }
}
