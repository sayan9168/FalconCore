// src/compiler.rs - FalconCore Bytecode Compiler (Updated for VM)
use crate::parser::Expr;

#[derive(Debug, Clone)]
pub enum Opcode {
    LoadConst(usize),
    LoadVar(String),
    StoreVar(String),
    Add,
    Print,
    JumpIfFalse(usize),
    Jump(usize),
    RepeatStart(usize),
    RepeatEnd,
    Call(String, usize),
    Return,
}

pub struct Compiler {
    constants: Vec<Expr>,
    code: Vec<Opcode>,
    labels: Vec<usize>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            constants: vec![],
            code: vec![],
            labels: vec![],
        }
    }

    pub fn compile(&mut self, ast: Vec<Expr>) {
        for expr in ast {
            self.compile_expr(&expr);
        }
        self.code.push(Opcode::Return);
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
            Expr::Identifier(id) => {
                self.code.push(Opcode::LoadVar(id.clone()));
            }
            Expr::Binary { left, op, right } => {
                self.compile_expr(left);
                self.compile_expr(right);
                match op {
                    TokenType::Plus => self.code.push(Opcode::Add),
                    _ => panic!("Unsupported operator"),
                }
            }
            Expr::Let { name, value, .. } => {
                self.compile_expr(value);
                self.code.push(Opcode::StoreVar(name.clone()));
            }
            Expr::Print { expr } => {
                self.compile_expr(expr);
                self.code.push(Opcode::Print);
            }
            Expr::If { condition, then_branch, else_branch } => {
                self.compile_expr(condition);
                let jump_false_pos = self.code.len();
                self.code.push(Opcode::JumpIfFalse(0)); // placeholder

                for stmt in then_branch {
                    self.compile_expr(stmt);
                }

                let jump_end_pos = self.code.len();
                self.code.push(Opcode::Jump(0)); // placeholder

                self.code[jump_false_pos] = Opcode::JumpIfFalse(self.code.len());

                if let Some(else_branch) = else_branch {
                    for stmt in else_branch {
                        self.compile_expr(stmt);
                    }
                }

                self.code[jump_end_pos] = Opcode::Jump(self.code.len());
            }
            Expr::Repeat { times, body } => {
                self.compile_expr(times);
                let loop_start = self.code.len();
                self.code.push(Opcode::RepeatStart(loop_start));

                for stmt in body {
                    self.compile_expr(stmt);
                }

                self.code.push(Opcode::RepeatEnd);
            }
            Expr::FnDef { name, params, body } => {
                // Store function start position
                let start_ip = self.code.len();
                self.code.push(Opcode::Return); // placeholder for return
                for stmt in body {
                    self.compile_expr(stmt);
                }
                // self.functions.insert(name.clone(), (params.len(), start_ip)); // later
            }
            Expr::Return { value } => {
                if let Some(val) = value {
                    self.compile_expr(val);
                }
                self.code.push(Opcode::Return);
            }
            _ => panic!("Unsupported expression"),
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
}// src/compiler.rs - FalconCore Bytecode Compiler (Basic)
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
