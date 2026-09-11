use crate::compiler::{FunctionInfo, Opcode};
use crate::parser::Expr;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    StackUnderflow, InvalidConstant(usize), UndefinedVariable(String), UndefinedFunction(String),
    ArityMismatch { function: String, expected: usize, got: usize }, TypeError(&'static str),
    DivisionByZero, InvalidJump(usize), CallStackOverflow,
}

#[derive(Debug, Clone)]
struct Frame { return_ip: usize, locals: HashMap<String, Expr> }

pub struct VM {
    stack: Vec<Expr>, constants: Vec<Expr>, code: Vec<Opcode>, ip: usize,
    globals: HashMap<String, Expr>, frames: Vec<Frame>, functions: HashMap<String, FunctionInfo>,
    loop_stack: Vec<(usize, i64)>, max_call_depth: usize,
}

impl VM {
    pub fn new(constants: Vec<Expr>, code: Vec<Opcode>) -> Self { Self::with_functions(constants,code,HashMap::new()) }
    pub fn with_functions(constants: Vec<Expr>, code: Vec<Opcode>, functions: HashMap<String,FunctionInfo>) -> Self {
        Self {stack:Vec::new(),constants,code,ip:0,globals:HashMap::new(),frames:Vec::new(),functions,loop_stack:Vec::new(),max_call_depth:1024}
    }
    pub fn run(&mut self)->Result<(),RuntimeError>{
        while self.ip<self.code.len(){let op=self.code[self.ip].clone();match op{
            Opcode::LoadConst(i)=>{self.stack.push(self.constants.get(i).cloned().ok_or(RuntimeError::InvalidConstant(i))?);}
            Opcode::LoadVar(name)=>{let v=self.load_var(&name)?;self.stack.push(v);}
            Opcode::StoreVar(name)=>{let v=self.pop()?;self.store_var(name,v);}
            Opcode::Add|Opcode::Sub|Opcode::Mul|Opcode::Div=>self.binary_numeric(op)?,
            Opcode::Equal|Opcode::NotEqual|Opcode::Greater|Opcode::Less|Opcode::GreaterEqual|Opcode::LessEqual=>self.compare(op)?,
            Opcode::Print=>{let v=self.pop()?;println!("{v:?}");}
            Opcode::JumpIfFalse(t)=>{let c=self.pop()?;if !Self::truthy(&c){self.jump(t)?;continue;}}
            Opcode::Jump(t)=>{self.jump(t)?;continue;}
            Opcode::RepeatStart=>{let n=match self.pop()?{Expr::Number(n) if n>=0=>n,_=>return Err(RuntimeError::TypeError("repeat expects a non-negative integer"))};self.loop_stack.push((self.ip,n));}
            Opcode::RepeatEnd(start)=>{if let Some((_,n))=self.loop_stack.pop(){if n>1{self.loop_stack.push((start,n-1));self.jump(start+1)?;continue;}}}
            Opcode::Call(name,argc)=>self.call(&name,argc)?,
            Opcode::Return=>{if self.frames.is_empty(){break;}let result=self.pop().unwrap_or(Expr::Number(0));let frame=self.frames.pop().unwrap();self.ip=frame.return_ip;self.stack.push(result);continue;}
            Opcode::NetworkScan=>{let _=self.pop()?;self.stack.push(Expr::String("network.scan requires an explicit authorized host-side capability".into()));}
            Opcode::Halt=>break,
        }self.ip+=1;}Ok(())}
    fn call(&mut self,name:&str,argc:usize)->Result<(),RuntimeError>{let info=self.functions.get(name).cloned().ok_or_else(||RuntimeError::UndefinedFunction(name.into()))?;if argc!=info.params.len(){return Err(RuntimeError::ArityMismatch{function:name.into(),expected:info.params.len(),got:argc})}if self.frames.len()>=self.max_call_depth{return Err(RuntimeError::CallStackOverflow)}let mut locals=HashMap::new();for p in info.params.iter().rev(){locals.insert(p.clone(),self.pop()?);}let return_ip=self.ip+1;self.frames.push(Frame{return_ip,locals});self.ip=info.entry;Ok(())}
    fn load_var(&self,name:&str)->Result<Expr,RuntimeError>{if let Some(f)=self.frames.last(){if let Some(v)=f.locals.get(name){return Ok(v.clone())}}self.globals.get(name).cloned().ok_or_else(||RuntimeError::UndefinedVariable(name.into()))}
    fn store_var(&mut self,name:String,v:Expr){if let Some(f)=self.frames.last_mut(){f.locals.insert(name,v);}else{self.globals.insert(name,v);}}
    fn pop(&mut self)->Result<Expr,RuntimeError>{self.stack.pop().ok_or(RuntimeError::StackUnderflow)}
    fn binary_numeric(&mut self,op:Opcode)->Result<(),RuntimeError>{let r=self.pop()?;let l=self.pop()?;match(l,r){(Expr::Number(a),Expr::Number(b))=>{if matches!(op,Opcode::Div)&&b==0{return Err(RuntimeError::DivisionByZero)}let v=match op{Opcode::Add=>Expr::Number(a+b),Opcode::Sub=>Expr::Number(a-b),Opcode::Mul=>Expr::Number(a*b),Opcode::Div=>Expr::Number(a/b),_=>unreachable!()};self.stack.push(v)},(Expr::Float(a),Expr::Float(b))|(Expr::Number(a),Expr::Float(b))=>{let a=a as f64;if matches!(op,Opcode::Div)&&b==0.0{return Err(RuntimeError::DivisionByZero)}let v=match op{Opcode::Add=>a+b,Opcode::Sub=>a-b,Opcode::Mul=>a*b,Opcode::Div=>a/b,_=>unreachable!()};self.stack.push(Expr::Float(v))},(Expr::Float(a),Expr::Number(b))=>{let b=b as f64;if matches!(op,Opcode::Div)&&b==0.0{return Err(RuntimeError::DivisionByZero)}let v=match op{Opcode::Add=>a+b,Opcode::Sub=>a-b,Opcode::Mul=>a*b,Opcode::Div=>a/b,_=>unreachable!()};self.stack.push(Expr::Float(v))},_=>return Err(RuntimeError::TypeError("arithmetic expects numeric values"))}Ok(())}
    fn compare(&mut self,op:Opcode)->Result<(),RuntimeError>{let r=self.pop()?;let l=self.pop()?;let v=match(&l,&r,op){(Expr::Number(a),Expr::Number(b),Opcode::Equal)=>a==b,(Expr::Number(a),Expr::Number(b),Opcode::NotEqual)=>a!=b,(Expr::Number(a),Expr::Number(b),Opcode::Greater)=>a>b,(Expr::Number(a),Expr::Number(b),Opcode::Less)=>a<b,(Expr::Number(a),Expr::Number(b),Opcode::GreaterEqual)=>a>=b,(Expr::Number(a),Expr::Number(b),Opcode::LessEqual)=>a<=b,(Expr::Float(a),Expr::Float(b),Opcode::Equal)=>a==b,(Expr::Float(a),Expr::Float(b),Opcode::NotEqual)=>a!=b,(Expr::Float(a),Expr::Float(b),Opcode::Greater)=>a>b,(Expr::Float(a),Expr::Float(b),Opcode::Less)=>a<b,(Expr::Float(a),Expr::Float(b),Opcode::GreaterEqual)=>a>=b,(Expr::Float(a),Expr::Float(b),Opcode::LessEqual)=>a<=b,(Expr::String(a),Expr::String(b),Opcode::Equal)=>a==b,(Expr::String(a),Expr::String(b),Opcode::NotEqual)=>a!=b,_=>return Err(RuntimeError::TypeError("comparison requires compatible values"))};self.stack.push(Expr::Number(if v{1}else{0}));Ok(())}
    fn truthy(v:&Expr)->bool{match v{Expr::Number(n)=>*n!=0,Expr::Float(n)=>*n!=0.0,Expr::String(s)=>!s.is_empty(),_=>true}}
    fn jump(&mut self,t:usize)->Result<(),RuntimeError>{if t>=self.code.len(){return Err(RuntimeError::InvalidJump(t))}self.ip=t;Ok(())}
}
