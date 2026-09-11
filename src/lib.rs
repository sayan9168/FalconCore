//! FalconCore language core and developer toolchain.
pub mod bytecode;
pub mod compiler;
pub mod lexer;
pub mod parser;
pub mod vm;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::{compiler::Compiler, lexer::Lexer, parser::Parser, vm::VM};

    fn run(source: &str) -> Result<(), super::vm::RuntimeError> {
        let mut parser = Parser::new(Lexer::new(source));
        let ast = parser.parse();
        let mut compiler = Compiler::new();
        compiler.compile(ast);
        VM::with_functions(
            compiler.get_constants().to_vec(),
            compiler.get_code().to_vec(),
            compiler.get_functions().clone(),
        )
        .run()
    }

    #[test]
    fn arithmetic() { run("print 2 + 3 * 4").unwrap(); }

    #[test]
    fn float_math() { run("print 2.5 * 4.0").unwrap(); }

    #[test]
    fn functions() { run("fn add(a,b){ return a+b } print add(7,5)").unwrap(); }

    #[test]
    fn recursive_functions() {
        run("fn count(n){ if n > 0 { print n return count(n-1) } return 0 } count(3)").unwrap();
    }

    #[test]
    fn division_by_zero() {
        assert!(matches!(
            run("print 10 / 0"),
            Err(super::vm::RuntimeError::DivisionByZero)
        ));
    }
}
