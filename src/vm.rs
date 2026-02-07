// src/vm.rs - FalconCore Virtual Machine (Basic)
use crate::compiler::Opcode;
use crate::parser::Expr;

pub struct VM {
    stack: Vec<Expr>,
    constants: Vec<Expr>,
    code: Vec<Opcode>,
    ip: usize,  // instruction pointer
}

impl VM {
    pub fn new(constants: Vec<Expr>, code: Vec<Opcode>) -> Self {
        VM {
            stack: vec![],
            constants,
            code,
            ip: 0,
        }
    }

    pub fn run(&mut self) {
        while self.ip < self.code.len() {
            let op = &self.code[self.ip];
            match op {
                Opcode::LoadConst(idx) => {
                    let value = self.constants[*idx].clone();
                    self.stack.push(value);
                }
                Opcode::Add => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    match (left, right) {
                        (Expr::Number(a), Expr::Number(b)) => {
                            self.stack.push(Expr::Number(a + b));
                        }
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
                Opcode::Return => {
                    println!("VM: Return executed");
                    break;
                }
                _ => println!("VM: Unsupported opcode {:?}", op),
            }
            self.ip += 1;
        }

        println!("VM execution complete!");
    }
          }
