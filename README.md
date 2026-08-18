# hello-world

Minimal Rust demo crate used to validate an air-gapped
Linux → Windows (`x86_64-pc-windows-gnu`) cross-compilation pipeline
with `cargo-auditable` packaging.

## What this is

- A zero-dependency binary (`hello-world`, with a `count N` mode) —
  no external crates, so there's no supply chain to audit and no
  registry access needed to build.
- A release profile tuned for a small, fast binary: LTO, single
  codegen unit, `panic = "abort"`, stripped symbols.
- Build instructions for producing a Windows binary from a
  completely offline Linux machine, packaged with `cargo-auditable`
  so the resulting `.exe` carries an embedded SBOM.

See [docs/AIRGAPPED_BUILD.md](docs/AIRGAPPED_BUILD.md) for the exact
offline pipeline.

## Local build (online, native target)

```sh
cargo build --release
./target/release/hello-world
./target/release/hello-world count 5
```

## Scope note

This is a boilerplate capability demo, not a finished product. Two
items from the original spec were adjusted:

- **"Security/latency-optimized Cargo.toml"**: a manifest doesn't
  control runtime latency — what's actually tunable is the release
  profile (above) and dependency surface (zero, here). Documented
  as such rather than claiming a metric that doesn't apply.
- **Temperature benchmark**: dropped in favor of build-time and
  binary-size numbers, which are the metrics that actually mean
  something for a hello-world binary.
