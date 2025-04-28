# Cool‑rs

A lexical analyzer and parser for the [COOL](https://theory.stanford.edu/~aiken/software/cool/cool.html) programming language, written in Rust.

---

## Description

`cool-rs` provides:

- **Lexical analysis**: A scanner that reads a `.cl` file, breaks it into tokens, and reports any invalid lexemes.
- **Parsing**: A LALRPOP‑generated parser that takes your token stream and builds a Rust AST (abstract syntax tree) representing classes, features, and expressions.

This tool is intended as the front end for a COOL compiler or interpreter, enabling semantic checks, code generation, or further analysis.

---

## Features

### Token kinds

The scanner recognizes these token types:

```rust
// Operators
Add, Sub, Mul, Div, Neg,
Lt, Le, Gt, Ge, Eq,

// Keywords
If, Then, Else, Fi,
While, Loop, Pool,
Let, In, Case, Of, Esac,
New, IsVoid, Not,
Class, Inherits,
True, False,

// Identifiers & Types
Objectid(String), Typeid(String),

// Literals
IntConst(String), StrConst(String), BoolConst(bool),

// Symbols
Colon, Semicolon, Comma,
Dot, Lparen, Rparen, Lbrace, Rbrace, At,
Assign, Darrow,

// Error
Error(String)
```

### Token struct

Each lexeme is wrapped in a `Token`:

```rust
pub struct Token {
    pub ty: TokenType,
    pub lexeme: Vec<u8>,
    pub literal: Option<Literal>,
    pub loc: Loc,
}
```

- `lexeme: Vec<u8>` lets us efficiently handle raw bytes and convert to `String` or slices.
- `literal` is `Some(...)` only for tokens that carry data (identifiers, numbers, strings, booleans).
- `loc` captures line/column for error reporting.

---

## Grammar & Parser

We use [LALRPOP](https://github.com/lalrpop/lalrpop) for parsing:

- All grammar rules live in `src/cool.lalrpop`.
- Build is driven by a `Makefile` that first runs `lalrpop` to generate `src/cool.rs`.
- The parser produces an AST defined in `src/ast.rs` with types like `Class`, `Feature`, `Expr`, etc.

### AST overview

Key AST nodes:

- `Class { name: String, inherits: Option<String>, feature_list: Vec<Feature> }`
- `Feature::Attribute(VarDecl)` or `Feature::Method(...)`
- `Expr` variants: `Identifier`, `Int`, `Bool`, `Str`, `Math`, `Comparison`, `Dispatch`, `If`, `While`, `Case`, etc.

---

## Getting started

### Prerequisites

- Rust (1.70+)
- Cargo
- `lalrpop` CLI (optional, installed via `cargo install lalrpop`)

### Clone and build

```bash
git clone https://github.com/yourusername/cool-rs.git
cd cool-rs
make all        # runs lalrpop + cargo build
```

If you prefer pure Cargo without the Makefile, add a `build.rs`:

```rust
fn main() {
    lalrpop::process_root().unwrap();
}
```

and then simply:

```bash
cargo build
```

---

## Usage

### Scan only

```bash
cargo run -- --file examples/hello.cl
```

This prints the parsed AST to stdout.

### Example

`examples/simple.cl`:

```cool
CLASS A INHERITS IO {
  x: Int = 42;
  f(y: Int): Int { y + x; };
};
```

```bash
cargo run -- --file examples/simple.cl
```

---

## Testing

Run the built‑in unit tests for the scanner and parser:

```bash
cargo test
```

Add more tests under `tests/` for end‑to‑end parsing checks.

---

## Roadmap

- Semantic analysis (type checking)
- Code generation to JVM bytecode or C++
- REPL mode

