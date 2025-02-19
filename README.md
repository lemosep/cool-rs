# Description

A lexical analyzer for the [Cool](https://theory.stanford.edu/~aiken/software/cool/cool.html) programming language written in rust.

# Features

A scanner reads inputs from a file passed via command line and consumes and builds the tokens that will be used in next compilation stages. The language contains a set of tokens and reserved words, such as:
```rs
// Operators
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Lt,
    Lteq,
    Gt,
    Gteq,
    Eq,
    
    // Keywords,
    If,
    Fi,
    Else,
    True,
    False,
    In,
    Loop,
    Pool,
    Let,
    While,
    Then,
    Case,
    Esac,
    New,
    Of,
    Not,
    Class,
    IsVoid,
    Inherits,

    Identifier,
    Type,
    // Literals
    String,
    Number,

    // Error
    Error,

    // Misc
    Colon,
    Semicolon,
    Comma,
    Dot,
    Lparen,
    Rparen,
    Lbracket,
    Rbracket,
    At,
    Assign,
    CaseAssign,
    OpenMc,
    CloseMc
``` 
After a lexeme is analyzed, if compatible with the language standards, the lexeme will be converted into an instance of `Token` for further usage. The `Token` struct is described below:
```rs
    pub struct Token {
    pub ty: TokenType,
    pub lexeme: Vec<u8>,
    pub literal: Option<Literal>,
    pub line: usize,
}
```
storing the `lexeme` field in bytes enables Rust to provide methods for converting between strings and byte slices, facilitating operations that require byte-level access.

`literal` contains the wrapper `Option` into it's type since not every token will contain a literal specifying it.

## Running

### Running a .cl file
```bash
cargo run -- --file $FILE_PATH
```

### Running Tests
`scanner.rs` contains tests regarding different scenarios, whether correct or not. To run tests, simply type in your terminal:
```bash
cargo test
```
