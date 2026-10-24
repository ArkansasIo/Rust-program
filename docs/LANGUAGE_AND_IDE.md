# Universal Mathematics Language and IDE roadmap

## Goal

Evolve this repository from the Rust Environment + Mathematics API into a Rust-backed
programming environment with a Lua-inspired scripting language, object-oriented
constructs, and a reusable mathematics library. The current implementation is a
small expression evaluator and library foundation, not yet a complete IDE or a Lua
implementation.

## Shipped expression foundation

The public `language` module supports:
- signed 64-bit integer literals and named integer/boolean variables;
- parenthesized expressions and standard arithmetic precedence;
- checked `+`, `-`, `*`, integer-only `/`, and `%`;
- comparisons `< <= > >= == !=` and boolean operators `! && ||`;
- structured errors with the byte offset of the problem.

Variables are supplied by the caller as a `HashMap<String, Value>`. Evaluation
does not read files, environment variables, invoke processes, access the network,
or mutate state. It also does not implement short-circuit evaluation yet: both
operands of `&&` and `||` are evaluated. Do not treat it as a security sandbox
for untrusted expressions.

Example:

```rust
use rust_environment_api::language::{evaluate, Value};
use std::collections::HashMap;

let variables = HashMap::from([
    ("strength".to_owned(), Value::Integer(18)),
    ("alive".to_owned(), Value::Boolean(true)),
]);
assert_eq!(
    evaluate("strength >= 15 && alive", &variables),
    Ok(Value::Boolean(true))
);
```

## Proposed language layers

1. **Lexer and parser:** expand the existing tokenizer into a syntax tree (AST),
   add statements, source spans, recoverable diagnostics, and a grammar reference.
2. **Runtime values:** add floats, strings, arrays, maps, functions, and explicit
   conversion rules; retain checked arithmetic where integer semantics are desired.
3. **Lua-inspired syntax:** `let`, `fn`, `if/else`, `while`, `for`, `return`,
   and module imports. This is a new language with Lua-like ergonomics, not Lua
   compatibility.
4. **Object-oriented model:** define `class`, fields, methods, constructors,
   interfaces/traits, visibility, inheritance or composition, and method dispatch.
   Prefer explicit semantics before adding syntax sugar.
5. **Math standard library:** integer theory (prime/even/odd/GCD), checked numeric
   operations, floating point, complex numbers, vectors/matrices, statistics,
   probability, symbolic algebra, and numeric algorithms in separate modules.
6. **IDE frontend:** editor component, syntax highlighting, diagnostics, project
   tree, run/debug controls, REPL/console, formatter, and documentation browser.
   Consider an LSP server for completion, go-to-definition, hover, and diagnostics.
7. **Extension/API layer:** versioned public Rust APIs, module manifests, plugin
   capability declarations, dependency/version pinning, and resource limits.

## Suggested architecture

- `src/language.rs`: current expression evaluator.
- `src/math.rs`: deterministic, checked integer helpers.
- `src/ast/` and `src/runtime/`: future parser and execution engine.
- `src/stdlib/`: namespaced mathematics and utility modules.
- `tools/language-lsp/`: future language server.
- `ide/`: future desktop or web IDE client.
- `examples/`: starter programs.
- `tests/`: unit and integration coverage for language semantics.

## Example future syntax (design sketch only)

```lua
class Vector2
    field x: number
    field y: number

    fn magnitude(self)
        return sqrt(self.x * self.x + self.y * self.y)
    end
end

let velocity = Vector2.new(3, 4)
if velocity.magnitude() == 5 then
    print("unit test passed")
end
```

The syntax above is illustrative and is not accepted by the current evaluator.

## Build and validation

```bash
cargo fmt --all -- --check
cargo check --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

## Implementation principles

- Make language semantics deterministic and documented.
- Report parse and evaluation errors; never silently coerce types.
- Use checked integer operations and explicit division rules.
- Keep the evaluator free of side effects until capabilities are designed.
- Add tests for precedence, type errors, overflow, invalid syntax, and edge cases.
- Benchmark only after correctness tests and define resource limits before adding loops.
