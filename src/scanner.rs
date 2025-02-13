use std::fmt;

pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
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
            line: 1
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
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        char::from(self.source[self.current - 1])
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

#[derive(Debug, Clone)]
pub enum TokenType {
    // Operators
    ADD,
    SUB,
    MUL,
    DIV,
    INVERSE,
    LT,
    LTEQ,
    GT,
    GTEQ,
    ASSIGN,
    CASEASSIGN,
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
    AT
}

#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String),
    Str(String),
    Number(u64)
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

