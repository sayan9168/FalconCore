use crate::{lexer::TokenType, parser::Expr};

/// Performs deterministic, semantics-preserving constant folding on the AST.
pub fn optimize_program(ast: Vec<Expr>) -> Vec<Expr> {
    ast.into_iter().map(optimize_expr).collect()
}

fn optimize_expr(expr: Expr) -> Expr {
    match expr {
        Expr::Binary { left, op, right } => {
            let left = optimize_expr(*left);
            let right = optimize_expr(*right);
            fold_binary(left.clone(), &op, right.clone()).unwrap_or(Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            })
        }
        Expr::Let { is_secure, is_const, name, value } => Expr::Let {
            is_secure,
            is_const,
            name,
            value: Box::new(optimize_expr(*value)),
        },
        Expr::Print { expr } => Expr::Print { expr: Box::new(optimize_expr(*expr)) },
        Expr::If { condition, then_branch, else_branch } => Expr::If {
            condition: Box::new(optimize_expr(*condition)),
            then_branch: then_branch.into_iter().map(optimize_expr).collect(),
            else_branch: else_branch.map(|b| b.into_iter().map(optimize_expr).collect()),
        },
        Expr::Repeat { times, body } => Expr::Repeat {
            times: Box::new(optimize_expr(*times)),
            body: body.into_iter().map(optimize_expr).collect(),
        },
        Expr::FnDef { name, params, param_types, return_type, body } => Expr::FnDef {
            name,
            params,
            param_types,
            return_type,
            body: body.into_iter().map(optimize_expr).collect(),
        },
        Expr::Call { name, args } => Expr::Call { name, args: args.into_iter().map(optimize_expr).collect() },
        Expr::Return { value } => Expr::Return { value: value.map(|v| Box::new(optimize_expr(*v))) },
        Expr::NetworkScan { subnet } => Expr::NetworkScan { subnet: Box::new(optimize_expr(*subnet)) },
        Expr::Export { item } => Expr::Export { item: Box::new(optimize_expr(*item)) },
        other => other,
    }
}

fn fold_binary(left: Expr, op: &TokenType, right: Expr) -> Option<Expr> {
    match (left, right) {
        (Expr::Number(a), Expr::Number(b)) => match op {
            TokenType::Plus => Some(Expr::Number(a + b)),
            TokenType::Minus => Some(Expr::Number(a - b)),
            TokenType::Star => Some(Expr::Number(a * b)),
            TokenType::Slash if b != 0 => Some(Expr::Number(a / b)),
            TokenType::EqualEqual => Some(Expr::Bool(a == b)),
            TokenType::NotEqual => Some(Expr::Bool(a != b)),
            TokenType::Greater => Some(Expr::Bool(a > b)),
            TokenType::Less => Some(Expr::Bool(a < b)),
            TokenType::GreaterEqual => Some(Expr::Bool(a >= b)),
            TokenType::LessEqual => Some(Expr::Bool(a <= b)),
            _ => None,
        },
        (Expr::Float(a), Expr::Float(b)) => match op {
            TokenType::Plus => Some(Expr::Float(a + b)),
            TokenType::Minus => Some(Expr::Float(a - b)),
            TokenType::Star => Some(Expr::Float(a * b)),
            TokenType::Slash if b != 0.0 => Some(Expr::Float(a / b)),
            TokenType::EqualEqual => Some(Expr::Bool(a == b)),
            TokenType::NotEqual => Some(Expr::Bool(a != b)),
            TokenType::Greater => Some(Expr::Bool(a > b)),
            TokenType::Less => Some(Expr::Bool(a < b)),
            TokenType::GreaterEqual => Some(Expr::Bool(a >= b)),
            TokenType::LessEqual => Some(Expr::Bool(a <= b)),
            _ => None,
        },
        (Expr::String(a), Expr::String(b)) => match op {
            TokenType::EqualEqual => Some(Expr::Bool(a == b)),
            TokenType::NotEqual => Some(Expr::Bool(a != b)),
            _ => None,
        },
        (Expr::Bool(a), Expr::Bool(b)) => match op {
            TokenType::EqualEqual => Some(Expr::Bool(a == b)),
            TokenType::NotEqual => Some(Expr::Bool(a != b)),
            _ => None,
        },
        (Expr::Null, Expr::Null) => match op {
            TokenType::EqualEqual => Some(Expr::Bool(true)),
            TokenType::NotEqual => Some(Expr::Bool(false)),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn folds_integer_arithmetic() {
        let ast = optimize_program(vec![Expr::Binary {
            left: Box::new(Expr::Number(2)), op: TokenType::Plus, right: Box::new(Expr::Number(3)),
        }]);
        assert_eq!(ast, vec![Expr::Number(5)]);
    }
    #[test]
    fn preserves_division_by_zero() {
        let ast = optimize_program(vec![Expr::Binary {
            left: Box::new(Expr::Number(10)), op: TokenType::Slash, right: Box::new(Expr::Number(0)),
        }]);
        assert!(matches!(&ast[0], Expr::Binary { .. }));
    }
}
