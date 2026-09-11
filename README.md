# FalconCore

![CI](https://github.com/sayan9168/FalconCore/actions/workflows/ci.yml/badge.svg)

FalconCore is an experimental, security-minded programming language and embeddable runtime built in Rust.

## v1.4 — Production Language Core

v1.4 strengthens the artifact and package trust boundaries introduced in v1.3. FCBC bytecode is now structurally verified before VM execution, and package locks can be checked against manifests before dependency use.

### What's new

- FCBC bytecode verifier module
- VM execution gate: malformed bytecode is rejected before dispatch
- constant-index, jump-target, repeat-target and function-entry validation
- function call arity validation during bytecode verification
- required `Halt` validation for executable artifacts
- structured bytecode verification runtime error
- manifest ↔ lockfile consistency verification
- missing dependency detection
- undeclared lock entry detection
- simple version requirement matching
- registry-source checksum requirement
- version bumped to `1.4.0`

### Security pipeline

```text
Falcon source
     ↓
 Lexer → Parser → Module Linker
                    ↓
             Type / Symbol checks
                    ↓
             AST Optimizer
                    ↓
              Bytecode Compiler
                    ↓
            FCBC serialize/load
                    ↓
             Bytecode Verifier
                    ↓
          Capability-aware VM
                    ↓
        Explicit host capability policy
```

The VM does not execute an externally loaded artifact until the verifier accepts its structural invariants. Sensitive operations still require explicit host capabilities; verification does not grant permissions.

### Package lock verification

```rust
lock.verify_against(&manifest)?;
```

The check ensures every manifest dependency is locked exactly once, its simple requirement matches the locked version, no undeclared package is present, and registry dependencies carry a checksum.

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
- [x] Namespace syntax
- [x] Cross-module linking and CLI integration
- [x] Deterministic package-lock serialization
- [x] Capability usage snapshots and batch authorization
- [x] Compiler constant-folding optimizer
- [x] Capability-aware VM dispatch foundation
- [x] FCBC bytecode structural verification
- [x] Manifest / lock consistency verification
- [ ] Immutable-constant assignment enforcement
- [ ] Registry-backed dependency resolution
- [ ] Full semantic version solver
- [ ] FCBC v3
- [ ] Bytecode optimization passes beyond constant folding
- [ ] Native/AOT backend stabilization
- [ ] Language server / editor integration
- [ ] Standard library and package registry

## License

Apache License 2.0.
