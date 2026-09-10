//! FalconCore language core.
//!
//! FalconCore is designed as a small, security-minded language with an
//! embeddable lexer → parser → bytecode compiler → VM pipeline.

pub mod compiler;
pub mod lexer;
pub mod parser;
pub mod vm;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::{compiler::Compiler, lexer::Lexer, parser::{Expr, Parser}, vm::VM};

    fn compile_and_run(source: &str) -> Result<(), super::vm::RuntimeError> {
        let mut parser = Parser::new(Lexer::new(source));
        let ast = parser.parse();
        let mut compiler = Compiler::new();
        compiler.compile(ast);
        VM::new(compiler.get_constants().to_vec(), compiler.get_code().to_vec()).run()
    }

    #[test]
    fn parses_secure_variable() {
        let mut parser = Parser::new(Lexer::new("secure let answer = 42"));
        let ast = parser.parse();
        assert!(matches!(ast.first(), Some(Expr::Let { is_secure: true, is_const: false, .. })));
    }

    #[test]
    fn arithmetic_pipeline_executes() {
        compile_and_run("secure let x = 2 + 3 * 4\nprint x").unwrap();
    }

    #[test]
    fn division_by_zero_is_reported() {
        let result = compile_and_run("print 10 / 0");
        assert!(matches!(result, Err(super::vm::RuntimeError::DivisionByZero)));
    }
}
