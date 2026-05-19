# Reality Engine

A deterministic emergent reality substrate. Not a game, not a physics sandbox,
not a scripted simulation. A mathematical universe where everything that exists
must arise from local field interactions — nothing important is hardcoded.

---

## What This Is (and Isn't)

**This IS:**
- A sparse-graph Hamiltonian field theory running on your CPU
- A tool for studying spontaneous emergence of structure from pure mathematics
- An honest computational model of leading cosmological theories
- An open research platform for finding emergent phenomena

**This is NOT:**
- A recreation of Earth physics
- A game engine or particle sandbox
- A neural network or machine learning system
- A claim to have "solved" the universe

The only axioms are: **space, time, and deterministic mathematical transformation.**
Everything else — stability, structure, particle-like boundaries, geometry,
and eventually higher-order complexity — must emerge or it does not exist.

---

## Quick Start

### Build
```bash
# Requires Rust (install from https://rustup.rs)
cargo build --release
```

### Run (terminal mode)
```bash
./target/release/reality-engine
```

### Run (3D browser visualization)
```bash
./target/release/reality-engine --web
# Open: http://localhost:8080
```

### Run (Big Bang origin mode)
```bash
./target/release/reality-engine --web --big-bang
# Open: http://localhost:8080
```

---

## All CLI Flags

| Flag | Default | Description |
|------|---------|-------------|
| `--seed N` | 42 | Deterministic universe seed. Same seed = identical outcome. |
| `--nodes N` | 4096 | Node count (rounded to nearest perfect square grid). |
| `--ticks N` | 0 | Ticks to run. 0 = infinite. |
| `--observe-interval N` | 100 | Print observation every N ticks (terminal mode). |
| `--web` | off | Launch browser visualization server. |
| `--big-bang` | off | Initialize as singularity (energy spike at center, false vacuum everywhere). |
| `--port N` | 8080 | Web server port. |
| `--energy F` | 1.5 | Big Bang energy scale (higher = more violent initial expansion). |
| `--viz` | off | Show ASCII ψ field grid (terminal mode). |
| `--chi-viz` | off | Show ASCII χ metric grid (terminal mode). |
| `--histogram` | off | Show ψ distribution histogram (terminal mode). |
| `--tps N` | 0 | Target ticks/second speed limit (terminal mode). 0 = unlimited. |

### Examples

```bash
# Watch emergence from random seed in terminal
./target/release/reality-engine --viz --histogram --observe-interval 50

# Big Bang at 2× energy, custom seed, browser view
./target/release/reality-engine --web --big-bang --energy 2.0 --seed 777

# Fast benchmark run: 10,000 ticks, measure what emerges
./target/release/reality-engine --ticks 10000 --observe-interval 1000

# Web view, normal emergence, watch symmetry breaking happen
./target/release/reality-engine --web --nodes 4096
```

---

## Browser Controls (Web Mode)

| Control | Action |
|---------|--------|
| 🌌 Big Bang | Reset to singularity; camera flies outward |
| ↺ Reset | New random seed, normal emergence |
| ⏸ Pause / ▶ Resume | Freeze/unfreeze simulation |
| Time Scale buttons | Planck / Second / Day / Week / Month / Year / Decade / Century / Millennium — sets ticks-per-frame for each render |
| Custom tpf input | Type any ticks-per-frame value and press Set |
| Top View | Camera directly overhead |
| Angle View | 45° orbiting perspective |
| Mouse drag | Orbit camera |
| Mouse wheel | Zoom in / out |
| Explore bar | Natural language query: *"show a domain wall"*, *"where is most energy?"*, *"is there life?"* — camera flies to matching region with description |
| Universe Chronicle | Rolling narrative log of cosmic events (symmetry breaking, stability milestones, domain wall formation, era transitions) |
| Esc | Close search result popup |

---

## How It Works: The Physics

### The Fields

Every node in the universe carries six values (24 bytes total):

| Field | Symbol | Role |
|-------|--------|------|
| Primary field | ψ (psi) | Exists in a double-well potential. Breaks symmetry spontaneously. |
| Primary momentum | π (pi) | Rate of change of ψ. Carries kinetic energy. |
| Secondary field | φ (phi) | Harmonic potential. Mass-like. Couples to ψ. |
| Secondary momentum | ρ (rho) | Rate of change of φ. |
| Metric factor | χ (chi) | Local geometry. High energy bends how neighbors interact. |
| Topology flux | ω (omega) | Drives edge creation/destruction at high field gradients. |

### The Core Equation

Each node updates via symplectic leapfrog integration (Hamiltonian mechanics):

```
π_{t+1} = π_t + Δt · [χ·∇²ψ  −  V'(ψ)  −  g·φ]
ψ_{t+1} = ψ_t + Δt · π_{t+1}
```

Where:
- `V(ψ) = −A·ψ² + B·ψ⁴` — the **double-well potential** (identical to the Higgs field mechanism)
- `V'(ψ) = −2A·ψ + 4B·ψ³` — its gradient (force)
- `χ·∇²ψ` — metric-weighted discrete Laplacian (geometry modulates coupling)
- `g·φ` — inter-field coupling (two fields push on each other)

The φ field obeys a harmonic equation (U(φ) = M²φ²/2), giving it mass.

### Why This Produces Structure

**Double-well potential:** ψ=0 is an *unstable* equilibrium. Given any perturbation,
the field must fall to either ψ=+1 or ψ=−1 (the two vacua). Different regions fall
to different vacua. The boundaries between them — **domain walls** — are topologically
stable, persistent, and particle-like. This is not programmed. It is mathematical necessity.

**Metric curvature:** χ responds to local energy density. High-energy regions develop
stronger coupling to their neighbors — waves travel differently through them.
This is analogous (not identical) to mass curving spacetime in General Relativity.

**Wave speed limit:** The leapfrog integration on a discrete lattice produces a maximum
signal propagation speed c = 1/Δt. No information can travel faster. This emerges
from the mathematics of the integration scheme — it is not put in by hand.

**Energy conservation:** Hamiltonian mechanics approximately conserves total energy.
The tanh saturation acts as a UV regulator (like renormalization in QFT) — a tiny
controlled energy drift, not blow-up.

### The Big Bang Mode

The universe initializes with:
- ψ=0 everywhere (false vacuum — unstable, but quiet)
- A Gaussian spike of kinetic energy (π >> 0) at the center
- Metric curvature χ elevated at the singularity

What follows:
1. Energy wave expands at maximum speed c = 1/Δt (inflation analog)
2. The wave front perturbs ψ off the unstable ψ=0 equilibrium
3. Different regions fall to ψ=+1 or ψ=−1 (reheating + symmetry breaking analog)
4. Domain walls crystallize between regions (matter formation analog)
5. χ field concentrates at walls — proto-gravitational wells

**Important:** The universe was never "empty." In quantum mechanics,
empty space = quantum vacuum = a physical state with definite energy and
unavoidable fluctuations (Heisenberg: ΔE·Δt ≥ ℏ/2). The Big Bang most likely
emerged from a quantum vacuum fluctuation, not from literal nothingness.
Our simulation models this: the false vacuum is not nothing — it is the
ground state of the ψ field, energetically loaded by the double-well potential.

---

## What the Observer Measures

The observation layer runs separately from physics (it cannot influence the simulation).
It reports:

| Metric | What it reveals |
|--------|----------------|
| ψ entropy / variance | Order vs. disorder in the primary field |
| Total energy + variance | How well energy is conserved; distribution |
| Kinetic fraction | What fraction of energy is motion vs. configuration |
| Max / avg stability | How long individual nodes hold their field sign |
| Compression ratio | How many macro-regions describe the universe (self-organization measure) |
| Macro-region count | Number of coherent large-scale structures |
| Domain wall edges | Count of edges where ψ changes sign (particle-like boundaries) |
| Symmetry broken | Whether ψ has separated into ±vacuum populations |
| +/− fractions | Matter/antimatter analog ratio |

---

## Architecture

```
reality-engine/
├── src/
│   ├── kernel/
│   │   ├── constants.rs    — Physical constants (DT, coupling, potential parameters)
│   │   ├── node.rs         — NodeState (6×f32, 24 bytes), energy function
│   │   └── universe.rs     — Universe struct, tick engine, init modes
│   ├── topology/
│   │   ├── graph.rs        — Sparse graph (adjacency list + FxHashSet edges)
│   │   └── mutation.rs     — Edge creation/destruction driven by ω field
│   ├── transform/
│   │   └── local.rs        — Hamiltonian field update (the core physics)
│   ├── observer/
│   │   ├── persistence.rs  — Tracks how long field signs remain stable
│   │   ├── entropy.rs      — Energy statistics, field entropy
│   │   ├── compression.rs  — Detects macro-compressible regions
│   │   └── symmetry.rs     — Tracks spontaneous symmetry breaking, domain walls
│   ├── web/
│   │   └── server.rs       — Axum HTTP + WebSocket server, binary frame streaming
│   ├── visualization/
│   │   └── ascii.rs        — Terminal rendering (ψ grid, χ grid, histogram)
│   └── replay/
│       └── recorder.rs     — Rolling snapshot buffer for replay
└── static/
    └── index.html          — Three.js 3D visualization frontend
```

### Data Flow (Web Mode)

```
Universe::tick() ──→ build_frame() ──→ broadcast::Sender ──→ WebSocket ──→ Browser
     ↑                                       ↑
  OS thread                        tokio async runtime
  (CPU-bound)                      (I/O-bound)
```

Binary frame format (per tick, ~12KB for 64×64):
```
[0..8]       tick (u64 LE)
[8..16]      width, height (u32 LE each)
[16..16+N]   ψ quantized to u8 (N = width×height)
[16+N..16+2N] χ quantized to u8
[16+2N..16+3N] ω quantized to u8
[16+3N..]    stats: energy, fractions, domain walls, stability (f32/u32 LE)
```

---

## The Golden Ratio Question

**Is φ = (1+√5)/2 ≈ 1.618 the fundamental ratio of the universe?**

Honest answer: it is not a fundamental constant of physics (unlike c, ℏ, G).
But it is not arbitrary either.

The golden ratio appears wherever **recursive self-similar growth** occurs:
- Fibonacci branching: f(n) = f(n−1) + f(n−2) → ratio converges to φ
- Optimal packing in 2D (sunflower seeds, pine cones)
- Quasicrystal diffraction patterns (Penrose tilings)
- Neural connectivity in certain cortical maps

It emerges in these cases because φ is the positive root of x² = x + 1 —
the characteristic equation of any process where "next = sum of two previous."

**In this simulation, the golden ratio is a hypothesis to test:**
- Does the topology graph develop Fibonacci-like branching ratios?
- Do stable domain sizes follow φ-spaced scales?
- Does the eigenvalue spectrum of the connectivity matrix contain φ?

These are real questions with real answers that the simulation can provide.
Future versions will measure this explicitly.

---

## Roadmap

### Current: V0.3 — Immersive Visualization + Time Control

**Physics (from V0.2):**
- ✅ Energy-conserving dynamics (symplectic leapfrog)
- ✅ Spontaneous symmetry breaking (double-well potential)
- ✅ Proto-gravity (metric field χ)
- ✅ Wave speed limit (c = 1/Δt from mathematics)
- ✅ Domain wall particles (stable topological boundaries)
- ✅ Big Bang mode (singularity → expansion → symmetry breaking)

**Visualization (V0.3 new):**
- ✅ Custom GLSL shader with DataTexture — 256×256 mesh driven by 64×64 sim data via GPU bilinear interpolation (smooth, non-blocky surface)
- ✅ Per-vertex normal computation in vertex shader — correct lighting on terrain features
- ✅ Cinematic color palette: matter (deep red), antimatter (deep blue), domain walls (warm gold glow)
- ✅ ACESFilmic tone mapping + UnrealBloom post-processing
- ✅ 10,000-star field with fog
- ✅ Universe clock displaying age as "X billion years" in cosmological time units
- ✅ Cosmic era labels (Genesis → Inflation → Symmetry Breaking → Matter Era → …)
- ✅ Time scale tiers: Planck / Second / Day / Week / Month / Year / Decade / Century / Millennium
- ✅ Custom ticks-per-frame text input
- ✅ Universe Chronicle — rolling narrative log of cosmic events in plain language
- ✅ Explore / search system — natural language queries fly camera to matching field region with description
- ✅ Fix: no white-plane flash at startup (DataTexture initialized to neutral grey)
- ✅ Fix: symmetry-broken flag requires 100-tick minimum — no false positive at tick 0
- ✅ Adaptive sim loop: low tpf targets 60 fps; high tpf runs at full CPU speed

### V0.4 — Quantum Vacuum Physics (next)
- [ ] Scale-invariant initial power spectrum (Harrison-Zel'dovich)
- [ ] Inflation phase (exponential scale factor expansion)
- [ ] Reheating (inflaton → matter field energy transfer)
- [ ] CP violation analog (seeded matter/antimatter asymmetry from math)
- [ ] Golden ratio measurement in observer
- [ ] 3D grid (32×32×32) with volumetric rendering

### V1.0 — Proto-Chemistry
- [ ] Multiple field species (different masses, coupling constants)
- [ ] Stable soliton bound states (analogs of atoms)
- [ ] Interaction rules emerging from field overlap integrals
- [ ] "Periodic table" of stable topological configurations (emergent, not designed)
- [ ] Bond formation/breaking dynamics
- [ ] Energy landscapes across configuration space

### V1.5 — Self-Organization and Replication
- [ ] Dissipative structures (Prigogine: order through energy flow)
- [ ] Autocatalytic cycles (patterns that produce the conditions for their own continuation)
- [ ] Information density as an observable (Shannon entropy of local state)
- [ ] Selection pressure: patterns that consume less energy and persist longer survive
- [ ] Proto-metabolism: sustained energy processing structures

### V2.0 — Evolutionary Dynamics
- [ ] Variation + selection + heredity (the three conditions for Darwinian evolution)
- [ ] No species are defined — they emerge as stable information boundaries
- [ ] Extinction: when the field configuration supporting a structure becomes unstable
- [ ] Adaptive radiation: when a new field region opens (after topology mutation)
- [ ] Biodiversity measurement (observer layer)
- [ ] Climate analog: large-scale χ field circulation patterns

### V3.0 — Cognitive Emergence
- [ ] Information processing structures (nodes whose state predicts future states)
- [ ] Memory: structures whose current state encodes past interactions
- [ ] Predictive modeling: patterns that reduce their own future energy cost
- [ ] No "intelligence" is programmed — it emerges as the most efficient survival strategy

**Note on timeline:** Each version above represents months of research and development.
The mathematics is real and grounded. The path is legitimate. But emergence of true
life-like complexity from field theory is an unsolved problem in physics —
this project is contributing to that frontier, not claiming to have solved it.

---

## Scientific Honesty

This simulation demonstrates real mathematical principles:

**What is real physics:**
- Hamiltonian dynamics and energy conservation
- Spontaneous symmetry breaking (the actual Higgs mechanism)
- Reaction-diffusion Turing instability (appears in real chemistry and biology)
- Scale-invariant fluctuations (observed in the CMB)
- Domain wall topological stability

**What is honest analogy (not exact physics):**
- Our χ field is proto-gravity, not General Relativity
- Our lattice violates Lorentz invariance (preferred frame exists)
- Our fields are classical, not quantum (no superposition, no entanglement)
- Our "particles" are domain walls, not Standard Model particles

**What is genuinely unknown (open questions this project explores):**
- Does complexity inevitably emerge from any Hamiltonian field theory?
- Is the golden ratio a universal scaling law for self-similar emergence?
- Can evolutionary selection pressure emerge without being programmed?
- What is the minimum mathematical complexity required for proto-life?

Any finding from this simulation that appears novel should be:
1. Characterized mathematically with precision
2. Checked against known physics and mathematics
3. Verified as reproducible (deterministic seed → same result)
4. Submitted for peer review before being claimed as a discovery

The goal is truth, not spectacle.

---

## Hardware Notes

Optimized for low-end hardware (Intel i3-4000M, 8GB RAM, 2GB VRAM):

- Default 64×64 = 4096 nodes: ~1,700 ticks/second
- Each node: 24 bytes → total state ~96KB (fits in L2 cache)
- Web visualization: ~12KB per frame at 30fps = ~360KB/s (minimal bandwidth)
- No GPU dependency for simulation kernel
- Three.js visualization runs on integrated graphics

For higher performance hardware, increase `--nodes`:
- 128×128 = 16,384 nodes: ~400 ticks/second
- 256×256 = 65,536 nodes: ~90 ticks/second (interesting at this scale)

---

## Building from Source

```bash
# Clone
git clone https://github.com/pranav-pradeesh/universe
cd universe

# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build --release

# Run tests
cargo test

# Run
./target/release/reality-engine --web --big-bang
```

---

## Version History

| Version | Description |
|---------|-------------|
| V0.1 | Minimal deterministic kernel: reaction-diffusion, 1D ring topology, basic emergence |
| V0.2 | Hamiltonian field dynamics, spontaneous symmetry breaking, proto-gravity, 2D torus, 3D visualization, Big Bang mode |
| V0.3 | Custom GLSL shader rendering, Universe Chronicle narrative, Explore search system, time-tier controls, universe clock, bug fixes |
| V0.4 | *(planned)* Quantum vacuum origin, inflation, scale-invariant spectrum, golden ratio measurement |

---

*Everything meaningful must arise from recursive local transformation across evolving spacetime.
That is the entire point of the system.*
