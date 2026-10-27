# hello_rust_wasm

Minimal Rust + WebAssembly "hello world" using `no_std`, raw `extern "C"` exports, and an xtask to embed the WASM as base64 into a standalone HTML.

## Build

```bash
cargo run -p xtask
```

Output: `target/index.html` (self-contained, no server needed).

## Exports

- `add(a: i32, b: i32) -> i32`
