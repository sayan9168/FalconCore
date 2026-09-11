# FalconCore

![CI](https://github.com/sayan9168/FalconCore/actions/workflows/ci.yml/badge.svg)

FalconCore is an experimental, security-minded programming language and embeddable runtime built in Rust.

## v1.2 — Cross-Module Language Platform

v1.2 turns the v1.1 namespace foundation into a real source-level module workflow: qualified names are lexed and parsed, resolved module graphs are linked into a single compilation unit, exported declarations from dependency modules receive stable namespaces, and the CLI now runs/checks/builds the linked program.

### What's new

- `::` namespace separator in the lexer
- Qualified identifiers and calls such as `math::add(2, 3)`
- Cross-module linker over the existing dependency resolver
- Stable module namespaces derived from module names
- Export-only dependency exposure
- Import aliases mapped to the linked module namespace
- Qualified references inside exported dependency declarations
- CLI `run`, `check`, and `build` now use the linker for source files
- Fixed the module AST container's invalid `Eq` derivation
- Version bumped to `1.2.0`

### Example

`math.falcon`:

```falcon
export fn add(a, b) {
    return a + b
}
```

`main.falcon`:

```falcon
import "math.falcon" as math
print math::add(2, 3)
```

Then:

```bash
falconcore run main.falcon
falconcore check main.falcon
falconcore build main.falcon
```

The linker keeps dependency declarations namespaced (`math::add`) while the entry module remains the executable root.

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
                         FalconCore v1.2
                                │
          ┌─────────────────────┼──────────────────────┐
          ↓                     ↓                      ↓
        Lexer                 Parser                   CLI
          │                     │                      │
          └─────────────────── AST ───────────────────┘
                                ↓
                       Module Resolution
                                ↓
                    Namespace-aware Linker
                                ↓
                       Symbol + Type Check
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

FalconCore follows a capability-oriented model: sensitive host operations are opt-in, deny-by-default, and policy controlled. Module imports are resolved relative to the importing file, missing modules and dependency cycles are rejected, and the linker exposes dependency declarations only through their exported namespace. The runtime does not silently grant operating-system access.

FCBC artifacts validate their magic header, format version, typed constant tags, opcode tags, UTF-8 strings, and trailing data before execution.

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
- [ ] Immutable-constant assignment enforcement
- [ ] Capability-aware VM opcode dispatch
- [ ] Registry-backed dependency resolution
- [ ] Dependency graph solver and lockfile verification
- [ ] FCBC v3
- [ ] Optimizer / bytecode optimization passes
- [ ] Native/AOT backend stabilization
- [ ] Language server / editor integration
- [ ] Standard library and package registry

## License

Apache License 2.0.
