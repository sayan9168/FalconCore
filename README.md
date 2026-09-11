# FalconCore

![CI](https://github.com/sayan9168/FalconCore/actions/workflows/ci.yml/badge.svg)

FalconCore is an experimental, security-minded programming language and embeddable runtime built in Rust.

## v1.1 — Real Language Platform Foundation

v1.1 extends the v1.0 security/platform primitives with deterministic qualified symbol resolution, stronger capability boundaries, and reproducible package-lock output.

### What's new

- Qualified module references using the `module::symbol` namespace model in the symbol resolver
- Deterministic module namespace construction with `BTreeMap`
- Export-only qualified resolution: private symbols cannot be imported through the resolver
- Symbol validation for empty module/symbol names
- Capability batch authorization and usage snapshots
- Explicit policy inspection without granting capabilities
- Deterministic package-lock serialization
- Lock validation for unique, sorted dependencies and required metadata
- Escaping of package metadata in lock output
- Version bumped to `1.1.0`

### Qualified symbols

The parser's existing `import` / `export` syntax remains source-compatible while the platform now has a canonical qualified-symbol representation:

```text
math::add
math::answer
```

The resolver accepts only exported symbols, making the module boundary explicit:

```rust
let symbol = resolve_qualified(&namespace, "math::answer")?;
```

This is the platform foundation for future grammar-level namespace expressions and import binding resolution.

### Capability boundary

Capabilities remain deny-by-default. v1.1 adds policy inspection, batch requests, and auditable usage snapshots without granting operating-system access automatically.

```rust
let policy = CapabilityPolicy::deny_all()
    .allow(Capability::NetworkScan)
    .limit(Capability::NetworkScan, 10);
let mut registry = CapabilityRegistry::new(policy);
registry.request(Capability::NetworkScan)?;
let usage = registry.snapshot();
```

`network.scan` remains a safe placeholder until a host integrates an authorized implementation. The language runtime does not silently turn a language-level capability request into arbitrary OS access.

### Reproducible package locks

The package layer can now produce deterministic lock text from sorted dependency records:

```rust
let mut lock = PackageLock::default();
lock.add(LockedDependency {
    name: "core".into(),
    version: "1.1.0".into(),
    source: "registry".into(),
    checksum: Some("...".into()),
});
let lock_text = lock.to_lock_text()?;
```

This is metadata only. FalconCore does not download or execute third-party packages.

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
                         FalconCore v1.1
                                │
          ┌─────────────────────┼──────────────────────┐
          ↓                     ↓                      ↓
        Lexer                 Parser                   CLI
          │                     │                      │
          └─────────────────── AST ───────────────────┘
                                ↓
                       Module Resolution
                                ↓
                  Namespace + Symbol Analysis
                                ↓
                       Type Checking
                                ↓
                    Capability Policy Boundary
                                ↓
                       Bytecode Compiler
                                ↓
                       FCBC Serializer
                                ↓
                         Stack-based VM
                                ↓
                    Explicit Host Integration
```

## Security Model

FalconCore follows a capability-oriented model: sensitive host operations are opt-in, deny-by-default, and policy controlled. The registry supports explicit authorization, bounded use counts, batch checks, and usage snapshots. Filesystem module resolution remains scoped to the importing file's directory and rejects missing files and dependency cycles.

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
- [x] CI configuration
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
- [x] Qualified symbol resolver foundation
- [x] Deterministic package-lock serialization
- [x] Capability usage snapshots and batch authorization
- [ ] Namespace syntax and import binding in the language grammar
- [ ] Immutable-constant assignment enforcement
- [ ] Capability-aware VM opcode dispatch
- [ ] Registry-backed dependency resolution
- [ ] Dependency graph solver and lockfile verification
- [ ] Native/AOT backend stabilization
- [ ] Language server / editor integration
- [ ] Standard library and package registry

## License

Apache License 2.0.
