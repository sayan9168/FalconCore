# FalconCore

![CI](https://github.com/sayan9168/FalconCore/actions/workflows/ci.yml/badge.svg)

FalconCore is an experimental, security-minded programming language and embeddable runtime built in Rust.

## v0.4 — Developer Toolchain Jump

v0.4 turns FalconCore from a runtime prototype into a usable language toolchain foundation.

- Standalone CLI entry point
- Inline execution with `--eval`
- File execution with `--file`
- Lexer inspection with `--tokens`
- AST inspection with `--ast`
- Bytecode inspection with `--bytecode`
- Version/help commands
- Dedicated bytecode disassembler module
- Executable `.falcon` example
- Removed unused native-code-generation dependencies from the default build
- Preserved VM call frames, recursion limits, mixed numeric arithmetic, and explicit security capabilities

## Quick Start

```bash
cargo run -- --eval 'print 2 + 3 * 4'
cargo run -- --file examples/hello.falcon
cargo run -- --tokens 'print 42'
cargo run -- --ast 'print 42'
cargo run -- --bytecode 'print 42'
cargo run -- --version
```

Once installed as a binary, the same interface is available as:

```bash
falconcore --eval 'print "Hello, FalconCore"'
falconcore --file examples/hello.falcon
```

## Language Example

```falcon
fn add(a, b) {
    return a + b
}

secure let answer = add(20, 22)
print answer

if answer == 42 {
    print "The answer is correct."
} else {
    print "Unexpected result."
}
```

## Architecture

```text
                  FalconCore Toolchain
                         │
        ┌────────────────┼────────────────┐
        ↓                ↓                ↓
      Lexer            Parser           CLI
        │                │                │
        └─────────────── AST ─────────────┘
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

Security-sensitive operations remain explicit capabilities. The runtime does not silently perform unrestricted network activity. `network.scan` currently stops at an explicit capability boundary and is intended to be connected later to an authorized host-side API with policy enforcement.

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
- [ ] Structured parser diagnostics
- [ ] First-class `bool` / `null` values
- [ ] Immutable-constant enforcement
- [ ] Bytecode serialization and versioning
- [ ] Module/package system
- [ ] Capability registry and permission policy
- [ ] Native/AOT backend stabilization
- [ ] Language server / editor integration

## License

Apache License 2.0.
