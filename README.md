# FalconCore

![CI](https://github.com/sayan9168/FalconCore/actions/workflows/ci.yml/badge.svg)

FalconCore is an experimental, security-minded programming language and embeddable runtime built in Rust.

## v0.3 — Execution Engine Jump

- First-class integer, float and string values in the front-end
- Function-call AST with positional arguments
- Real VM call frames with isolated local variables
- Function metadata and bytecode entry points
- Recursive function calls with a bounded call stack
- Arity validation and undefined-function errors
- Mixed integer/float arithmetic
- Floating-point comparisons
- Safer return handling
- Existing explicit `network.scan` capability boundary retained

## Example

```falcon
fn add(a, b) {
    return a + b
}

secure let answer = add(10, 32)
print answer
print 3.5 * 2.0

repeat 2 {
    print "FalconCore v0.3"
}
```

## Architecture

```text
Source
  ↓
Lexer → Parser → AST
               ↓
          Bytecode Compiler
               ↓
      constants + opcodes + functions
               ↓
        Stack VM + Call Frames
               ↓
        explicit capabilities
```

### Function ABI

Each compiled function receives its arguments from the VM stack, binds them to named local slots, and returns one value. Frames carry their own local environment and return instruction pointer. The VM enforces a maximum call depth to prevent unbounded recursion from exhausting the process stack.

## Security model

Security-sensitive operations remain explicit capabilities. The language runtime does not silently perform unrestricted network activity. `network.scan` currently returns a capability-boundary message and is intended to be connected later to an authorized host-side API.

## Development

```bash
cargo fmt --all
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run
```

## Roadmap

- [x] Lexer / parser foundation
- [x] Bytecode compiler
- [x] Hardened stack VM
- [x] Function calls and call frames
- [x] Float support
- [x] CI
- [ ] Structured parser diagnostics
- [ ] First-class `bool` / `null` values
- [ ] Immutable-constant enforcement
- [ ] Bytecode serialization and versioning
- [ ] Module/package system
- [ ] Stable CLI: `falcon check`, `falcon run`, `falcon build`
- [ ] Capability registry and permission policy
- [ ] Native/AOT backend stabilization

## License

Apache License 2.0.
