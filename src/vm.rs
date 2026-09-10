use crate::compiler::Opcode;
use crate::parser::Expr;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    StackUnderflow,
    InvalidConstant(usize),
    UndefinedVariable(String),
    TypeError(&'static str),
    DivisionByZero,
    InvalidJump(usize),
}

pub struct VM {
    stack: Vec<Expr>,
    constants: Vec<Expr>,
    code: Vec<Opcode>,
    ip: usize,
    variables: HashMap<String, Expr>,
    loop_stack: Vec<(usize, i64)>,
}

impl VM {
    pub fn new(constants: Vec<Expr>, code: Vec<Opcode>) -> Self {
        Self { stack: Vec::new(), constants, code, ip: 0, variables: HashMap::new(), loop_stack: Vec::new() }
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        while self.ip < self.code.len() {
            let op = self.code[self.ip].clone();
            match op {
                Opcode::LoadConst(index) => {
                    let value = self.constants.get(index).cloned().ok_or(RuntimeError::InvalidConstant(index))?;
                    self.stack.push(value);
                }
                Opcode::LoadVar(name) => {
                    let value = self.variables.get(&name).cloned().ok_or_else(|| RuntimeError::UndefinedVariable(name.clone()))?;
                    self.stack.push(value);
                }
                Opcode::StoreVar(name) => {
                    let value = self.pop()?;
                    self.variables.insert(name, value);
                }
                Opcode::Add | Opcode::Sub | Opcode::Mul | Opcode::Div => self.binary_numeric(op)?,
                Opcode::Equal | Opcode::NotEqual | Opcode::Greater | Opcode::Less | Opcode::GreaterEqual | Opcode::LessEqual => self.compare(op)?,
                Opcode::Print => {
                    let value = self.pop()?;
                    match value { Expr::Number(n) => println!("{n}"), Expr::String(s) => println!("{s}"), other => println!("{other:?}") }
                }
                Opcode::JumpIfFalse(target) => {
                    let cond = self.pop()?;
                    if !Self::truthy(&cond) { self.jump(target)?; continue; }
                }
                Opcode::Jump(target) => { self.jump(target)?; continue; }
                Opcode::RepeatStart => {
                    let count = match self.pop()? { Expr::Number(n) if n >= 0 => n, _ => return Err(RuntimeError::TypeError("repeat expects a non-negative integer")) };
                    self.loop_stack.push((self.ip, count));
                }
                Opcode::RepeatEnd(start) => {
                    if let Some((_, remaining)) = self.loop_stack.pop() {
                        if remaining > 1 { self.loop_stack.push((start, remaining - 1)); self.jump(start + 1)?; continue; }
                    }
                }
                Opcode::NetworkScan => {
                    let _subnet = self.pop()?;
                    self.stack.push(Expr::String("network.scan is an explicit host-side capability; use the NetworkStack API from an authorized environment".into()));
                }
                Opcode::Return | Opcode::Halt => break,
                Opcode::Call(_, _) => return Err(RuntimeError::TypeError("function calls are not enabled in VM v0.2 yet")),
            }
            self.ip += 1;
        }
        Ok(())
    }

    fn pop(&mut self) -> Result<Expr, RuntimeError> { self.stack.pop().ok_or(RuntimeError::StackUnderflow) }

    fn binary_numeric(&mut self, op: Opcode) -> Result<(), RuntimeError> {
        let right = self.pop()?;
        let left = self.pop()?;
        let (a, b) = match (left, right) { (Expr::Number(a), Expr::Number(b)) => (a, b), _ => return Err(RuntimeError::TypeError("arithmetic expects integers")) };
        let value = match op { Opcode::Add => a + b, Opcode::Sub => a - b, Opcode::Mul => a * b, Opcode::Div => if b == 0 { return Err(RuntimeError::DivisionByZero) } else { a / b }, _ => unreachable!() };
        self.stack.push(Expr::Number(value));
        Ok(())
    }

    fn compare(&mut self, op: Opcode) -> Result<(), RuntimeError> {
        let right = self.pop()?;
        let left = self.pop()?;
        let result = match (&left, &right, op) {
            (Expr::Number(a), Expr::Number(b), Opcode::Equal) => a == b,
            (Expr::Number(a), Expr::Number(b), Opcode::NotEqual) => a != b,
            (Expr::Number(a), Expr::Number(b), Opcode::Greater) => a > b,
            (Expr::Number(a), Expr::Number(b), Opcode::Less) => a < b,
            (Expr::Number(a), Expr::Number(b), Opcode::GreaterEqual) => a >= b,
            (Expr::Number(a), Expr::Number(b), Opcode::LessEqual) => a <= b,
            (Expr::String(a), Expr::String(b), Opcode::Equal) => a == b,
            (Expr::String(a), Expr::String(b), Opcode::NotEqual) => a != b,
            _ => return Err(RuntimeError::TypeError("comparison requires compatible values")),
        };
        self.stack.push(Expr::Number(if result { 1 } else { 0 }));
        Ok(())
    }

    fn truthy(value: &Expr) -> bool { match value { Expr::Number(n) => *n != 0, Expr::String(s) => !s.is_empty(), _ => true } }
    fn jump(&mut self, target: usize) -> Result<(), RuntimeError> { if target >= self.code.len() { return Err(RuntimeError::InvalidJump(target)); } self.ip = target; Ok(()) }
}
