//! FalconCore language core and developer toolchain.
pub mod bytecode;
pub mod compiler;
pub mod diagnostics;
pub mod lexer;
pub mod module;
pub mod parser;
pub mod typecheck;
pub mod vm;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::{compiler::Compiler, diagnostics::{Diagnostic, Severity}, lexer::Lexer, parser::Parser, typecheck::{Type, TypeChecker}, vm::VM};
    fn parse(source:&str)->Vec<super::parser::Expr>{Parser::new(Lexer::new(source)).parse()}
    fn run(source:&str)->Result<(),super::vm::RuntimeError>{let ast=parse(source);let mut c=Compiler::new();c.compile(ast);VM::with_functions(c.get_constants().to_vec(),c.get_code().to_vec(),c.get_functions().clone()).run()}
    #[test]fn arithmetic(){run("print 2 + 3 * 4").unwrap()}
    #[test]fn float_math(){run("print 2.5 * 4.0").unwrap()}
    #[test]fn functions(){run("fn add(a,b){ return a+b } print add(7,5)").unwrap()}
    #[test]fn recursive_functions(){run("fn count(n){ if n > 0 { print n return count(n-1) } return 0 } count(3)").unwrap()}
    #[test]fn division_by_zero(){assert!(matches!(run("print 10 / 0"),Err(super::vm::RuntimeError::DivisionByZero)))}
    #[test]fn type_checker_accepts_numeric_program(){let ast=parse("secure let x = 10 print x + 2");let mut c=TypeChecker::new();c.check(&ast).unwrap();assert_eq!(c.variables().get("x"),Some(&Type::Int));}
    #[test]fn type_checker_rejects_bad_repeat(){let ast=parse("repeat \"nope\" { print 1 }");let mut c=TypeChecker::new();assert!(c.check(&ast).is_err());}
    #[test]fn type_checker_rejects_unknown_function(){let ast=parse("print missing(1)");let mut c=TypeChecker::new();assert!(c.check(&ast).is_err());}
    #[test]fn diagnostics_render_with_code_and_help(){let d=Diagnostic::error("E1001","undefined variable: x").at(3,7).with_help("declare x before using it");assert_eq!(d.severity,Severity::Error);assert!(d.to_string().contains("E1001"));assert!(d.to_string().contains("3:7"));assert!(d.to_string().contains("help"));}
    #[test]fn module_syntax_parses(){let ast=parse("import \"lib/math\" as math export secure const answer = 42");assert!(matches!(&ast[0],super::parser::Expr::Import{path,..} if path=="lib/math"));assert!(matches!(&ast[1],super::parser::Expr::Export{..}));}
}
