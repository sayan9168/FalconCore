# FalconCore

![CI](https://github.com/sayan9168/FalconCore/actions/workflows/ci.yml/badge.svg)

FalconCore is an experimental, security-minded programming language and embeddable runtime built in Rust.

## v0.9 — Toolchain and Bytecode Artifacts

v0.9 turns the runtime into a small source-to-artifact toolchain while extending the module boundary from a library-only resolver into the normal `check` and `build` workflows.

- `falconcore run <file>` source execution
- `falconcore check <file>` module-aware type checking
- `falconcore build <file>` compilation into `.fbc`
- `falconcore run-fbc <file>` execution of a versioned FCBC artifact
- `falconcore inspect <file>` bytecode/function inspection
- Binary FCBC serializer/deserializer with magic header and versioning
- Function metadata persisted in artifacts
- Module resolution and type checking during builds
- Clear CLI command-oriented help while retaining v0.8 legacy flags
- Version bumped to `0.9.0`

### Build and run

```bash
cargo run -- build examples/hello.falcon
cargo run -- inspect examples/hello.fbc
cargo run -- run-fbc examples/hello.fbc
cargo run -- check examples/hello.falcon
```

The FCBC artifact stores compiler constants, instructions, and function metadata. The format starts with the `FCBC` magic header and a format version, so incompatible artifacts can be rejected instead of executed accidentally.

### Module-aware workflow

```text
main.falcon
   │
   ├── import "lib/math"
   │
   ↓
ModuleResolver
   │
   ├── canonical paths
   ├── missing-module detection
   └── cycle detection
   │
   ↓
TypeChecker (all resolved modules)
   │
   ↓
Compiler (entry module)
   │
   ↓
FCBC artifact
```

## Quick Start

```bash
cargo run -- eval 'print 2 + 3 * 4'
cargo run -- run examples/hello.falcon
cargo run -- build examples/hello.falcon
cargo run -- inspect examples/hello.fbc
cargo run -- run-fbc examples/hello.fbc
cargo run -- tokens 'print 42'
cargo run -- ast 'print 42'
cargo run -- bytecode 'print 42'
cargo run -- check examples/hello.falcon
cargo run -- --version
```

## Architecture

```text
                    FalconCore Toolchain
                           │
          ┌────────────────┼─────────────────┐
          ↓                ↓                 ↓
        Lexer            Parser             CLI
          │                │                 │
          └─────────────── AST ──────────────┘
                           ↓
                    Module Resolver
                           ↓
                  Type Checking / Errors
                           ↓
                    Bytecode Compiler
                           ↓
                 FCBC Serializer
                           ↓
             .fbc Constants + Opcodes
                           ↓
                 FCBC Deserializer
                           ↓
                    Stack-based VM
                           ↓
              Explicit Host Capabilities
```

## Security Model

Security-sensitive operations remain explicit capabilities. Module resolution is filesystem-scoped to the importing file's directory and rejects missing files and dependency cycles. `network.scan` remains behind an explicit runtime capability boundary and is intended to connect later to an authorized host-side API with policy enforcement.

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
- [ ] First-class module namespaces and symbol resolution
- [ ] Immutable-constant assignment enforcement
- [ ] Capability registry and permission policy
- [ ] Native/AOT backend stabilization
- [ ] Language server / editor integration
- [ ] Package manifest and dependency lockfile

## License

Apache License 2.0.
