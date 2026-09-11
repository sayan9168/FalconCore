use crate::lexer::TokenType;
use crate::parser::Expr;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Opcode { LoadConst(usize), LoadVar(String), StoreVar(String), Add, Sub, Mul, Div, Equal, NotEqual, Greater, Less, GreaterEqual, LessEqual, Print, JumpIfFalse(usize), Jump(usize), RepeatStart, RepeatEnd(usize), Call(String, usize), Return, NetworkScan, Halt }
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionInfo { pub entry: usize, pub params: Vec<String> }
#[derive(Debug, Default)]
pub struct Compiler { constants: Vec<Expr>, code: Vec<Opcode>, functions: HashMap<String, FunctionInfo> }
impl Compiler {
    pub fn new()->Self{Self::default()}
    pub fn compile(&mut self,ast:Vec<Expr>){for expr in &ast{if let Expr::FnDef{name,params,..}=expr{self.functions.insert(name.clone(),FunctionInfo{entry:usize::MAX,params:params.clone()});}}for expr in &ast{self.compile_expr(expr);}self.code.push(Opcode::Halt);}
    fn compile_expr(&mut self,expr:&Expr){match expr{
        Expr::Number(n)=>self.load_constant(Expr::Number(*n)),Expr::Float(n)=>self.load_constant(Expr::Float(*n)),Expr::String(s)=>self.load_constant(Expr::String(s.clone())),Expr::Identifier(n)=>self.code.push(Opcode::LoadVar(n.clone())),
        Expr::Binary{left,op,right}=>{self.compile_expr(left);self.compile_expr(right);let o=match op{TokenType::Plus=>Opcode::Add,TokenType::Minus=>Opcode::Sub,TokenType::Star=>Opcode::Mul,TokenType::Slash=>Opcode::Div,TokenType::EqualEqual=>Opcode::Equal,TokenType::NotEqual=>Opcode::NotEqual,TokenType::Greater=>Opcode::Greater,TokenType::Less=>Opcode::Less,TokenType::GreaterEqual=>Opcode::GreaterEqual,TokenType::LessEqual=>Opcode::LessEqual,_=>panic!("unsupported binary operator: {:?}",op)};self.code.push(o)},
        Expr::Let{name,value,..}=>{self.compile_expr(value);self.code.push(Opcode::StoreVar(name.clone()));},Expr::Print{expr}=>{self.compile_expr(expr);self.code.push(Opcode::Print)},
        Expr::If{condition,then_branch,else_branch}=>{self.compile_expr(condition);let jf=self.code.len();self.code.push(Opcode::JumpIfFalse(usize::MAX));for s in then_branch{self.compile_expr(s)}let je=self.code.len();self.code.push(Opcode::Jump(usize::MAX));let es=self.code.len();if let Some(b)=else_branch{for s in b{self.compile_expr(s)}}let end=self.code.len();self.code[jf]=Opcode::JumpIfFalse(es);self.code[je]=Opcode::Jump(end)},
        Expr::Repeat{times,body}=>{self.compile_expr(times);let start=self.code.len();self.code.push(Opcode::RepeatStart);for s in body{self.compile_expr(s)}self.code.push(Opcode::RepeatEnd(start))},
        Expr::FnDef{name,params,body}=>{let skip=self.code.len();self.code.push(Opcode::Jump(usize::MAX));let entry=self.code.len();self.functions.insert(name.clone(),FunctionInfo{entry,params:params.clone()});for s in body{self.compile_expr(s)}if !matches!(self.code.last(),Some(Opcode::Return)){self.code.push(Opcode::Return)}let after=self.code.len();self.code[skip]=Opcode::Jump(after)},
        Expr::Call{name,args}=>{for a in args{self.compile_expr(a)}self.code.push(Opcode::Call(name.clone(),args.len()))},Expr::Return{value}=>{if let Some(v)=value{self.compile_expr(v)}else{self.load_constant(Expr::Number(0))}self.code.push(Opcode::Return)},Expr::NetworkScan{subnet}=>{self.compile_expr(subnet);self.code.push(Opcode::NetworkScan)},
        Expr::Import{..}=>{},Expr::Export{item}=>self.compile_expr(item),
    }}
    fn load_constant(&mut self,v:Expr){let i=self.add_constant(v);self.code.push(Opcode::LoadConst(i))}fn add_constant(&mut self,v:Expr)->usize{let i=self.constants.len();self.constants.push(v);i}
    pub fn get_code(&self)->&[Opcode]{&self.code}pub fn get_constants(&self)->&[Expr]{&self.constants}pub fn get_functions(&self)->&HashMap<String,FunctionInfo>{&self.functions}
}
