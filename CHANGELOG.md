# 🧾 Changelog

All notable changes to this project will be documented in this file.
This project adheres to Semantic Versioning (https://semver.org/).

---

# [v0.2.1] — 2025-11-01
### 🧠 Summary
Unified MemoryField into a single rotating, JSON-driven buffer with retention, analytics helpers, and updated reflection/manifold writers to feed coherent snapshots into persistent storage.
Finalized GPU substrate boot by producing a fully initialized GpuContext resource and added an interface visualization sprite that reflects live coherence/entropy readings from shared memory.
Replaced the bespoke structure transform with Bevy’s component while migrating the engine to tracing-based diagnostics, including an optional verbose feature and runtime subscriber wiring in main.rs.
Made the functional feedback loop adaptive by deriving decay and phase rates from recent MemoryField coherence and entropy trends, stabilizing loop behaviour based on historical analytics.

### 🧪 Testing
⚠️ cargo check (fails: missing system library wayland-client required by wayland-sys)
⚠️ cargo run (fails: missing system library wayland-client required by wayland-sys)

## [v0.2.0] — 2025-11-01
### ✨ Added
- Implemented `MemoryField` system for circular memory management.
- Integrated reflective feedback loop between `core`, `reflection`, and `manifold`.
- Added persistent `void_state.json` snapshot file.

### 🧠 Improved
- Enhanced initialization flow in `lib.rs` for modular system boot order.
- Refined log outputs for clarity and symbolic hierarchy.
- Updated `README.md` with full module breakdown and philosophy.

### 🧹 Fixed
- Removed redundant `pub use MemoryField` reimport.
- Corrected Bevy `init(app)` signature mismatch.
- Fixed unresolved imports in `manifold` and `reflection`.

---

## [v0.1.0] — 2025-10-31
### 🧩 Added
- Initial stable architecture of Void Engine.
- Core ECS setup with Bevy.
- Base modular systems: `core`, `substrate`, `dynamics`, `structure`, `function`, `reflection`, `interface`, `manifold`.

