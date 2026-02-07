// src/vm.rs - FalconCore VM (Enhanced)
use crate::compiler::Opcode;
use crate::parser::Expr;
use std::collections::HashMap;

pub struct VM {
    stack: Vec<Expr>,
    constants: Vec<Expr>,
    code: Vec<Opcode>,
    ip: usize,
    variables: HashMap<String, Expr>,
    functions: HashMap<String, (usize, usize)>, // name → (param_count, start_ip)
    call_stack: Vec<usize>, // return addresses
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
        }
    }

    pub fn run(&mut self) {
        while self.ip < self.code.len() {
            let op = self.code[self.ip].clone();
            match op {
                Opcode::LoadConst(idx) => {
                    self.stack.push(self.constants[idx].clone());
                }
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
                    } else {
                        panic!("Add only supports numbers");
                    }
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
                Opcode::RepeatStart(target) => {
                    // Simple loop - jump back to target after body
                    self.ip = target;
                    continue;
                }
                Opcode::RepeatEnd => {
                    // Jump back to RepeatStart (placeholder)
                    self.ip = 0; // will be fixed later
                }
                Opcode::Call(name, _) => {
                    if let Some((_, start_ip)) = self.functions.get(&name) {
                        self.call_stack.push(self.ip + 1);
                        self.ip = *start_ip;
                        continue;
                    }
                }
                Opcode::Return => {
                    if let Some(return_ip) = self.call_stack.pop() {
                        self.ip = return_ip;
                        continue;
                    } else {
                        break;
                    }
                }
            }
            self.ip += 1;
        }
        println!("VM execution complete!");
    }
}// src/vm.rs - FalconCore Virtual Machine (Enhanced with jump, loop, function call)
use crate::compiler::Opcode;
use crate::parser::Expr;
use std::collections::HashMap;

pub struct VM {
    stack: Vec<Expr>,
    constants: Vec<Expr>,
    code: Vec<Opcode>,
    ip: usize,  // instruction pointer
    variables: HashMap<String, Expr>,
    functions: HashMap<String, (usize, usize)>, // fn name → (param count, code start index)
    call_stack: Vec<usize>, // return addresses
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
        }
    }

    pub fn run(&mut self) {
        while self.ip < self.code.len() {
            let op = &self.code[self.ip].clone();
            match op {
                Opcode::LoadConst(idx) => {
                    let value = self.constants[*idx].clone();
                    self.stack.push(value);
                }
                Opcode::LoadVar(name) => {
                    let value = self.variables.get(name).cloned().unwrap_or(Expr::String("undefined".to_string()));
                    self.stack.push(value);
                }
                Opcode::StoreVar(name) => {
                    let value = self.stack.pop().unwrap();
                    self.variables.insert(name.clone(), value);
                }
                Opcode::Add => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    match (left, right) {
                        (Expr::Number(a), Expr::Number(b)) => self.stack.push(Expr::Number(a + b)),
                        _ => panic!("Add only supports numbers"),
                    }
                }
                Opcode::Print => {
                    let value = self.stack.pop().unwrap();
                    match value {
                        Expr::Number(n) => println!("{}", n),
                        Expr::String(s) => println!("{}", s),
                        Expr::Identifier(id) => println!("Identifier: {}", id),
                        _ => println!("{:?}", value),
                    }
                }
                Opcode::JumpIfFalse(target) => {
                    let cond = self.stack.pop().unwrap();
                    if let Expr::Number(n) = cond {
                        if n == 0 {
                            self.ip = *target;
                            continue;
                        }
                    }
                }
                Opcode::Jump(target) => {
                    self.ip = *target;
                    continue;
                }
                Opcode::RepeatStart(target) => {
                    // Loop start - save current ip as target for repeat end
                    self.ip = *target;
                    continue;
                }
                Opcode::RepeatEnd => {
                    // Jump back to repeat start (simple loop)
                    self.ip = 0; // placeholder - real loop logic later
                }
                Opcode::Call(name, arg_count) => {
                    if let Some((param_count, start_ip)) = self.functions.get(name) {
                        if *arg_count != *param_count {
                            panic!("Function call argument mismatch");
                        }
                        // Push return address
                        self.call_stack.push(self.ip + 1);
                        self.ip = *start_ip;
                        continue;
                    } else {
                        panic!("Function {} not defined", name);
                    }
                }
                Opcode::Return => {
                    if let Some(return_ip) = self.call_stack.pop() {
                        self.ip = return_ip;
                    } else {
                        println!("VM: Program ended");
                        break;
                    }
                }
                _ => println!("VM: Unsupported opcode {:?}", op),
            }
            self.ip += 1;
        }

        println!("VM execution complete!");
    }
}
