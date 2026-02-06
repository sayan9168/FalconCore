// src/lexer.rs - FalconCore Lexer (Enhanced with more tokens)
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords
    SecureLet,
    SecureConst,
    Fn,
    Return,
    If,
    ElseIf,
    Else,
    EndIf,
    Repeat,
    EndRepeat,
    Break,
    Continue,
    Print,

    // Built-in commands
    NetworkScan,
    CryptoRandom,
    TimeNow,
    Wait,

    // Literals
    Identifier(String),
    String(String),
    Number(i64),
    Float(f64),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    EqualEqual,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Assign,

    // Punctuation
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Colon,
    Semi,

    // End of file
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable(),
            line: 1,
            column: 1,
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.column += 1;
        self.chars.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                if *c == '\n' {
                    self.line += 1;
                    self.column = 1;
                }
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self, first: char) -> TokenType {
        let mut ident = first.to_string();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || *c == '_' {
                ident.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        match ident.as_str() {
            "secure" => {
                self.skip_whitespace();
                if let Some('l') = self.peek() {
                    let mut next = self.advance().unwrap().to_string();
                    if let Some('e') = self.peek() {
                        next.push(self.advance().unwrap());
                        if let Some('t') = self.peek() {
                            next.push(self.advance().unwrap());
                            if next == "let" {
                                return TokenType::SecureLet;
                            }
                        }
                    }
                }
                if let Some('c') = self.peek() {
                    let mut next = self.advance().unwrap().to_string();
                    if let Some('o') = self.peek() {
                        next.push(self.advance().unwrap());
                        if let Some('n') = self.peek() {
                            next.push(self.advance().unwrap());
                            if let Some('s') = self.peek() {
                                next.push(self.advance().unwrap());
                                if let Some('t') = self.peek() {
                                    next.push(self.advance().unwrap());
                                    if next == "const" {
                                        return TokenType::SecureConst;
                                    }
                                }
                            }
                        }
                    }
                }
                TokenType::Identifier(ident)
            }
            "fn" => TokenType::Fn,
            "return" => TokenType::Return,
            "if" => TokenType::If,
            "elseif" => TokenType::ElseIf,
            "else" => TokenType::Else,
            "endif" => TokenType::EndIf,
            "repeat" => TokenType::Repeat,
            "endrepeat" => TokenType::EndRepeat,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "print" => TokenType::Print,
            "network" => {
                self.skip_whitespace();
                if let Some('.') = self.peek() {
                    self.advance();
                    if let Some('s') = self.peek() {
                        let mut next = self.advance().unwrap().to_string();
                        if let Some('c') = self.peek() {
                            next.push(self.advance().unwrap());
                            if let Some('a') = self.peek() {
                                next.push(self.advance().unwrap());
                                if let Some('n') = self.peek() {
                                    next.push(self.advance().unwrap());
                                    if next == "scan" {
                                        return TokenType::NetworkScan;
                                    }
                                }
                            }
                        }
                    }
                }
                TokenType::Identifier(ident)
            }
            "crypto" => {
                self.skip_whitespace();
                if let Some('.') = self.peek() {
                    self.advance();
                    if let Some('r') = self.peek() {
                        let mut next = self.advance().unwrap().to_string();
                        if let Some('a') = self.peek() {
                            next.push(self.advance().unwrap());
                            if let Some('n') = self.peek() {
                                next.push(self.advance().unwrap());
                                if let Some('d') = self.peek() {
                                    next.push(self.advance().unwrap());
                                    if let Some('o') = self.peek() {
                                        next.push(self.advance().unwrap());
                                        if let Some('m') = self.peek() {
                                            next.push(self.advance().unwrap());
                                            if next == "random" {
                                                return TokenType::CryptoRandom;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                TokenType::Identifier(ident)
            }
            "time" => {
                self.skip_whitespace();
                if let Some('.') = self.peek() {
                    self.advance();
                    if let Some('n') = self.peek() {
                        let mut next = self.advance().unwrap().to_string();
                        if let Some('o') = self.peek() {
                            next.push(self.advance().unwrap());
                            if let Some('w') = self.peek() {
                                next.push(self.advance().unwrap());
                                if next == "now" {
                                    return TokenType::TimeNow;
                                }
                            }
                        }
                    }
                }
                TokenType::Identifier(ident)
            }
            "wait" => TokenType::Wait,
            _ => TokenType::Identifier(ident),
        }
    }

    fn read_string(&mut self) -> TokenType {
        let mut s = String::new();
        while let Some(c) = self.advance() {
            if c == '"' {
                break;
            }
            s.push(c);
        }
        TokenType::String(s)
    }

    fn read_number(&mut self, first: char) -> TokenType {
        let mut num = first.to_string();
        let mut is_float = false;

        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                num.push(self.advance().unwrap());
            } else if *c == '.' && !is_float {
                is_float = true;
                num.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        if is_float {
            TokenType::Float(num.parse().unwrap_or(0.0))
        } else {
            TokenType::Number(num.parse().unwrap_or(0))
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let line = self.line;
        let column = self.column;

        if let Some(c) = self.advance() {
            match c {
                '"' => Token { kind: self.read_string(), line, column },
                '0'..='9' => Token { kind: self.read_number(c), line, column },
                'a'..='z' | 'A'..='Z' | '_' => Token { kind: self.read_identifier(c), line, column },

                '+' => Token { kind: TokenType::Plus, line, column },
                '-' => Token { kind: TokenType::Minus, line, column },
                '*' => Token { kind: TokenType::Star, line, column },
                '/' => Token { kind: TokenType::Slash, line, column },

                '=' => {
                    if let Some('=') = self.peek() {
                        self.advance();
                        Token { kind: TokenType::EqualEqual, line, column }
                    } else {
                        Token { kind: TokenType::Assign, line, column }
                    }
                }

                '!' => {
                    if let Some('=') = self.peek() {
                        self.advance();
                        Token { kind: TokenType::NotEqual, line, column }
                    } else {
                        Token { kind: TokenType::Identifier("!".to_string()), line, column }
                    }
                }

                '>' => {
                    if let Some('=') = self.peek() {
                        self.advance();
                        Token { kind: TokenType::GreaterEqual, line, column }
                    } else {
                        Token { kind: TokenType::Greater, line, column }
                    }
                }

                '<' => {
                    if let Some('=') = self.peek() {
                        self.advance();
                        Token { kind: TokenType::LessEqual, line, column }
                    } else {
                        Token { kind: TokenType::Less, line, column }
                    }
                }

                '(' => Token { kind: TokenType::LParen, line, column },
                ')' => Token { kind: TokenType::RParen, line, column },
                '{' => Token { kind: TokenType::LBrace, line, column },
                '}' => Token { kind: TokenType::RBrace, line, column },
                '[' => Token { kind: TokenType::LBracket, line, column },
                ']' => Token { kind: TokenType::RBracket, line, column },
                ',' => Token { kind: TokenType::Comma, line, column },
                ':' => Token { kind: TokenType::Colon, line, column },

                _ => Token { kind: TokenType::Identifier(c.to_string()), line, column },
            }
        } else {
            Token { kind: TokenType::Eof, line, column }
        }
    }
                           }
