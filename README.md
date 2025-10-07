# gltfpack-sys

Compress glTF/GLB files (equivalent to `gltfpack -cc`).

## Library

```rust
gltfpack_sys::compress("input.glb", "output.glb")?;
```

## CLI

```bash
cargo install --path . --features cli
gltfpack-rs -i input.glb -o output.glb
```

Bundles [meshoptimizer](https://github.com/zeux/meshoptimizer) v0.25 by Arseny Kapoulkine. MIT licensed.
