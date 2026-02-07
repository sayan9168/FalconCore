// src/parser.rs - FalconCore Parser (Enhanced with repeat, fn, return, network.scan)
use crate::lexer::{Lexer, Token, TokenType};

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    String(String),
    Identifier(String),
    Binary {
        left: Box<Expr>,
        op: TokenType,
        right: Box<Expr>,
    },
    Let {
        is_secure: bool,
        is_const: bool,
        name: String,
        value: Box<Expr>,
    },
    Print {
        expr: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Vec<Expr>,
        else_branch: Option<Vec<Expr>>,
    },
    Repeat {
        times: Box<Expr>,
        body: Vec<Expr>,
    },
    FnDef {
        name: String,
        params: Vec<String>,
        body: Vec<Expr>,
    },
    Return {
        value: Option<Box<Expr>>,
    },
    NetworkScan {
        subnet: Box<Expr>,
    },
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token { kind: TokenType::Eof, line: 1, column: 1 },
        };
        parser.current_token = parser.lexer.next_token();
        parser
    }

    fn eat(&mut self, expected: TokenType) {
        if self.current_token.kind == expected {
            self.current_token = self.lexer.next_token();
        } else {
            panic!("Expected {:?}, found {:?}", expected, self.current_token.kind);
        }
    }

    pub fn parse(&mut self) -> Vec<Expr> {
        let mut statements = vec![];

        while self.current_token.kind != TokenType::Eof {
            statements.push(self.statement());
        }

        statements
    }

    fn statement(&mut self) -> Expr {
        match self.current_token.kind {
            TokenType::SecureLet => self.let_statement(true, false),
            TokenType::SecureConst => self.let_statement(true, true),
            TokenType::Print => self.print_statement(),
            TokenType::If => self.if_statement(),
            TokenType::Repeat => self.repeat_statement(),
            TokenType::Fn => self.fn_statement(),
            TokenType::Return => self.return_statement(),
            TokenType::NetworkScan => self.network_scan_statement(),
            _ => self.expr(),
        }
    }

    fn let_statement(&mut self, is_secure: bool, is_const: bool) -> Expr {
        if is_const {
            self.eat(TokenType::SecureConst);
        } else {
            self.eat(TokenType::SecureLet);
        }

        let name = if let TokenType::Identifier(n) = self.current_token.kind.clone() {
            self.eat(TokenType::Identifier(n.clone()));
            n
        } else {
            panic!("Expected identifier after secure let/const");
        };

        self.eat(TokenType::Assign);
        let value = self.expr();

        Expr::Let {
            is_secure,
            is_const,
            name,
            value: Box::new(value),
        }
    }

    fn print_statement(&mut self) -> Expr {
        self.eat(TokenType::Print);
        let expr = self.expr();
        Expr::Print {
            expr: Box::new(expr),
        }
    }

    fn if_statement(&mut self) -> Expr {
        self.eat(TokenType::If);
        let condition = self.expr();

        self.eat(TokenType::LBrace);
        let mut then_branch = vec![];
        while self.current_token.kind != TokenType::RBrace {
            then_branch.push(self.statement());
        }
        self.eat(TokenType::RBrace);

        let else_branch = if self.current_token.kind == TokenType::Else {
            self.eat(TokenType::Else);
            self.eat(TokenType::LBrace);
            let mut else_stmts = vec![];
            while self.current_token.kind != TokenType::RBrace {
                else_stmts.push(self.statement());
            }
            self.eat(TokenType::RBrace);
            Some(else_stmts)
        } else {
            None
        };

        Expr::If {
            condition: Box::new(condition),
            then_branch,
            else_branch,
        }
    }

    fn repeat_statement(&mut self) -> Expr {
        self.eat(TokenType::Repeat);
        let times = self.expr();

        self.eat(TokenType::LBrace);
        let mut body = vec![];
        while self.current_token.kind != TokenType::RBrace {
            body.push(self.statement());
        }
        self.eat(TokenType::RBrace);

        Expr::Repeat {
            times: Box::new(times),
            body,
        }
    }

    fn fn_statement(&mut self) -> Expr {
        self.eat(TokenType::Fn);
        let name = if let TokenType::Identifier(n) = self.current_token.kind.clone() {
            self.eat(TokenType::Identifier(n.clone()));
            n
        } else {
            panic!("Expected function name");
        };

        self.eat(TokenType::LParen);
        let mut params = vec![];
        while self.current_token.kind != TokenType::RParen {
            if let TokenType::Identifier(p) = self.current_token.kind.clone() {
                params.push(p);
                self.eat(TokenType::Identifier(p));
            }
            if self.current_token.kind == TokenType::Comma {
                self.eat(TokenType::Comma);
            }
        }
        self.eat(TokenType::RParen);

        self.eat(TokenType::LBrace);
        let mut body = vec![];
        while self.current_token.kind != TokenType::RBrace {
            body.push(self.statement());
        }
        self.eat(TokenType::RBrace);

        Expr::FnDef {
            name,
            params,
            body,
        }
    }

    fn return_statement(&mut self) -> Expr {
        self.eat(TokenType::Return);
        let value = if self.current_token.kind != TokenType::Semi && self.current_token.kind != TokenType::RBrace {
            Some(Box::new(self.expr()))
        } else {
            None
        };
        Expr::Return { value }
    }

    fn network_scan_statement(&mut self) -> Expr {
        self.eat(TokenType::NetworkScan);
        let subnet = self.expr();
        Expr::NetworkScan {
            subnet: Box::new(subnet),
        }
    }

    fn expr(&mut self) -> Expr {
        self.term()
    }

    fn term(&mut self) -> Expr {
        let mut left = self.factor();

        while matches!(self.current_token.kind, TokenType::Plus | TokenType::Minus) {
            let op = self.current_token.kind.clone();
            self.advance();
            let right = self.factor();
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    fn factor(&mut self) -> Expr {
        match self.current_token.kind {
            TokenType::Number(n) => {
                self.advance();
                Expr::Number(n)
            }
            TokenType::String(s) => {
                self.advance();
                Expr::String(s)
            }
            TokenType::Identifier(id) => {
                self.advance();
                Expr::Identifier(id)
            }
            _ => panic!("Unexpected token in factor: {:?}", self.current_token.kind),
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }
                 }
