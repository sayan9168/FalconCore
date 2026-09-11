# FalconCore

![CI](https://github.com/sayan9168/FalconCore/actions/workflows/ci.yml/badge.svg)

FalconCore is an experimental, security-minded programming language and embeddable runtime built in Rust.

## v1.0 — Security Platform Foundation

v1.0 turns FalconCore's module boundary and security model into explicit platform primitives while keeping the runtime dependency-free.

### What's new

- First-class module symbol analysis with deterministic symbol tables
- Explicit import/export tracking and export validation
- Duplicate declaration detection
- Capability policy engine with deny-by-default behavior
- Capability registry with per-capability usage limits
- Package manifest primitives with dependency declarations
- Minimal dependency lock representation for reproducible package metadata
- Version bumped to `1.0.0`

### Security capabilities

```rust
let policy = CapabilityPolicy::deny_all()
    .allow(Capability::NetworkScan)
    .limit(Capability::NetworkScan, 10);
let mut registry = CapabilityRegistry::new(policy);
registry.request(Capability::NetworkScan)?;
```

The runtime does not grant operating-system access merely because a program requests a capability. A host application must explicitly construct and pass an appropriate policy/registry. `network.scan` remains a safe placeholder until a host integrates an authorized implementation.

### Module symbols

A module can expose declarations with `export`:

```falcon
export secure const answer = 42
export fn add(a, b) { return a + b }
```

FalconCore can now analyze those declarations into a deterministic symbol table and distinguish variables, constants, and functions. Non-declarations cannot be exported, and duplicate declarations are rejected by the symbol analyzer.

### Package metadata

The v1.0 package layer provides a dependency-free manifest model and a deliberately small TOML reader for the package fields FalconCore needs:

```toml
[package]
name = "demo"
version = "1.0.0"
entry = "src/main.falcon"

[dependencies]
core = "1"
```

This is a foundation for a future registry/lock resolver; it does not download or execute third-party packages.

## Quick Start

```bash
cargo run -- eval 'print 2 + 3 * 4'
cargo run -- run examples/hello.falcon
cargo run -- build examples/hello.falcon
cargo run -- inspect examples/hello.fbc
cargo run -- run-fbc examples/hello.fbc
cargo run -- check examples/hello.falcon
cargo run -- --version
```

## Architecture

```text
                    FalconCore v1.0
                           │
          ┌────────────────┼─────────────────┐
          ↓                ↓                 ↓
        Lexer            Parser             CLI
          │                │                 │
          └─────────────── AST ──────────────┘
                           ↓
                  Module Resolver
                           ↓
              Symbol Analysis + Type Check
                           ↓
                  Capability Policy
                           ↓
                    Bytecode Compiler
                           ↓
                 FCBC Serializer
                           ↓
                    Stack-based VM
                           ↓
               Explicit Host Boundary
```

## Security Model

FalconCore follows a capability-oriented model: sensitive host operations are opt-in, deny-by-default, and policy controlled. The v1.0 registry supports explicit authorization and bounded use counts. Filesystem module resolution remains scoped to the importing file's directory and rejects missing files and dependency cycles.

FCBC artifacts validate their magic header, format version, typed constant tags, opcode tags, UTF-8 strings, and trailing data before execution. Malformed artifacts are rejected by the decoder.

## Development

```bash
cargo fmt --all
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## Roadmap

- [x] Lexer / parser foundation
- [x] Bytecode compiler
- [x] Hardened stack VM
- [x] Function calls and call frames
- [x] Float / bool / null runtime values
- [x] CI
- [x] Developer CLI
- [x] Bytecode disassembler
- [x] Structured compiler diagnostics foundation
- [x] Module/import resolver foundation
- [x] Module-aware `check` and `build`
- [x] FCBC bytecode serialization and versioning
- [x] Module symbol table foundation
- [x] Export validation
- [x] Capability registry / policy foundation
- [x] Package manifest / lock primitives
- [ ] Qualified module namespaces in the language grammar
- [ ] Immutable-constant assignment enforcement
- [ ] Capability-aware VM opcode dispatch
- [ ] Registry-backed dependency resolution
- [ ] Native/AOT backend stabilization
- [ ] Language server / editor integration

## License

Apache License 2.0.
