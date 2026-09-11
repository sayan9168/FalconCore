# FalconCore

![CI](https://github.com/sayan9168/FalconCore/actions/workflows/ci.yml/badge.svg)

FalconCore is an experimental, security-minded programming language and embeddable runtime built in Rust.

## v1.3 — Compiler + Security Runtime

v1.3 turns the v1.2 language platform into a safer compiler/runtime pipeline. The compiler now performs deterministic AST constant folding before bytecode generation, while sensitive runtime operations cross an explicit deny-by-default capability boundary.

### What's new

- Deterministic AST constant folding optimizer
- Arithmetic and comparison folding for literal values
- Division-by-zero expressions remain unfurled so runtime error semantics are preserved
- Optimizer integrated directly into `Compiler::compile`
- VM capability registry integration
- `NetworkScan` now requires an explicitly allowed `NetworkScan` capability
- Capability usage limits are enforced by the VM
- New runtime errors for denied and exhausted capabilities
- Existing `VM::new` / `with_functions` constructors remain deny-by-default
- New `VM::with_capabilities` constructor for explicit host policy injection
- Version bumped to `1.3.0`

### Security boundary

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
                 FCBC
                    ↓
             Capability-aware VM
                    ↓
        Explicit host capability policy
```

The VM never implicitly grants network access. A host embedding FalconCore must explicitly construct a `CapabilityPolicy`, allow the required capability, and optionally configure a usage limit before `NetworkScan` can proceed. The current network operation remains a safe host-adapter placeholder rather than arbitrary socket access.

### Optimizer example

```falcon
print 2 + 3 * 4
```

Literal arithmetic is folded before bytecode generation, reducing unnecessary runtime work. Expressions such as `10 / 0` are deliberately preserved so the VM can report its normal `DivisionByZero` error.

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
- [ ] Immutable-constant assignment enforcement
- [ ] Registry-backed dependency resolution
- [ ] Dependency graph solver and lockfile verification
- [ ] FCBC v3
- [ ] Bytecode optimization passes beyond constant folding
- [ ] Native/AOT backend stabilization
- [ ] Language server / editor integration
- [ ] Standard library and package registry

## License

Apache License 2.0.
