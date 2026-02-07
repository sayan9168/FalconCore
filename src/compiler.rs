// src/compiler.rs - FalconCore Bytecode Compiler (Basic)
use crate::parser::Expr;

#[derive(Debug, Clone)]
pub enum Opcode {
    LoadConst(usize), // index in constant table
    Add,
    Print,
    Return,
}

pub struct Compiler {
    constants: Vec<Expr>,
    code: Vec<Opcode>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            constants: vec![],
            code: vec![],
        }
    }

    pub fn compile(&mut self, ast: Vec<Expr>) {
        for expr in ast {
            self.compile_expr(&expr);
        }
    }

    fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(n) => {
                let idx = self.add_constant(Expr::Number(*n));
                self.code.push(Opcode::LoadConst(idx));
            }
            Expr::String(s) => {
                let idx = self.add_constant(Expr::String(s.clone()));
                self.code.push(Opcode::LoadConst(idx));
            }
            Expr::Binary { left, op, right } => {
                self.compile_expr(left);
                self.compile_expr(right);
                match op {
                    TokenType::Plus => self.code.push(Opcode::Add),
                    _ => panic!("Unsupported operator"),
                }
            }
            Expr::Print { expr } => {
                self.compile_expr(expr);
                self.code.push(Opcode::Print);
            }
            _ => panic!("Unsupported expression in compiler"),
        }
    }

    fn add_constant(&mut self, value: Expr) -> usize {
        let idx = self.constants.len();
        self.constants.push(value);
        idx
    }

    pub fn get_code(&self) -> &Vec<Opcode> {
        &self.code
    }

    pub fn get_constants(&self) -> &Vec<Expr> {
        &self.constants
    }
}
