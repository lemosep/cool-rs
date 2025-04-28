use std::collections::HashMap;
use super::token::{Token, LexicalError, Loc};

pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<(Token, Loc)>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
    keywords: HashMap<&'static str, Token>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        let keywords = vec![
            ("class", Token::Class_),
            ("else", Token::Else),
            ("fi", Token::Fi),
            ("if", Token::If),
            ("in", Token::In),
            ("inherits", Token::Inherits),
            ("let", Token::Let),
            ("loop", Token::Loop),
            ("pool", Token::Pool),
            ("then", Token::Then),
            ("while", Token::While),
            ("case", Token::Case),
            ("esac", Token::Esac),
            ("of", Token::Of),
            ("new", Token::New),
            ("isvoid", Token::Isvoid),
            ("not", Token::Not),
            ("true", Token::BoolConst(true)),
            ("false", Token::BoolConst(false)),
        ]
        .into_iter()
        .collect();

        Scanner {
            source: source.as_bytes().to_vec(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 0,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<(Token, Loc)>, LexicalError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), LexicalError> {
        let c = self.advance();
        let loc = Loc {
            line: self.line,
            column: self.column,
        };

        match c {
            ';' => Ok(self.add_token(Token::Semicolon, loc)),
            ':' => Ok(self.add_token(Token::Colon, loc)),
            '{' => Ok(self.add_token(Token::Lbrace, loc)),
            '}' => Ok(self.add_token(Token::Rbrace, loc)),
            '(' => Ok(self.add_token(Token::Lparen, loc)),
            ')' => Ok(self.add_token(Token::Rparen, loc)),
            ',' => Ok(self.add_token(Token::Comma, loc)),
            '.' => Ok(self.add_token(Token::Period, loc)),
            '@' => Ok(self.add_token(Token::At, loc)),
            '~' => Ok(self.add_token(Token::Neg, loc)),
            '=' => {
                if self.match_next('>') {
                    Ok(self.add_token(Token::Darrow, loc))
                } else {
                    Ok(self.add_token(Token::Equal, loc))
                }
            }
            '+' => Ok(self.add_token(Token::Plus, loc)),
            '*' => Ok(self.add_token(Token::Mul, loc)),
            '/' => Ok(self.add_token(Token::Divide, loc)),
            '<' => {
                if self.match_next('=') {
                    Ok(self.add_token(Token::Le, loc))
                } else if self.match_next('-') {
                    Ok(self.add_token(Token::Assign, loc))
                } else {
                    Ok(self.add_token(Token::Lt, loc))
                }
            }
            '"' => self.handle_string(loc),
            ' ' | '\r' | '\t' => Ok(()),
            '\n' => {
                self.line += 1;
                self.column = 0;
                Ok(())
            }
            '-' => {
                if self.match_next('-') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(())
                } else {
                    Ok(self.add_token(Token::Minus, loc))
                }
            }
            c if c.is_ascii_digit() => self.handle_number(loc),
            c if is_alpha(c) => self.handle_identifier(loc),
            c => Err(LexicalError::InvalidChar(c, loc)),
        }
    }

    fn handle_string(&mut self, loc: Loc) -> Result<(), LexicalError> {
        let mut value = String::new();
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 0;
            }
            let c = self.advance();
            value.push(c);
        }
        if self.is_at_end() {
            return Err(LexicalError::UnterminatedString(loc));
        }
        self.advance(); // Consume closing quote
        self.add_token(Token::StrConst(value), loc);
        Ok(())
    }

    fn handle_number(&mut self, loc: Loc) -> Result<(), LexicalError> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        let value: String = self.source[self.start..self.current]
            .iter()
            .map(|&c| c as char)
            .collect();
        if value.parse::<i32>().is_ok() {
            self.add_token(Token::IntConst(value), loc);
            Ok(())
        } else {
            Err(LexicalError::InvalidNumber(value, loc))
        }
    }

    fn handle_identifier(&mut self, loc: Loc) -> Result<(), LexicalError> {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }
        let text: String = self.source[self.start..self.current]
            .iter()
            .map(|&c| c as char)
            .collect();
        if let Some(token) = self.keywords.get(text.to_lowercase().as_str()) {
            self.add_token(token.clone(), loc);
        } else if text.chars().next().unwrap().is_uppercase() {
            self.add_token(Token::Typeid(text), loc);
        } else {
            self.add_token(Token::Objectid(text), loc);
        }
        Ok(())
    }

    fn add_token(&mut self, token: Token, loc: Loc) {
        self.tokens.push((token, loc));
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.column += 1;
        char::from(self.source[self.current - 1])
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            char::from(self.source[self.current])
        }
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() || char::from(self.source[self.current]) != expected {
            false
        } else {
            self.current += 1;
            self.column += 1;
            true
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_lowercase() || c.is_ascii_uppercase() || c == '_'
}

fn is_alphanumeric(c: char) -> bool {
    is_alpha(c) || c.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords() {
        let input = "class if while let";
        let mut scanner = Scanner::new(input);
        let tokens = scanner.scan_tokens().unwrap();
        let loc = Loc { line: 1, column: 0 };
        let expected = vec![
            (Token::Class_, loc.clone()),
            (Token::If, loc.clone()),
            (Token::While, loc.clone()),
            (Token::Let, loc),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_operators() {
        let input = "+ - * / <- =>";
        let mut scanner = Scanner::new(input);
        let tokens = scanner.scan_tokens().unwrap();
        let loc = Loc { line: 1, column: 0 };
        let expected = vec![
            (Token::Plus, loc.clone()),
            (Token::Minus, loc.clone()),
            (Token::Mul, loc.clone()),
            (Token::Divide, loc.clone()),
            (Token::Assign, loc.clone()),
            (Token::Darrow, loc),
        ];
        assert_eq!(tokens, expected);
    }
}
