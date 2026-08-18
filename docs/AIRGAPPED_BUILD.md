# Air-Gapped Cross-Compile: Linux → Windows (x86_64-pc-windows-gnu)

Everything below is prepared on a machine *with* internet access, then
carried across the air gap (USB/removable media) and installed on the
offline build server.

## 1. On a networked machine: stage the toolchain

```sh
# Download the rustup installer and stable toolchain + Windows GNU target
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup-init.sh

# Fetch offline-installable toolchain components for the current
# host (Linux) plus the Windows GNU cross target
rustup toolchain install stable --profile minimal
rustup target add x86_64-pc-windows-gnu
rustup component add rust-std --target x86_64-pc-windows-gnu

# Locate rustup's local cache to copy across the air gap
# (usually ~/.rustup and ~/.cargo)
tar czf rustup-cache.tar.gz -C "$HOME" .rustup .cargo

# Grab the mingw-w64 linker package (and its dependencies) for the
# offline host's distro, e.g. on Debian/Ubuntu:
apt-get download mingw-w64 mingw-w64-x86-64-dev binutils-mingw-w64-x86-64 \
    gcc-mingw-w64-x86-64

# Install cargo-auditable's binary (or vendor its source) for offline use
cargo install cargo-auditable --root ./cargo-auditable-bin
```

Transfer `rustup-cache.tar.gz`, the `.deb` files, and
`cargo-auditable-bin/` to the air-gapped machine.

## 2. On the air-gapped machine: install

```sh
# Restore the Rust toolchain + Windows target
tar xzf rustup-cache.tar.gz -C "$HOME"
export PATH="$HOME/.cargo/bin:$PATH"

# Install the mingw-w64 linker from the staged .deb packages
sudo dpkg -i mingw-w64*.deb gcc-mingw-w64-x86-64*.deb \
    binutils-mingw-w64-x86-64*.deb

# Install cargo-auditable from the staged binary
cp cargo-auditable-bin/bin/cargo-auditable "$HOME/.cargo/bin/"
```

The `.cargo/config.toml` in this repo already points the
`x86_64-pc-windows-gnu` target at `x86_64-w64-mingw32-gcc`, so no
further linker configuration is needed.

## 3. Build, fully offline

```sh
cargo auditable build --release \
    --target x86_64-pc-windows-gnu \
    --offline
```

This produces `target/x86_64-pc-windows-gnu/release/hello-world.exe`
with an embedded SBOM (verify later with `cargo audit bin
hello-world.exe`, also run offline against a locally mirrored
advisory database if you want vulnerability checks — otherwise it
just confirms the dependency manifest embedded in the binary, which
for this zero-dependency crate is trivially empty).

## 4. Sanity-check on Windows

Copy the `.exe` to a Windows test machine and run:

```
hello-world.exe
hello-world.exe count 5
```

Expect `Hello, world!` and `1 2 3 4 5` respectively.

## Notes on scope

- No dependencies to fetch means step 3 needs no crates.io mirror —
  that's what keeps this "offline" story trivial. A real MVP with
  dependencies would additionally need a vendored `cargo` registry
  (`cargo vendor` + `[source]` overrides in `.cargo/config.toml`)
  staged across the air gap the same way.
- Binary size and build time are the meaningful, reproducible metrics
  for a hello-world binary — a CPU temperature benchmark isn't, since
  there's no sustained workload to generate thermal load.
