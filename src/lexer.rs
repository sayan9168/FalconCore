// FalconCore lexer
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    SecureLet, SecureConst, Fn, Return, If, ElseIf, Else, EndIf, Repeat, EndRepeat,
    Break, Continue, Print, Import, Export, As,
    NetworkScan, CryptoRandom, TimeNow, Wait,
    Identifier(String), String(String), Number(i64), Float(f64),
    Plus, Minus, Star, Slash, EqualEqual, NotEqual, Greater, Less, GreaterEqual, LessEqual, Assign,
    LParen, RParen, LBrace, RBrace, LBracket, RBracket, Comma, Colon, ColonColon, Semi, Eof,
}

#[derive(Debug, Clone)]
pub struct Token { pub kind: TokenType, pub line: usize, pub column: usize }
pub struct Lexer<'a> { chars: Peekable<Chars<'a>>, line: usize, column: usize }

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self { Self { chars: input.chars().peekable(), line: 1, column: 1 } }
    fn advance(&mut self) -> Option<char> { self.column += 1; self.chars.next() }
    fn peek(&mut self) -> Option<&char> { self.chars.peek() }
    fn skip_whitespace(&mut self) { while let Some(c)=self.peek() { if c.is_whitespace() { if *c=='\n' { self.line+=1; self.column=1; } self.advance(); } else { break; } } }
    fn read_identifier(&mut self, first: char) -> TokenType {
        let mut ident=first.to_string();
        while let Some(c)=self.peek() { if c.is_alphanumeric() || *c=='_' { ident.push(self.advance().unwrap()); } else { break; } }
        match ident.to_lowercase().as_str() {
            "secure" => { self.skip_whitespace(); match self.read_next_word().as_str() { "let"=>TokenType::SecureLet,"const"=>TokenType::SecureConst,_=>TokenType::Identifier(ident) } }
            "fn"=>TokenType::Fn,"return"=>TokenType::Return,"if"=>TokenType::If,"elseif"=>TokenType::ElseIf,"else"=>TokenType::Else,"endif"=>TokenType::EndIf,
            "repeat"=>TokenType::Repeat,"endrepeat"=>TokenType::EndRepeat,"break"=>TokenType::Break,"continue"=>TokenType::Continue,"print"=>TokenType::Print,
            "import"=>TokenType::Import,"export"=>TokenType::Export,"as"=>TokenType::As,
            "network"=>self.builtin(TokenType::NetworkScan,"scan",ident),"crypto"=>self.builtin(TokenType::CryptoRandom,"random",ident),
            "time"=>self.builtin(TokenType::TimeNow,"now",ident),"wait"=>TokenType::Wait,_=>TokenType::Identifier(ident)
        }
    }
    fn builtin(&mut self, token: TokenType, word: &str, ident: String) -> TokenType { self.skip_whitespace(); if let Some('.')=self.peek() { self.advance(); if self.read_next_word()==word { return token; } } TokenType::Identifier(ident) }
    fn read_next_word(&mut self)->String { let mut word=String::new(); while let Some(c)=self.peek() { if c.is_alphanumeric() { word.push(self.advance().unwrap()); } else { break } } word }
    fn read_string(&mut self)->TokenType { let mut s=String::new(); while let Some(c)=self.advance() { if c=='"' { break } s.push(c); } TokenType::String(s) }
    fn read_number(&mut self, first: char)->TokenType { let mut n=first.to_string(); let mut f=false; while let Some(c)=self.peek() { if c.is_ascii_digit(){n.push(self.advance().unwrap())} else if *c=='.'&&!f{f=true;n.push(self.advance().unwrap())} else{break} } if f {TokenType::Float(n.parse().unwrap_or(0.0))} else {TokenType::Number(n.parse().unwrap_or(0))} }
    pub fn next_token(&mut self)->Token {
        self.skip_whitespace(); let line=self.line; let column=self.column;
        let kind=match self.advance() {
            Some(c)=>match c {
                '"'=>self.read_string(),'0'..='9'=>self.read_number(c),'a'..='z'|'A'..='Z'|'_'=>self.read_identifier(c),
                '+'=>TokenType::Plus,'-'=>TokenType::Minus,'*'=>TokenType::Star,'/'=>TokenType::Slash,
                '='=>if self.peek()==Some(&'='){self.advance();TokenType::EqualEqual}else{TokenType::Assign},
                '!'=>if self.peek()==Some(&'='){self.advance();TokenType::NotEqual}else{TokenType::Identifier("!".into())},
                '>'=>if self.peek()==Some(&'='){self.advance();TokenType::GreaterEqual}else{TokenType::Greater},
                '<'=>if self.peek()==Some(&'='){self.advance();TokenType::LessEqual}else{TokenType::Less},
                ':'=>if self.peek()==Some(&':'){self.advance();TokenType::ColonColon}else{TokenType::Colon},
                '('=>TokenType::LParen,')'=>TokenType::RParen,'{'=>TokenType::LBrace,'}'=>TokenType::RBrace,'['=>TokenType::LBracket,']'=>TokenType::RBracket,','=>TokenType::Comma,';'=>TokenType::Semi,
                _=>TokenType::Identifier(c.to_string())
            },
            None=>TokenType::Eof
        };
        Token{kind,line,column}
    }
}
