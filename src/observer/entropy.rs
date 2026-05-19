use crate::kernel::node::NodeState;

pub struct EntropyAnalyzer {
    pub psi_entropy: f32,
    pub psi_mean: f32,
    pub psi_variance: f32,
    pub total_energy: f32,
    pub energy_mean: f32,
    pub energy_variance: f32,
    pub kinetic_fraction: f32,  // kinetic / total energy ratio
    pub active_fraction: f32,
}

impl EntropyAnalyzer {
    pub fn new() -> Self {
        Self {
            psi_entropy: 0.0,
            psi_mean: 0.0,
            psi_variance: 0.0,
            total_energy: 0.0,
            energy_mean: 0.0,
            energy_variance: 0.0,
            kinetic_fraction: 0.0,
            active_fraction: 0.0,
        }
    }

    pub fn update(&mut self, states: &[NodeState]) {
        let n = states.len() as f32;

        // ψ statistics
        let psi_sum: f32 = states.iter().map(|s| s.psi).sum();
        self.psi_mean = psi_sum / n;
        self.psi_variance = states.iter()
            .map(|s| (s.psi - self.psi_mean).powi(2))
            .sum::<f32>() / n;
        self.psi_entropy = entropy_from_variance(self.psi_variance);

        // Energy statistics
        let energies: Vec<f32> = states.iter().map(|s| s.local_energy()).collect();
        let e_sum: f32 = energies.iter().sum();
        self.total_energy = e_sum;
        self.energy_mean = e_sum / n;
        self.energy_variance = energies.iter()
            .map(|&e| (e - self.energy_mean).powi(2))
            .sum::<f32>() / n;

        // Kinetic vs total energy ratio (how much is "motion" vs "configuration")
        let total_ke: f32 = states.iter()
            .map(|s| 0.5 * s.pi * s.pi + 0.5 * s.rho * s.rho)
            .sum();
        self.kinetic_fraction = if self.total_energy.abs() > 0.001 {
            total_ke / (self.total_energy + total_ke).abs()
        } else {
            0.0
        };

        let active_count = states.iter().filter(|s| s.psi.abs() > 0.1).count();
        self.active_fraction = active_count as f32 / n;
    }
}

fn entropy_from_variance(variance: f32) -> f32 {
    if variance <= 0.0 {
        return 0.0;
    }
    (variance * 3.0).min(1.0).sqrt()
}
