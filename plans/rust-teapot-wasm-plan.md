# Rust Teapot WebAssembly Port Plan

## Overview
Port the existing Rust teapot renderer from native desktop (kiss3d) to WebAssembly for browser deployment.

## Current State Analysis
- **Current Implementation**: Uses `kiss3d` library with native windowing via glutin
- **Target Platform**: Web browser via WebAssembly
- **3D Model**: Utah teapot from `teapot.obj` (Wavefront OBJ format)
- **Current Features**:
  - Loads OBJ file at runtime
  - Rotating animation
  - Basic lighting
  - Camera controls

## Technical Approach - REVISED

### kiss3d has Built-in WASM Support!
According to the kiss3d documentation, the library has **WASM compatibility** built-in. The approach is much simpler than implementing a custom WebGL renderer.

### Key Changes Required:
1. **Update to modern kiss3d API**: Use `#[kiss3d::main]` with async/await pattern
2. **Handle file loading**: OBJ file cannot be loaded from filesystem in WASM
   - Solution: Embed OBJ data as a string constant using `include_str!`
3. **Configure for WASM target**: Set up crate-type as `cdylib`
4. **Create HTML host**: Simple HTML file to load the WASM module

## Implementation Plan

### Phase 1: Update Dependencies
1. Update `Cargo.toml`:
   - Keep `kiss3d` but ensure latest version
   - Add `wasm-bindgen` for JS interop
   - Configure crate type as `cdylib`

### Phase 2: Code Updates
1. **Update main.rs**:
   - Change from synchronous `fn main()` to async `#[kiss3d::main]`
   - Use `Window::new().await` pattern
   - Replace file loading with embedded string data
   - Use `window.add_obj_from_str()` if available, or load from embedded data

2. **Embed OBJ data**:
   - Use Rust's `include_str!` macro to embed teapot.obj at compile time
   - Parse and load the model from the embedded string

### Phase 3: HTML Integration
- Create `index.html` with canvas element
- Load WASM module using standard wasm-bindgen approach
- Handle initialization

### Phase 4: Build System
- Use `wasm-pack` for building
- Configure for web target

## File Structure
```
rust-teapot/
├── Cargo.toml          # Updated for WASM target
├── src/
│   ├── lib.rs          # WASM entry point (with #[kiss3d::main])
│   └── teapot_data.rs  # Embedded teapot.obj data
├── www/
│   └── index.html      # HTML host
└── teapot.obj          # Original model file (embedded at compile time)
```

## Dependencies
```toml
[dependencies]
kiss3d = "0.35"
wasm-bindgen = "0.2"

[lib]
crate-type = ["cdylib"]
```

## Build Instructions
```bash
# Install wasm-pack if not already installed
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build for web
wasm-pack build --target web

# Serve with a local server
cd pkg && python3 -m http.server 8000
```

## Browser Compatibility
- Requires WebGL support
- Modern browsers: Chrome, Firefox, Safari, Edge

## Notes
- The teapot.obj data will be embedded as a string constant using `include_str!`
- kiss3d handles all the WebGL/Web rendering internally
- Much simpler than custom WebGL implementation!
