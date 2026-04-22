# oxide-forge

> A Rust-based WebAssembly runtime for vibecoders.

`oxide-forge` is an embeddable, batteries-included WebAssembly runtime
written in Rust. It is designed for developers who want to ship fast,
sandboxed code without wrestling with low-level toolchains — just write,
compile to `.wasm`, and forge.

## Why oxide-forge?

- **Vibecoder-first ergonomics** — sensible defaults, zero-config start, friendly errors.
- **Rust-powered** — memory-safe host, predictable performance, no GC pauses.
- **Sandboxed by default** — untrusted modules stay in their lane.
- **Embeddable** — use it as a CLI or drop the `oxide_forge` crate into your own app.
- **Portable** — runs anywhere Rust runs: Linux, macOS, Windows, and beyond.

## Status

Early-stage / pre-alpha. The public API will change. Pin a specific
version if you depend on it.

## Installation

### From source

```bash
git clone https://github.com/nikhilranjan/oxide-forge.git
cd oxide-forge
cargo build --release
```

The binary will be available at `target/release/oxide-forge`.

### As a library

Add to your `Cargo.toml`:

```toml
[dependencies]
oxide-forge = { git = "https://github.com/nikhilranjan/oxide-forge" }
```

## Usage

### CLI

```bash
oxide-forge
```

### Library

```rust
use oxide_forge::Runtime;

fn main() {
    let runtime = Runtime::new();
    println!("{} v{}", Runtime::name(), runtime.version());
}
```

## Project layout

```
oxide-forge/
├── Cargo.toml        # crate manifest
├── src/
│   ├── lib.rs        # library entry point
│   └── main.rs       # CLI entry point
├── LICENSE           # Apache-2.0 license text
├── NOTICE            # attribution notice
├── README.md         # you are here
└── .gitignore
```

## Development

Common workflows:

```bash
cargo build             # debug build
cargo build --release   # optimized build
cargo test              # run unit tests
cargo fmt               # format code
cargo clippy            # lint
cargo run               # run the CLI
```

## Roadmap

- [ ] WebAssembly module loader
- [ ] Host function linker
- [ ] WASI support
- [ ] Component model support
- [ ] Async execution
- [ ] Hot reload for vibe-driven development
- [ ] Plugin system

## Contributing

Issues and pull requests are welcome. By submitting a contribution you
agree to license it under the project's Apache-2.0 license.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](./LICENSE)
and [NOTICE](./NOTICE) for details.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in this work, as defined in the Apache-2.0
license, shall be licensed as above, without any additional terms or
conditions.
