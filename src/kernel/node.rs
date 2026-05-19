pub type NodeId = u32;

/// Six-dimensional causal state container — 24 bytes.
///
/// ψ (psi) / π (pi):   Primary scalar field + conjugate momentum.
///                      Obeys double-well potential — spontaneous symmetry breaking.
/// φ (phi) / ρ (rho):  Secondary scalar field + conjugate momentum.
///                      Obeys harmonic potential — mass-like behavior.
/// χ (chi):             Local metric factor. Modulates neighbor coupling strength.
///                      Responds to local energy density — proto-gravity.
/// ω (omega):           Topology flux. Drives edge creation/destruction.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NodeState {
    pub psi: f32,
    pub pi: f32,
    pub phi: f32,
    pub rho: f32,
    pub chi: f32,
    pub omega: f32,
}

impl NodeState {
    pub fn is_valid(&self) -> bool {
        self.psi.is_finite()
            && self.pi.is_finite()
            && self.phi.is_finite()
            && self.rho.is_finite()
            && self.chi.is_finite()
            && self.omega.is_finite()
    }

    /// Local energy density: kinetic + potential (double-well for ψ, harmonic for φ).
    /// Does not include gradient term (that requires neighbors).
    pub fn local_energy(&self) -> f32 {
        use crate::kernel::constants::{WELL_A, WELL_B, FIELD_M2};
        let ke = 0.5 * self.pi * self.pi + 0.5 * self.rho * self.rho;
        let pe_psi = -WELL_A * self.psi * self.psi + WELL_B * self.psi.powi(4);
        let pe_phi = 0.5 * FIELD_M2 * self.phi * self.phi;
        ke + pe_psi + pe_phi
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_valid() {
        assert!(NodeState::default().is_valid());
    }

    #[test]
    fn energy_at_vacuum() {
        use crate::kernel::constants::{WELL_A, WELL_B};
        // At psi = +1 (vacuum), pe_psi = -WELL_A + WELL_B = -0.10 + 0.05 = -0.05
        let s = NodeState { psi: 1.0, pi: 0.0, phi: 0.0, rho: 0.0, chi: 1.0, omega: 0.0 };
        let e = s.local_energy();
        let expected = -WELL_A + WELL_B; // = -0.05
        assert!((e - expected).abs() < 1e-5, "energy at vacuum = {e}, expected {expected}");
    }

    #[test]
    fn energy_at_unstable_equilibrium() {
        // At psi = 0, pe_psi = 0 (local maximum of -V, unstable)
        let s = NodeState::default();
        let e = s.local_energy();
        assert!(e.abs() < 1e-6);
    }

    #[test]
    fn size_is_24_bytes() {
        assert_eq!(std::mem::size_of::<NodeState>(), 24);
    }
}
