use crate::parser::Expr;
use crate::lexer::TokenType;
use std::collections::HashMap;
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Type{Int,Float,String,Bool,Unknown}
#[derive(Debug,Clone,PartialEq,Eq)]pub struct TypeError{pub message:String}
#[derive(Debug,Default)]pub struct TypeChecker{variables:HashMap<String,Type>,functions:HashMap<String,(Vec<Type>,Type)>,errors:Vec<TypeError>}
impl TypeChecker{
 pub fn new()->Self{Self::default()}
 pub fn check(&mut self,p:&[Expr])->Result<(),Vec<TypeError>>{for e in p{if let Expr::FnDef{name,params,..}=e{self.functions.insert(name.clone(),(vec![Type::Unknown;params.len()],Type::Unknown));}}for e in p{self.check_expr(e);}if self.errors.is_empty(){Ok(())}else{Err(std::mem::take(&mut self.errors))}}
 fn error(&mut self,m:impl Into<String>){self.errors.push(TypeError{message:m.into()})}
 fn check_expr(&mut self,e:&Expr)->Type{match e{
  Expr::Number(_)=>Type::Int,Expr::Float(_)=>Type::Float,Expr::String(_)=>Type::String,
  Expr::Identifier(n)=>self.variables.get(n).copied().unwrap_or_else(||{self.error(format!("undefined variable: {n}"));Type::Unknown}),
  Expr::Let{name,value,..}=>{let t=self.check_expr(value);self.variables.insert(name.clone(),t);t},Expr::Print{expr}=>{self.check_expr(expr);Type::Unknown},
  Expr::If{condition,then_branch,else_branch}=>{let t=self.check_expr(condition);if t!=Type::Bool&&t!=Type::Unknown{self.error("if condition must be boolean")}for s in then_branch{self.check_expr(s)}if let Some(b)=else_branch{for s in b{self.check_expr(s)}}Type::Unknown},
  Expr::Repeat{times,body}=>{let t=self.check_expr(times);if t!=Type::Int&&t!=Type::Unknown{self.error("repeat count must be an integer")}for s in body{self.check_expr(s)}Type::Unknown},
  Expr::Binary{left,op,right}=>{let l=self.check_expr(left);let r=self.check_expr(right);self.check_binary(l,op,r)},
  Expr::FnDef{body,..}=>{for s in body{self.check_expr(s)}Type::Unknown},
  Expr::Call{name,args}=>{let ts:Vec<_>=args.iter().map(|a|self.check_expr(a)).collect();if let Some((ps,_))=self.functions.get(name).cloned(){if ps.len()!=ts.len(){self.error(format!("function {name} expects {} arguments, got {}",ps.len(),ts.len()))}}else{self.error(format!("undefined function: {name}"))}Type::Unknown},
  Expr::Return{value}=>value.as_deref().map(|v|self.check_expr(v)).unwrap_or(Type::Unknown),Expr::NetworkScan{subnet}=>{self.check_expr(subnet);Type::Unknown},
  Expr::Import{..}=>Type::Unknown,Expr::Export{item}=>self.check_expr(item),
 }}
 fn check_binary(&mut self,l:Type,op:&TokenType,r:Type)->Type{use TokenType::*;match op{Plus|Minus|Star|Slash=>{let n=matches!((l,r),(Type::Int,Type::Int)|(Type::Int,Type::Float)|(Type::Float,Type::Int)|(Type::Float,Type::Float));if !n&&l!=Type::Unknown&&r!=Type::Unknown{self.error("arithmetic operands must be numeric");Type::Unknown}else if l==Type::Float||r==Type::Float{Type::Float}else{Type::Int}},EqualEqual|NotEqual|Greater|Less|GreaterEqual|LessEqual=>Type::Bool,_=>Type::Unknown}}
 pub fn variables(&self)->&HashMap<String,Type>{&self.variables}
}
