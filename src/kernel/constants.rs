//! Physical constants for Reality Engine V0.2.
//!
//! All constants tuned for numerical stability on the i3-4000M target.

/// Double-well potential V(ψ) = -WELL_A·ψ² + WELL_B·ψ⁴
/// Minima (vacuum expectation values) at ψ_± = ±sqrt(WELL_A / (2·WELL_B)) = ±1.0
pub const WELL_A: f32 = 0.10;
pub const WELL_B: f32 = 0.05;

/// Secondary field potential U(φ) = FIELD_M2·φ²/2 (harmonic / mass term)
pub const FIELD_M2: f32 = 0.04;

/// Inter-field coupling strength ψ↔φ
pub const COUPLING_G: f32 = 0.015;

/// Integration time step.
/// Stability requires: DT < 1/sqrt(|V''(ψ_vac)|) = 1/sqrt(4·WELL_A) ≈ 1.58
/// Also DT < 1/sqrt(FIELD_M2) = 5.0. We use 0.12 for comfortable stability margin.
pub const DT: f32 = 0.12;

/// Metric field (χ) responds to local energy concentration
pub const METRIC_RESPONSE: f32 = 0.0003;
/// Metric field spatial diffusion (keeps χ smooth)
pub const METRIC_DIFFUSION: f32 = 0.015;
pub const METRIC_MIN: f32 = 0.20;
pub const METRIC_MAX: f32 = 2.50;

/// Topology mutation constants (ω field)
pub const OMEGA_DECAY: f32 = 0.92;
pub const OMEGA_GRADIENT_DRIVE: f32 = 0.10;

/// Initial condition noise amplitude (small: near unstable equilibrium ψ=0)
pub const INIT_NOISE: f32 = 0.04;
