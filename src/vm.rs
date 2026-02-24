// src/vm.rs - FalconCore VM (Complete with And, Or, Not + logical ops)
use crate::compiler::Opcode;
use crate::parser::Expr;
use std::collections::HashMap;

pub struct VM {
    stack: Vec<Expr>,
    constants: Vec<Expr>,
    code: Vec<Opcode>,
    ip: usize,
    variables: HashMap<String, Expr>,
    functions: HashMap<String, (Vec<String>, usize)>, // name → (params, start_ip)
    call_stack: Vec<(usize, HashMap<String, Expr>)>,
    loop_stack: Vec<(usize, i64)>,
}

impl VM {
    pub fn new(constants: Vec<Expr>, code: Vec<Opcode>) -> Self {
        VM {
            stack: vec![],
            constants,
            code,
            ip: 0,
            variables: HashMap::new(),
            functions: HashMap::new(),
            call_stack: vec![],
            loop_stack: vec![],
        }
    }

    pub fn run(&mut self) {
        while self.ip < self.code.len() {
            let op = self.code[self.ip].clone();
            match op {
                Opcode::LoadConst(idx) => self.stack.push(self.constants[idx].clone()),
                Opcode::LoadVar(name) => {
                    let value = self.variables.get(&name).cloned().unwrap_or(Expr::String("undefined".to_string()));
                    self.stack.push(value);
                }
                Opcode::StoreVar(name) => {
                    let value = self.stack.pop().unwrap();
                    self.variables.insert(name, value);
                }

                Opcode::Add => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    if let (Expr::Number(a), Expr::Number(b)) = (left, right) {
                        self.stack.push(Expr::Number(a + b));
                    }
                }
                Opcode::Sub => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    if let (Expr::Number(a), Expr::Number(b)) = (left, right) {
                        self.stack.push(Expr::Number(a - b));
                    }
                }
                Opcode::Mul => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    if let (Expr::Number(a), Expr::Number(b)) = (left, right) {
                        self.stack.push(Expr::Number(a * b));
                    }
                }
                Opcode::Div => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    if let (Expr::Number(a), Expr::Number(b)) = (left, right) {
                        if b == 0 { panic!("Division by zero"); }
                        self.stack.push(Expr::Number(a / b));
                    }
                }

                // Logical opcodes (1 = true, 0 = false)
                Opcode::And => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    let result = if let (Expr::Number(a), Expr::Number(b)) = (left, right) {
                        if a != 0 && b != 0 { 1 } else { 0 }
                    } else { 0 };
                    self.stack.push(Expr::Number(result));
                }
                Opcode::Or => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    let result = if let (Expr::Number(a), Expr::Number(b)) = (left, right) {
                        if a != 0 || b != 0 { 1 } else { 0 }
                    } else { 0 };
                    self.stack.push(Expr::Number(result));
                }
                Opcode::Not => {
                    let value = self.stack.pop().unwrap();
                    let result = if let Expr::Number(n) = value {
                        if n == 0 { 1 } else { 0 }
                    } else { 0 };
                    self.stack.push(Expr::Number(result));
                }

                Opcode::Print => {
                    let value = self.stack.pop().unwrap();
                    match value {
                        Expr::Number(n) => println!("{}", n),
                        Expr::String(s) => println!("{}", s),
                        _ => println!("{:?}", value),
                    }
                }

                Opcode::JumpIfFalse(target) => {
                    let cond = self.stack.pop().unwrap();
                    if let Expr::Number(n) = cond {
                        if n == 0 {
                            self.ip = target;
                            continue;
                        }
                    }
                }
                Opcode::Jump(target) => {
                    self.ip = target;
                    continue;
                }

                Opcode::RepeatStart => {
                    let times = if let Expr::Number(n) = self.stack.pop().unwrap() {
                        n
                    } else {
                        panic!("Repeat expects number");
                    };
                    self.loop_stack.push((self.ip, times));
                }
                Opcode::RepeatEnd => {
                    if let Some((start_ip, mut count)) = self.loop_stack.pop() {
                        count -= 1;
                        if count > 0 {
                            self.loop_stack.push((start_ip, count));
                            self.ip = start_ip;
                            continue;
                        }
                    }
                }

                Opcode::Call(name, arg_count) => {
                    if let Some((params, start_ip)) = self.functions.get(&name) {
                        if *arg_count != params.len() {
                            panic!("Argument count mismatch");
                        }
                        let mut locals = HashMap::new();
                        for param in params.iter().rev() {
                            let arg = self.stack.pop().unwrap();
                            locals.insert(param.clone(), arg);
                        }
                        self.call_stack.push((self.ip + 1, self.variables.clone()));
                        self.variables = locals;
                        self.ip = *start_ip;
                        continue;
                    }
                }
                Opcode::Return => {
                    if let Some((return_ip, saved_locals)) = self.call_stack.pop() {
                        self.variables = saved_locals;
                        self.ip = return_ip;
                        continue;
                    } else {
                        break;
                    }
                }
                _ => println!("Unsupported opcode: {:?}", op),
            }
            self.ip += 1;
        }
        println!("VM finished!");
    }
                            }
