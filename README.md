# leptos-basic-template

A basic template for building a frontend application using [Leptos](https://leptos.dev/) with Rust, compiled to WebAssembly (WASM).

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [wasm32 target](https://rustwasm.github.io/wasm-pack/installer/): `rustup target add wasm32-unknown-unknown`
- [Trunk](https://trunkrs.dev/): `cargo install trunk`

## Getting Started

### Development server

```bash
trunk serve --open
```

This will compile the app to WASM and serve it at `http://localhost:8080`.

### Production build

```bash
trunk build --release
```

The output will be in the `dist/` directory.

## Project Structure

```
leptos-basic-template/
├── src/
│   ├── main.rs    # Entry point
│   └── app.rs     # Root App component
├── index.html     # HTML entry point used by Trunk
├── Trunk.toml     # Trunk configuration
└── Cargo.toml     # Rust dependencies
```

## Dependencies

- `leptos` with the `csr` feature for client-side rendering compiled to WASM