# 🧾 Changelog

All notable changes to this project will be documented in this file.
This project adheres to Semantic Versioning (https://semver.org/).

---

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

