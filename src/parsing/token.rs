use std::fmt;

#[derive(Debug, Clone, PartialEq)]
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
    StrConst(String),
    IntConst(String),
    BoolConst(bool),
    Typeid(String),
    Objectid(String),
    Darrow,
    Assign,
    Le,
    Lbrace,
    Rbrace,
    Lparen,
    Rparen,
    Colon,
    Semicolon,
    At,
    Plus,
    Minus,
    Divide,
    Mul,
    Neg,
    Equal,
    Lt,
    Period,
    Comma,
    Error(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Class_ => write!(f, "CLASS"),
            Token::Else => write!(f, "ELSE"),
            Token::Fi => write!(f, "FI"),
            Token::If => write!(f, "IF"),
            Token::In => write!(f, "IN"),
            Token::Inherits => write!(f, "INHERITS"),
            Token::Let => write!(f, "LET"),
            Token::Loop => write!(f, "LOOP"),
            Token::Pool => write!(f, "POOL"),
            Token::Then => write!(f, "THEN"),
            Token::While => write!(f, "WHILE"),
            Token::Case => write!(f, "CASE"),
            Token::Esac => write!(f, "ESAC"),
            Token::Of => write!(f, "OF"),
            Token::New => write!(f, "NEW"),
            Token::Isvoid => write!(f, "ISVOID"),
            Token::Not => write!(f, "NOT"),
            Token::StrConst(s) => write!(f, "\"{}\"", s),
            Token::IntConst(s) => write!(f, "{}", s),
            Token::BoolConst(b) => write!(f, "{}", b),
            Token::Typeid(s) => write!(f, "{}", s),
            Token::Objectid(s) => write!(f, "{}", s),
            Token::Darrow => write!(f, "=>"),
            Token::Assign => write!(f, "<-"),
            Token::Le => write!(f, "<="),
            Token::Lbrace => write!(f, "{{"),
            Token::Rbrace => write!(f, "}}"),
            Token::Lparen => write!(f, "("),
            Token::Rparen => write!(f, ")"),
            Token::Colon => write!(f, ":"),
            Token::Semicolon => write!(f, ";"),
            Token::At => write!(f, "@"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Divide => write!(f, "/"),
            Token::Mul => write!(f, "*"),
            Token::Neg => write!(f, "~"),
            Token::Equal => write!(f, "="),
            Token::Lt => write!(f, "<"),
            Token::Period => write!(f, "."),
            Token::Comma => write!(f, ","),
            Token::Error(s) => write!(f, "error({})", s),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Loc {
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexicalError {
    InvalidChar(char, Loc),
    UnterminatedString(Loc),
    InvalidNumber(String, Loc),
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexicalError::InvalidChar(c, loc) => write!(f, "Invalid character '{}' at {}", c, loc),
            LexicalError::UnterminatedString(loc) => write!(f, "Unterminated string at {}", loc),
            LexicalError::InvalidNumber(s, loc) => write!(f, "Invalid number '{}' at {}", s, loc),
        }
    }
}