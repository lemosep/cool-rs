# Cool‑rs

A lexical analyzer and parser for the [COOL](https://theory.stanford.edu/~aiken/software/cool/cool.html) programming language, written in Rust. It implements:
  
  - **Lexical Analysis** (scanner)
  - **Parsing** (LALRPOP-generated grammar -> Rust AST)
  - **Semantic Checks** (inheritance, symbol tables, type checking) 

---
## Table of Contents

1. [Description](#description)  
2. [Features](#features)  
   - [Tokens & Scanner](#tokens--scanner)  
   - [Parser & AST](#parser--ast)  
   - [Semantic Analysis](#semantic-analysis)  
3. [Getting Started](#getting-started)  
   - [Prerequisites](#prerequisites)  
   - [Clone & Build](#clone--build)  
4. [Usage](#usage)  
   - [Running the Compiler Front End](#running-the-compiler-front-end)  
   - [Example `.cl` Programs](#example-cl-programs)  
5. [Testing](#testing)  
   - [Test Suite Structure](#test-suite-structure)  
   - [Automated Test Script](#automated-test-script)  
---

## Description

`cool-rs` is a self-contained front end for the COOL language, written in Rust. It reads a `.cl` source file, tokenizes it, parses it into an Abstract Syntax Tree (AST), and performs the following semantic checks:

1. **Inheritance validation**  
   - Duplicate class names  
   - Undefined parents  
   - Illegal inheritance from basic types (`Int`, `String`, `Bool`, `SELF_TYPE`)  
   - Inheritance cycles (excluding the trivial `Object → Object`)  

2. **Class‐level symbol checking**  
   - Duplicate attributes & methods within a class  
   - Invalid method overrides (signature mismatches)  

3. **Expression‐level type checking**  
   - Undefined variables  
   - Undefined classes  
   - Type mismatches in arithmetic/comparison/dispatch/conditional/etc.  
   - Argument‐count mismatches on method calls  
   - Dispatch‐on‐void, case‐on‐void, missing case branch  

If all checks pass, `cool-rs` prints “Semantic checks passed without errors.”

---

## Features

### Tokens & Scanner

The scanner recognizes:

The scanner recognizes:

```rust
// Arithmetic & Logical Operators
Plus, Minus, Mul, Divide, Neg,   // +, –, *, /, ~  
Lt, Le, Eq,                       // <, <=, =  
Not, Isvoid

// Keywords 
Class, Inherits, If, Then, Else, Fi, 
While, Loop, Pool, Let, In, Case, Of, Esac,
New, True, False, Self

// Identifiers & Types
Objectid(String), Typeid(String)

// Literals
IntConst(String), StrConst(Result<String, StringLiteralError>), BoolConst(bool)

// Symbols 
Colon, Semicolon, Comma,
Dot, Lparen, Rparen, Lbrace, Rbrace, At, Assign (“<-“), Darrow (“=>”)

// Error 
Error(String)
```
Each lexeme is wrapped as a `Token`:

```rust
pub enum Token {
    Class_,
    Else,
    Fi,
    If,
    In,
    Inherits,
    Let,
    Loop,
    Pool,
    Then,
    While,
    Case,
    Esac,
    Of,
    New,
    Isvoid,
    Not,
    StrConst(Result<String, StringLiteralError>),
    IntConst(String),
    BoolConst(bool),
    Typeid(String),
    Objectid(String),
    Darrow, Assign, Le, Lbrace, Rbrace, Lparen, Rparen, Colon,
    Semicolon, At, Plus, Minus, Divide, Mul, Neg, Equal, Lt,
    Period, Comma, Error(String),
}
```
-  `StrConst` carries `Result<String, StringLiteralError>` to capture unterminated or invalid escape errors.
- Each `Token` also tracks a `Loc { line: usize, column: usize }` for precise error reporting.

### Parser & AST

We use [LALRPOP](https://lalrpop.github.io/lalrpop/) to generate a `cool.rs` parser from `src/cool.lalrpop`.
Parsed programs produce an AST defined in src/ast.rs, e.g.:

```rust

pub struct Class {
    pub name: String,
    pub inherits: Option<String>,
    pub feature_list: Vec<Feature>,
}

pub enum Feature {
    Attribute(VarDecl),
    Method(String, Vec<ArgDecl>, String, TypedExpr),
}

pub struct VarDecl {
    pub oid: String,
    pub tid: String,
    pub expr: Option<TypedExpr>, // initializer
}

pub struct ArgDecl {
    pub id: String,
    pub tid: String,
}

pub enum Expr {
    Identifier(String),
    Int(i32),
    Bool(bool),
    Str(String),
    Case(Box<TypedExpr>, Vec<CaseBranch>),
    New(String),
    Assignment(String, Box<TypedExpr>),
    // … and more …
}

pub struct TypedExpr {
    pub expr: Expr,
    pub static_type: Option<String>,
    pub line: usize,
}
```
- `TypedExpr` carries its computed type (filled in by the type checker) and its source-line for errors.
- LALRPOP grammar is in `src/cool.lalrpop`.

---

### Semantic Analisys

All code under `src/semantic`:

1. `error.rs` -> Defines `SemanticError` variants (duplicate class, undefined parent, type mismatch, etc.) and a `Display` impl.

2. `collector.rs` -> A simple `ErrorCollector`  of `SemanticError` to accumulate and report.

3. `analyzer.rs` (inheritance checks) ->
    
    * Duplicate class detection
    * Inheriting from basic types (`Int`, `String`, `Bool`, `SELF_TYPE`) forbidden
    * Undefined parent classes
    * Cycles in inheritance graph (excluding trivial `Object - Object`)

4. `symbol.rs` (class-level symbol checks)
    * Builds a `ClassInfo` table (via `class_table.rs`)
    * Detects duplicate attributes/methods in each class
    * Enforces valid method overrides (same parameter types & return type)

5. `type_checker.rs` (expression-level checks)
    * Infers the type of every `TypedExpr` node.
    * Reports undefined variables, undefined classes, mismatched operand types for arithmetic/comparison/dispatch/conditionals, dispatch‐on‐void, etc.
    * Verifies that each method’s body type matches its declared return type

6. `class_table.rs`
    * Builds a lookup map `HashMap<String, ClassInfo> `from each `Class` slice (including built‐in classes).
    * Each `ClassInfo` stores attributes and methods (name, return type, parameter types) for dispatch resolution.

---
## Getting Started

### Prerequisites
* **Rust & Cargo** (1.70+ recommended)
    
    Simply run the command
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
* **LALRPOP** (if you want to regenerate the parser manually):

    ```bash
    cargo install lalrpop
    ```

### Clone & Build
```bash
git clone https://github.com/lemosep/cool-rs.git
cd cool-rs
make all
```
That will:
    1. Run `lalrpop`(via makefile) to generate `src/cool.rs`.
    2. Compile all Rust code (`scanner`, `parser`, `semantic`, etc).

---

## Usage

```bash
cargo run -- --file <PATH_TO_COOL_SOURCE.cl>
```
* **On success**, you'll see the printed AST (including built-ins + user classes) followed by either:
    * `Semantic checks passed without errors.`
    * Or a list of semantic errors (one per line).

---

## Testing

### Test Suite Structure

```
tests/
├── valid/
│   ├── hello.cl
│   ├── arithmetic.cl
│   └── inheritance.cl
└── invalid/
    ├── syntax_error.cl
    ├── type_error.cl
    └── inheritance_cycle.cl
```
* `tests/valid/*.cl`

    Programs that should parse & pass semantic checks.

* `tests/invalid/*.cl`

    Programs designed to trigger errors (parsing failures or semantic failures).

### Automated Test Script

I include `run_tests.sh` at the project root. It:

1. Iterates over `tests/valid`, expects **no errors**.
2. Iterates over `tests/invalid`, expects **either parsing or semantic errors**.

#### How to Run

Make it executable

```bash
chmod +x run_tests.sh
```

Then simply run:
```bash
./run_tests.sh
```
---