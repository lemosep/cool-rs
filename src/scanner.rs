//================================================================================
//  Imports
//================================================================================

use std::{collections::HashMap, fmt};

//================================================================================
//  Structs
//================================================================================

pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    keywords: HashMap<String, TokenType>,
    start: usize,
    current: usize,
    line: usize
}

impl Default for Scanner {
    fn default() -> Self {
        Scanner {
            source: Vec::new(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: vec![
                ("if", TokenType::IF),
                ("fi", TokenType::FI),
                ("else", TokenType::ELSE),
                ("true", TokenType::TRUE),
                ("false", TokenType::FALSE),
                ("in", TokenType::IN),
                ("loop", TokenType::LOOP),
                ("pool", TokenType::POOL),
                ("let", TokenType::LET),
                ("while", TokenType::WHILE),
                ("then", TokenType::THEN),
                ("case", TokenType::CASE),
                ("esac", TokenType::ESAC),
                ("new", TokenType::NEW),
                ("of", TokenType::OF),
                ("not", TokenType::NOT),
                ("class", TokenType::CLASS),
                ("isvoid", TokenType::ISVOID),
                ("inherits", TokenType::INHERITS),
            ]
            .into_iter()
            .map(|(k,v)| (String::from(k), v))
            .collect(),
        }
    }
}

impl Scanner {
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
            ')' => self.add_token(TokenType::RPAREN),
            '{' => self.add_token(TokenType::LBRACKET),
            '}' => self.add_token(TokenType::RBRACKET),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            ';' => self.add_token(TokenType::SEMICOLON),
            ':' => self.add_token(TokenType::COLON),
            '~' => self.add_token(TokenType::NEG),
            '+' => self.add_token(TokenType::ADD),
            '/' => self.add_token(TokenType::DIV),
            '@' => self.add_token(TokenType::AT),

            // Multi-character operators
            '<' => {
                if self.match_next('=') {
                    self.add_token(TokenType::LTEQ);
                } else if self.match_next('-') {
                    self.add_token(TokenType::ASSIGN);
                } else {
                    self.add_token(TokenType::LT);
                }
            },
            '>' => {
                if self.match_next('=') {
                    self.add_token(TokenType::GTEQ);
                } else {
                    self.add_token(TokenType::GT)
                };
            },
            '=' => {
                if self.match_next('>') {
                    self.add_token(TokenType::CASEASSIGN);
                } else {
                    self.add_token(TokenType::EQ);
                }
            },
            '(' => {
                if self.match_next('*') {
                    self.add_token(TokenType::OPENMC);
                } else {
                    self.add_token(TokenType::LPAREN);
                }
            },
            '*' => {
                if self.match_next(')') {
                    self.add_token(TokenType::CLOSEMC);
                } else {
                    self.add_token(TokenType::MUL);
                }
            },
            '-' => {
                if self.match_next('-') {
                    // Ignore single-lined comments
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SUB);
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
                if self.is_digit(c) {
                    println!("Dis shit a digit!!");
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

                println!("{:?}", self.tokens);

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
            return '\0';
        } else {
            return char::from(self.source[self.current]);
        }
    }

    ///# Description
    /// Build the string contained between quotation marks
    /// and add it's value to a `Literal::Str`
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
        }

        self.advance();

        let mut substring: String = String::new();
        for &val in &self.source[self.start + 1..self.current - 1] {
            substring.push(char::from(val));
        }

        println!("Added {} as literal", substring);

        self.add_token_literal(TokenType::STRING, Some(Literal::Str(substring)));
    }

    ///# Description
    /// Fetches the `String` containing the numeric
    /// literal and adds it as a new i32 token.
    /// 
    fn handle_number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        let mut val_str: String = String::new();
        for &c in &self.source[self.start..self.current]{
            val_str.push(char::from(c))
        }

        let value:i32 = val_str.parse().expect("Failed to parse string");
        self.add_token_literal(TokenType::NUMBER, Some(Literal::Number(value)));
    }

    fn handle_identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let mut text: String = String::new();
        for &c in &self.source[self.start..self.current] {
            text.push(char::from(c));
        }

        let mut ty: Option<TokenType> = self.keywords.get(&text).clone().cloned();

        match ty {
            Some(_) => (),
            None => ty = Some(TokenType::IDENTIFIER),
        }

        self.add_token(ty.unwrap());
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') ||
        (c >= 'A' && c <= 'Z') ||
        c == '_'
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
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
    ADD,
    SUB,
    MUL,
    DIV,
    NEG,
    LT,
    LTEQ,
    GT,
    GTEQ,
    EQ,
    
    // Keywords,
    IF,
    FI,
    ELSE,
    TRUE,
    FALSE,
    IN,
    LOOP,
    POOL,
    LET,
    WHILE,
    THEN,
    CASE,
    ESAC,
    NEW,
    OF,
    NOT,
    CLASS,
    ISVOID,
    INHERITS,

    // Literals
    IDENTIFIER,
    STRING,
    NUMBER,

    // Error
    ERROR,

    // Misc
    COLON,
    SEMICOLON,
    COMMA,
    DOT,
    LPAREN,
    RPAREN,
    LBRACKET,
    RBRACKET,
    AT,
    ASSIGN,
    CASEASSIGN,
    OPENMC,
    CLOSEMC
}

#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String),
    Str(String),
    Number(i32)
}

//================================================================================
//  Standalone Functions
//================================================================================

///# Description
/// Reads `input` string classifying each token.
pub fn scan_tokens(input: String) {
    let mut scanner: Scanner = Scanner::default();
    scanner.scan_tokens(input.clone());
}
