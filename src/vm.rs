// src/vm.rs - FalconCore VM (Real repeat loop with counter + function call)
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
    call_stack: Vec<(usize, HashMap<String, Expr>)>, // (return_ip, saved_locals)
    loop_counters: Vec<(usize, i64)>, // (loop_start_ip, remaining_iterations)
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
            loop_counters: vec![],
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
                Opcode::RepeatStart => {
                    // Expect times on stack
                    let times = if let Expr::Number(n) = self.stack.pop().unwrap() {
                        n
                    } else {
                        panic!("Repeat expects number");
                    };
                    self.loop_counters.push((self.ip, times));
                }
                Opcode::RepeatEnd => {
                    if let Some((start_ip, mut count)) = self.loop_counters.pop() {
                        count -= 1;
                        if count > 0 {
                            self.loop_counters.push((start_ip, count));
                            self.ip = start_ip;
                            continue;
                        }
                    }
                }
                Opcode::Call(name, arg_count) => {
                    if let Some((params, start_ip)) = self.functions.get(&name) {
                        if *arg_count != params.len() {
                            panic!("Argument count mismatch for {}", name);
                        }

                        // Create local scope
                        let mut locals = HashMap::new();
                        for param in params.iter().rev() {
                            let arg = self.stack.pop().unwrap();
                            locals.insert(param.clone(), arg);
                        }

                        // Push current state
                        self.call_stack.push((self.ip + 1, self.variables.clone()));
                        self.variables = locals;

                        self.ip = *start_ip;
                        continue;
                    } else {
                        panic!("Function {} not defined", name);
                    }
                }
                Opcode::Return => {
                    if let Some((return_ip, saved_locals)) = self.call_stack.pop() {
                        self.variables = saved_locals;
                        self.ip = return_ip;
                        continue;
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
