//================================================================================
//  Imports
//================================================================================

use std::{collections::HashMap, fmt};

//================================================================================
//  Structs
//================================================================================

pub struct Scanner<'a> {
    source: Vec<u8>,
    pub tokens: Vec<Token>,
    keywords: HashMap<&'a str, TokenType>,
    start: usize,
    current: usize,
    line: usize
}

impl Default for Scanner<'_> {
    fn default() -> Self {
        Scanner {
            source: Vec::new(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: vec![
                ("if", TokenType::If),
                ("fi", TokenType::Fi),
                ("else", TokenType::Else),
                ("true", TokenType::True),
                ("false", TokenType::False),
                ("in", TokenType::In),
                ("loop", TokenType::Loop),
                ("pool", TokenType::Pool),
                ("let", TokenType::Let),
                ("while", TokenType::While),
                ("then", TokenType::Then),
                ("case", TokenType::Case),
                ("esac", TokenType::Esac),
                ("new", TokenType::New),
                ("of", TokenType::Of),
                ("not", TokenType::Not),
                ("class", TokenType::Class),
                ("isvoid", TokenType::IsVoid),
                ("inherits", TokenType::Inherits),
                
                // Built-in types
                ("Int", TokenType::Type),
                ("String", TokenType::Type),
                ("Bool", TokenType::Type)
            ]
            .into_iter()
            .collect(),
        }
    }
}

impl Scanner<'_> {
    pub fn scan_tokens(&mut self, input: String) {
       self.source = input.into_bytes();

       while !self.is_at_end() {
           self.start = self.current;
           self.scan_token(); 
       } 
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
            ')' => self.add_token(TokenType::Rparen),
            '{' => self.add_token(TokenType::Lbracket),
            '}' => self.add_token(TokenType::Rbracket),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            ';' => self.add_token(TokenType::Semicolon),
            ':' => self.add_token(TokenType::Colon),
            '~' => self.add_token(TokenType::Neg),
            '+' => self.add_token(TokenType::Add),
            '/' => self.add_token(TokenType::Div),
            '@' => self.add_token(TokenType::At),

            // Multi-character operators
            '<' => {
                if self.match_next('=') {
                    self.add_token(TokenType::Lteq);
                } else if self.match_next('-') {
                    self.add_token(TokenType::Assign);
                } else {
                    self.add_token(TokenType::Lt);
                }
            },
            '>' => {
                if self.match_next('=') {
                    self.add_token(TokenType::Gteq);
                } else {
                    self.add_token(TokenType::Gt)
                };
            },
            '=' => {
                if self.match_next('>') {
                    self.add_token(TokenType::CaseAssign);
                } else {
                    self.add_token(TokenType::Eq);
                }
            },
            '(' => {
                if self.match_next('*') {
                    self.add_token(TokenType::OpenMc);
                } else {
                    self.add_token(TokenType::Lparen);
                }
            },
            '*' => {
                if self.match_next(')') {
                    self.add_token(TokenType::CloseMc);
                } else {
                    self.add_token(TokenType::Mul);
                }
            },
            '-' => {
                if self.match_next('-') {
                    // Ignore single-lined comments
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Sub);
                }
            },

            // Ingnore whitespace
            ' ' | '\r' | '\t' => (),
            '\n' => {
                self.line += 1;
            },
            
            // String literals
            '"' => {
                self.handle_string();
            },

            c => {
                if c.is_ascii_digit() {
                    self.handle_number();
                } else if self.is_alpha(c) {
                    self.handle_identifier();
                } else {
                    println!(
                        "Error: scanner cannot handle {}. Written at line {}",
                        c,
                        self.line
                   );
                }
            }
        }
    }

    fn add_token(&mut self, ty: TokenType) {
        self.add_token_literal(ty, None);
    }

    fn add_token_literal(&mut self, ty: TokenType, literal: Option<Literal>) {
        let text: Vec<u8> = self.source[self.start..self.current].to_vec();

        self.tokens.push(Token {
           ty,
           lexeme: text,
           literal,
           line: self.line, 
        });
    }
    
    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false
        }
    
        let c: char = char::from(self.source[self.current]);
        if c == expected {
            self.current += 1;
            return true
        }

        false
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        char::from(self.source[self.current - 1])
    }

    ///# Description
    /// Retrieves the next character without
    /// consuming it.
    /// 
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            char::from(self.source[self.current])
        }
    }

    ///# Description
    /// Build the string contained between quotation marks
    /// and add it's value to a Literal::Str
    ///  
    fn handle_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            println!("Error: unterminated string");
            self.add_token_literal(TokenType::Error, None);
            return;
        }

        self.advance();

        let mut substring: String = String::new();
        for &val in &self.source[self.start + 1..self.current - 1] {
            substring.push(char::from(val));
        }

        println!("Added {} as literal", substring);

        self.add_token_literal(TokenType::String, Some(Literal::Str(substring)));
    }

    ///# Description
    /// Fetches the String containing the numeric
    /// literal and adds it as a new i32 token.
    /// 
    fn handle_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        let mut val_str: String = String::new();
        for &c in &self.source[self.start..self.current]{
            val_str.push(char::from(c))
        }

        let value:i32 = val_str.parse().expect("Failed to parse string");
        self.add_token_literal(TokenType::Number, Some(Literal::Number(value)));
    }

    fn handle_identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text: String = self.source[self.start..self.current]
        .iter()
        .map(|&c| c as char)
        .collect();

        let ty = if let Some(ty) = self.keywords.get(text.as_str()) {
            ty.clone()
        } else {
            TokenType::Identifier
        };

        self.add_token(ty);
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || c.is_ascii_digit()
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_lowercase() ||
        ('A'..='z').contains(&c) ||
        c == '_'
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

#[derive(Clone)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: Vec<u8>,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token {{ ty: {:?}, lexeme: \"{}\", literal: {:?}, line: {:?}}}",
            self.ty,
            String::from_utf8(self.lexeme.clone()).unwrap(),
            self.literal,
            self.line
        )
    }
}

//================================================================================
//  Enums
//================================================================================

#[derive(Debug, Clone)]
pub enum TokenType {
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
}

impl PartialEq for TokenType{
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Str(String),
    Number(i32)
}

//================================================================================
//  Standalone Functions
//================================================================================

///# Description
/// Reads input string classifying each token.
pub fn scan_tokens(input: &str) {
    let mut scanner: Scanner = Scanner::default();
    scanner.scan_tokens(input.to_string());
    println!("{:#?}", scanner.tokens);
}

//================================================================================
//  Tests
//================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to scan input and return token types
    fn scan_and_get_tokens(input: &str) -> Vec<TokenType> {
        let mut scanner = Scanner::default();
        scanner.scan_tokens(input.to_string());
        scanner.tokens.iter().map(|token| token.ty.clone()).collect()
    }

    #[test]
    fn test_keywords() {
        let input = "if else while loop";
        let tokens = scan_and_get_tokens(input);
        let expected = vec![TokenType::If, TokenType::Else, TokenType::While, TokenType::Loop];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_operators() {
        let input = "+ - * / < <= > >= = <-";
        let tokens = scan_and_get_tokens(input);
        let expected = vec![
            TokenType::Add,
            TokenType::Sub,
            TokenType::Mul,
            TokenType::Div,
            TokenType::Lt,
            TokenType::Lteq,
            TokenType::Gt,
            TokenType::Gteq,
            TokenType::Eq,
            TokenType::Assign
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_identifiers() {
        let input = "foo bar my_variable";
        let tokens = scan_and_get_tokens(input);
        let expected = vec![TokenType::Identifier, TokenType::Identifier, TokenType::Identifier];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_types() {
        let input = "Int String Bool";
        let tokens = scan_and_get_tokens(input);
        let expected = vec![TokenType::Type, TokenType::Type, TokenType::Type];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_numbers() {
        let input = "42 123456";
        let tokens = scan_and_get_tokens(input);
        let expected = vec![TokenType::Number, TokenType::Number];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_strings() {
        let input = r#""hello" "world""#;
        let tokens = scan_and_get_tokens(input);
        let expected = vec![TokenType::String, TokenType::String];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_unterminated_string() {
        let input = r#""hello"#;
        let tokens = scan_and_get_tokens(input);
        let expected = vec![TokenType::Error];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_comments() {
        let input = "let x = 42; -- this is a comment\nlet y = 13;";
        let tokens = scan_and_get_tokens(input);
        let expected = vec![
            TokenType::Let, TokenType::Identifier, TokenType::Eq, TokenType::Number, TokenType::Semicolon,
            TokenType::Let, TokenType::Identifier, TokenType::Eq, TokenType::Number, TokenType::Semicolon
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_complex_case() {
        let input = r#"class Main inherits IO {
            let x: Int <- 42;
            let y: String <- "Hello";
        }"#;
        let tokens = scan_and_get_tokens(input);
        let expected = vec![
            TokenType::Class, TokenType::Identifier, TokenType::Inherits, TokenType::Identifier, TokenType::Lbracket,
            TokenType::Let, TokenType::Identifier, TokenType::Colon, TokenType::Type, TokenType::Assign, TokenType::Number, TokenType::Semicolon,
            TokenType::Let, TokenType::Identifier, TokenType::Colon, TokenType::Type, TokenType::Assign, TokenType::String, TokenType::Semicolon,
            TokenType::Rbracket
        ];
        assert_eq!(tokens, expected);
    }
}

