_WIP_

<img src="https://raw.githubusercontent.com/velostudio/levo/main/maxiquad.png" width="128" />

https://velostudio.github.io/blog/maxiquad.html

# Maxiquad

Create `macroquad.wasm` file:

```bash
cd guests/basic-shapes-example && ./build.sh
```

Or:

```bash
cargo build --manifest-path ./guests/basic-shapes-example/Cargo.toml --release --out-dir ./guests/basic-shapes-example/ -Z unstable-options --target wasm32-wasi

wasm-tools component new ./guests/basic-shapes-example/macroquad_basic_shapes.wasm -o macroquad.wasm --adapt ./guests/wasi_snapshot_preview1.reactor.wasm
```

Run maxiquad:

```bash
cargo r --release -- --path ./macroquad.wasm
```
