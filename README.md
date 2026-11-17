# `comp_macro`

A Rust procedural macro crate providing Python-style list and iterator comprehensions with support for nested `for` loops, pattern destructuring, and multiple `if` filter guards.

## Features

- **`comp!`**: Expands into a lazy, zero-overhead Rust `Iterator` chain (`IntoIterator`, `flat_map`, and `filter_map`).
- **`comp_vec!`**: Evaluates the comprehension and collects the results directly into a `Vec<T>`.
- **Nested `for` Clauses**: Chain multiple `for ... in ...` clauses from outer to inner loops.
- **Multiple `if` Conditions**: Attach zero or more `if` filter conditions to any `for` clause.
- **Pattern Destructuring**: Supports standard single-item Rust patterns (tuples, structs, references, etc.) in loop bindings.

---

## Usage Examples

### Basic Lazy Iterator (`comp!`)

```rust
use comp_macro::comp;

let evens_doubled: Vec<i32> = comp![x * 2 for x in 0..6 if x % 2 == 0].collect();
assert_eq!(evens_doubled, vec![0, 4, 8]);
```

### Nested Loops & Multiple Guards (`comp_vec!`)

```rust
use comp_macro::comp_vec;

let pairs = comp_vec![
    (x, y)
    for x in 1..=4 if x != 2
    for y in 1..x if (x + y) % 2 == 1
];

assert_eq!(pairs, vec![(3, 2), (4, 1), (4, 3)]);
```

### Pattern Destructuring

```rust
use comp_macro::comp_vec;

let items = vec![("a", 1), ("b", 2), ("c", 3)];
let filtered = comp_vec![name for (name, count) in items if count >= 2];

assert_eq!(filtered, vec!["b", "c"]);
```

---

## Project Structure

```text
comp_macro/
├── Cargo.toml
├── README.md
├── src/
│   └── lib.rs                  # Procedural macro entry points, AST structs, Parse & ToTokens impls
└── tests/
    ├── test_comp.rs            # Integration tests for valid macro invocations
    ├── compile_fail.rs         # trybuild test runner for compile-fail UI tests
    └── ui/                     # Compile-fail test cases and expected .stderr snapshots
        ├── invalid_condition.rs
        ├── invalid_condition.stderr
        ├── missing_for.rs
        ├── missing_for.stderr
        ├── missing_in_keyword.rs
        ├── missing_in_keyword.stderr
        ├── non_boolean_condition.rs
        └── non_boolean_condition.stderr
```

---

## How It Works

1. **Parsing (`syn::parse::Parse`)**:
   - `Comp` parses an initial mapping expression (`Mapping`) followed by one or more `ForIfClause` blocks.
   - Each `ForIfClause` parses `for <Pattern> in <Expr>` followed by zero or more `if <Expr>` conditions using `input.peek(Token![if])` so malformed conditions surface accurate syntax errors.
   - Internal AST structs (`Comp`, `Mapping`, `ForIfClause`, `Pattern`, `Condition`) use `pub(crate)` visibility so they remain accessible across internal crate modules without leaking into the public API.

2. **Code Generation (`quote::ToTokens`)**:
   - Clauses are folded in reverse order (from innermost to outermost).
   - The innermost clause becomes a `.filter_map(move |pattern| (true && cond1 && cond2).then(|| mapping))`.
   - Outer clauses wrap the inner iterator using `.flat_map(move |pattern| (true && conds).then(|| inner_iter).into_iter().flatten())`.
   - `move` closures ensure outer loop variables outlive inner iterator stack frames.

---

## Running Tests

### Run Standard & Compile-Fail Tests

```bash
cargo test
```

### Generating / Updating `trybuild` Snapshots

When adding new compile-fail test cases under `tests/ui/*.rs` or updating compiler error spans, run `trybuild` in overwrite mode to generate the `.stderr` baseline files:

```bash
TRYBUILD=overwrite cargo test
```

*(On Windows PowerShell, use `$env:TRYBUILD="overwrite"; cargo test`.)*

After generating the `.stderr` files:
1. Inspect the generated `tests/ui/*.stderr` files to verify that error spans point cleanly to the invalid tokens in the macro invocation.
2. Commit both the `.rs` test files and the `.stderr` snapshot files to version control so future `cargo test` runs detect any regressions in error reporting.

