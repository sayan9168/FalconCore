# FalconCore

![CI](https://github.com/sayan9168/FalconCore/actions/workflows/ci.yml/badge.svg)
![Version](https://img.shields.io/badge/version-0.2.0-blue?style=for-the-badge)
![License](https://img.shields.io/badge/license-Apache--2.0-green?style=for-the-badge)

**FalconCore** is an experimental, security-minded programming language built from scratch in Rust. The project focuses on a small, embeddable execution pipeline that can evolve toward an independent runtime, VM and native tooling ecosystem.

## v0.2 highlights

- Clean lexer → parser → bytecode compiler → VM pipeline
- Precedence-aware arithmetic parsing (`+`, `-`, `*`, `/`)
- Comparison operators (`==`, `!=`, `>`, `<`, `>=`, `<=`)
- `secure let` and `secure const` syntax
- `if` / `else` blocks
- `repeat` blocks
- Structured runtime errors instead of unchecked stack operations
- Division-by-zero protection
- Public library modules for embedding FalconCore
- Rust unit tests and GitHub Actions CI
- Cleaned duplicate/broken source that had accumulated in the bootstrap implementation

## Quick start

```bash
cargo run
```

Build a release binary:

```bash
cargo build --release
```

Run the test suite:

```bash
cargo test
```

## Language example

```falcon
secure let x = 10
secure let y = 32

print x + y

repeat 3 {
    print "FalconCore"
}

if x < y {
    print "comparison: true"
} else {
    print "comparison: false"
}
```

## Architecture

```text
Falcon source
     │
     ▼
   Lexer
     │ tokens
     ▼
   Parser
     │ AST
     ▼
  Compiler
     │ bytecode
     ▼
    VM
     │
     ▼
 Runtime
```

The compiler and VM are intentionally kept small so future work can add a stable value representation, function ABI, module system, diagnostics, bytecode serialization, and native/AOT backends without coupling the language front-end to a particular host application.

## Security direction

FalconCore is intended for legitimate software development, security research and controlled environments. Security-sensitive capabilities should be explicit, auditable and opt-in rather than hidden inside the runtime.

The current `network.scan` syntax is represented as a capability boundary in the VM; unrestricted network activity is not silently executed by the language runtime.

## Roadmap

- [x] Lexer foundation
- [x] Parser foundation
- [x] Bytecode compiler foundation
- [x] Safe VM error handling
- [x] CI and tests
- [ ] Structured diagnostics instead of parser panics
- [ ] First-class value types
- [ ] Function call frames and closures
- [ ] Bytecode serialization/versioning
- [ ] Package/module system
- [ ] Stable CLI (`falcon check`, `falcon run`, `falcon build`)
- [ ] AOT/native backend stabilization
- [ ] Standard library and capability model

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Please keep changes focused, tested and documented.

## License

Apache License 2.0. See [LICENSE](LICENSE).

Made with ❤️ by SAYAN.
