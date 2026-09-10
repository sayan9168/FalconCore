use crate::lexer::{Lexer, Token, TokenType};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(i64),
    String(String),
    Identifier(String),
    Binary { left: Box<Expr>, op: TokenType, right: Box<Expr> },
    Let { is_secure: bool, is_const: bool, name: String, value: Box<Expr> },
    Print { expr: Box<Expr> },
    If { condition: Box<Expr>, then_branch: Vec<Expr>, else_branch: Option<Vec<Expr>> },
    Repeat { times: Box<Expr>, body: Vec<Expr> },
    FnDef { name: String, params: Vec<String>, body: Vec<Expr> },
    Return { value: Option<Box<Expr>> },
    NetworkScan { subnet: Box<Expr> },
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Self {
            lexer,
            current_token: Token { kind: TokenType::Eof, line: 1, column: 1 },
        };
        parser.advance();
        parser
    }

    pub fn parse(&mut self) -> Vec<Expr> {
        let mut statements = Vec::new();
        while self.current_token.kind != TokenType::Eof {
            if self.current_token.kind == TokenType::Semi { self.advance(); continue; }
            statements.push(self.statement());
        }
        statements
    }

    fn statement(&mut self) -> Expr {
        match self.current_token.kind {
            TokenType::SecureLet => self.let_statement(false),
            TokenType::SecureConst => self.let_statement(true),
            TokenType::Print => self.print_statement(),
            TokenType::If => self.if_statement(),
            TokenType::Repeat => self.repeat_statement(),
            TokenType::Fn => self.fn_statement(),
            TokenType::Return => self.return_statement(),
            TokenType::NetworkScan => self.network_scan_statement(),
            _ => self.expression(),
        }
    }

    fn let_statement(&mut self, is_const: bool) -> Expr {
        self.advance();
        let name = self.expect_identifier("expected identifier after secure let/const");
        self.expect(TokenType::Assign);
        let value = self.expression();
        Expr::Let { is_secure: true, is_const, name, value: Box::new(value) }
    }

    fn print_statement(&mut self) -> Expr {
        self.expect(TokenType::Print);
        Expr::Print { expr: Box::new(self.expression()) }
    }

    fn if_statement(&mut self) -> Expr {
        self.expect(TokenType::If);
        let condition = self.expression();
        self.expect(TokenType::LBrace);
        let then_branch = self.block();
        let else_branch = if self.current_token.kind == TokenType::Else {
            self.advance();
            self.expect(TokenType::LBrace);
            Some(self.block())
        } else { None };
        Expr::If { condition: Box::new(condition), then_branch, else_branch }
    }

    fn repeat_statement(&mut self) -> Expr {
        self.expect(TokenType::Repeat);
        let times = self.expression();
        self.expect(TokenType::LBrace);
        let body = self.block();
        Expr::Repeat { times: Box::new(times), body }
    }

    fn fn_statement(&mut self) -> Expr {
        self.expect(TokenType::Fn);
        let name = self.expect_identifier("expected function name");
        self.expect(TokenType::LParen);
        let mut params = Vec::new();
        while self.current_token.kind != TokenType::RParen {
            params.push(self.expect_identifier("expected parameter name"));
            if self.current_token.kind == TokenType::Comma { self.advance(); } else { break; }
        }
        self.expect(TokenType::RParen);
        self.expect(TokenType::LBrace);
        let body = self.block();
        Expr::FnDef { name, params, body }
    }

    fn return_statement(&mut self) -> Expr {
        self.expect(TokenType::Return);
        let value = match self.current_token.kind {
            TokenType::Semi | TokenType::RBrace | TokenType::Eof => None,
            _ => Some(Box::new(self.expression())),
        };
        Expr::Return { value }
    }

    fn network_scan_statement(&mut self) -> Expr {
        self.expect(TokenType::NetworkScan);
        Expr::NetworkScan { subnet: Box::new(self.expression()) }
    }

    fn block(&mut self) -> Vec<Expr> {
        let mut body = Vec::new();
        while self.current_token.kind != TokenType::RBrace && self.current_token.kind != TokenType::Eof {
            if self.current_token.kind == TokenType::Semi { self.advance(); continue; }
            body.push(self.statement());
        }
        self.expect(TokenType::RBrace);
        body
    }

    fn expression(&mut self) -> Expr { self.comparison() }

    fn comparison(&mut self) -> Expr {
        let mut left = self.term();
        while matches!(self.current_token.kind, TokenType::EqualEqual | TokenType::NotEqual | TokenType::Greater | TokenType::Less | TokenType::GreaterEqual | TokenType::LessEqual) {
            let op = self.current_token.kind.clone();
            self.advance();
            let right = self.term();
            left = Expr::Binary { left: Box::new(left), op, right: Box::new(right) };
        }
        left
    }

    fn term(&mut self) -> Expr {
        let mut left = self.factor();
        while matches!(self.current_token.kind, TokenType::Plus | TokenType::Minus) {
            let op = self.current_token.kind.clone(); self.advance();
            let right = self.factor();
            left = Expr::Binary { left: Box::new(left), op, right: Box::new(right) };
        }
        left
    }

    fn factor(&mut self) -> Expr {
        let mut left = self.primary();
        while matches!(self.current_token.kind, TokenType::Star | TokenType::Slash) {
            let op = self.current_token.kind.clone(); self.advance();
            let right = self.primary();
            left = Expr::Binary { left: Box::new(left), op, right: Box::new(right) };
        }
        left
    }

    fn primary(&mut self) -> Expr {
        match self.current_token.kind.clone() {
            TokenType::Number(n) => { self.advance(); Expr::Number(n) }
            TokenType::String(s) => { self.advance(); Expr::String(s) }
            TokenType::Identifier(name) => { self.advance(); Expr::Identifier(name) }
            TokenType::LParen => { self.advance(); let value = self.expression(); self.expect(TokenType::RParen); value }
            ref other => panic!("unexpected token in expression: {:?}", other),
        }
    }

    fn expect_identifier(&mut self, message: &str) -> String {
        match self.current_token.kind.clone() {
            TokenType::Identifier(name) => { self.advance(); name }
            _ => panic!("{} at {}:{}", message, self.current_token.line, self.current_token.column),
        }
    }

    fn expect(&mut self, expected: TokenType) {
        if self.current_token.kind != expected {
            panic!("expected {:?}, found {:?} at {}:{}", expected, self.current_token.kind, self.current_token.line, self.current_token.column);
        }
        self.advance();
    }

    fn advance(&mut self) { self.current_token = self.lexer.next_token(); }
}
