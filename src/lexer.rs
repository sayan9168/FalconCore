// src/lexer.rs - FalconCore Lexer (Enhanced with network, crypto, time, wait)
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

    // read_string, read_number, next_token ফাংশনগুলো আগের মতোই রাখো
    // (আগের কোড থেকে কপি করে নিবি — শুধু read_identifier-এ নতুন টোকেন যোগ হয়েছে)
    // ...
                    }
