use crate::parser::Expr;
use crate::lexer::TokenType;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeError {
    pub message: String,
}

#[derive(Debug, Default)]
pub struct TypeChecker {
    variables: HashMap<String, Type>,
    functions: HashMap<String, (Vec<Type>, Type)>,
    errors: Vec<TypeError>,
}

impl TypeChecker {
    pub fn new() -> Self { Self::default() }

    pub fn check(&mut self, program: &[Expr]) -> Result<(), Vec<TypeError>> {
        for expr in program {
            if let Expr::FnDef { name, params, .. } = expr {
                self.functions.insert(
                    name.clone(),
                    (vec![Type::Unknown; params.len()], Type::Unknown),
                );
            }
        }
        for expr in program { self.check_expr(expr); }
        if self.errors.is_empty() { Ok(()) } else { Err(std::mem::take(&mut self.errors)) }
    }

    fn error(&mut self, message: impl Into<String>) {
        self.errors.push(TypeError { message: message.into() });
    }

    fn check_expr(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::Number(_) => Type::Int,
            Expr::Float(_) => Type::Float,
            Expr::String(_) => Type::String,
            Expr::Identifier(name) => self.variables.get(name).copied().unwrap_or_else(|| {
                self.error(format!("undefined variable: {name}")); Type::Unknown
            }),
            Expr::Let { name, value, .. } => {
                let ty = self.check_expr(value);
                self.variables.insert(name.clone(), ty);
                ty
            }
            Expr::Print { expr } => { self.check_expr(expr); Type::Unknown }
            Expr::If { condition, then_branch, else_branch } => {
                let condition_type = self.check_expr(condition);
                if condition_type != Type::Bool && condition_type != Type::Unknown {
                    self.error("if condition must be boolean");
                }
                for stmt in then_branch { self.check_expr(stmt); }
                if let Some(branch) = else_branch { for stmt in branch { self.check_expr(stmt); } }
                Type::Unknown
            }
            Expr::Repeat { times, body } => {
                let ty = self.check_expr(times);
                if ty != Type::Int && ty != Type::Unknown { self.error("repeat count must be an integer"); }
                for stmt in body { self.check_expr(stmt); }
                Type::Unknown
            }
            Expr::Binary { left, op, right } => {
                let l = self.check_expr(left);
                let r = self.check_expr(right);
                self.check_binary(l, op, r)
            }
            Expr::FnDef { body, .. } => {
                for stmt in body { self.check_expr(stmt); }
                Type::Unknown
            }
            Expr::Call { name, args } => {
                let arg_types: Vec<_> = args.iter().map(|arg| self.check_expr(arg)).collect();
                if let Some((params, _)) = self.functions.get(name).cloned() {
                    if params.len() != arg_types.len() {
                        self.error(format!("function {name} expects {} arguments, got {}", params.len(), arg_types.len()));
                    }
                } else {
                    self.error(format!("undefined function: {name}"));
                }
                Type::Unknown
            }
            Expr::Return { value } => value.as_deref().map(|v| self.check_expr(v)).unwrap_or(Type::Unknown),
            Expr::NetworkScan { subnet } => { self.check_expr(subnet); Type::Unknown }
        }
    }

    fn check_binary(&mut self, left: Type, op: &TokenType, right: Type) -> Type {
        use TokenType::*;
        match op {
            Plus | Minus | Star | Slash => {
                let numeric = matches!((left, right), (Type::Int, Type::Int) | (Type::Int, Type::Float) | (Type::Float, Type::Int) | (Type::Float, Type::Float));
                if !numeric && left != Type::Unknown && right != Type::Unknown {
                    self.error("arithmetic operands must be numeric");
                    Type::Unknown
                } else if left == Type::Float || right == Type::Float { Type::Float } else { Type::Int }
            }
            EqualEqual | NotEqual | Greater | Less | GreaterEqual | LessEqual => Type::Bool,
            _ => Type::Unknown,
        }
    }

    pub fn variables(&self) -> &HashMap<String, Type> { &self.variables }
}
