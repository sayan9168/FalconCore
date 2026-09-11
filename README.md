# FalconCore

![CI](https://github.com/sayan9168/FalconCore/actions/workflows/ci.yml/badge.svg)

FalconCore is an experimental, security-minded programming language and embeddable runtime built in Rust.

## v0.7 — Module System Foundation

v0.7 adds the first real module/package boundary while keeping module loading explicit and filesystem-scoped.

- `import "path/to/module"` syntax
- Optional import aliases with `as`
- `export` syntax for module-visible declarations
- Relative `.falcon` module resolution
- Recursive dependency discovery
- Missing-module diagnostics at resolver level
- Import-cycle detection
- Module AST representation
- Compiler/type-checker support for module declarations
- Version bumped to `0.7.0`

### Module example

```falcon
import "lib/math" as math

export secure const answer = 42
print answer
```

The resolver treats an extensionless import such as `lib/math` as `lib/math.falcon` relative to the importing file. It canonicalizes files, tracks the dependency stack, and rejects cycles instead of silently recursing.

## Quick Start

```bash
cargo run -- --eval 'print 2 + 3 * 4'
cargo run -- --file examples/hello.falcon
cargo run -- --tokens 'print 42'
cargo run -- --ast 'print 42'
cargo run -- --bytecode 'print 42'
cargo run -- --check 'print 42'
cargo run -- --version
```

## Architecture

```text
                  FalconCore Toolchain
                         │
        ┌────────────────┼─────────────────┐
        ↓                ↓                 ↓
      Lexer            Parser            CLI
        │                │                 │
        └─────────────── AST ──────────────┘
                         ↓
                  Module Resolver
                         ↓
                Type Checking / Diagnostics
                         ↓
                  Bytecode Compiler
                         ↓
             Constants + Instructions
                         ↓
                  Stack-based VM
                         ↓
                Call Frames / Locals
                         ↓
             Explicit Host Capabilities
```

## Security Model

Security-sensitive operations remain explicit capabilities. Module resolution is filesystem-scoped to the importing file's directory and rejects missing files and dependency cycles. `network.scan` remains behind an explicit runtime capability boundary and is intended to connect later to an authorized host-side API with policy enforcement.

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
- [x] Float support
- [x] CI
- [x] Developer CLI
- [x] Bytecode disassembler
- [x] Structured compiler diagnostics foundation
- [x] Module/import resolver foundation
- [ ] First-class `bool` / `null` values
- [ ] Immutable-constant enforcement
- [ ] Bytecode serialization and versioning
- [ ] Capability registry and permission policy
- [ ] Native/AOT backend stabilization
- [ ] Language server / editor integration

## License

Apache License 2.0.
