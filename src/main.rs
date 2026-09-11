use falconcore::{compiler::Compiler,lexer::Lexer,parser::Parser,vm::VM};
fn main(){let source=r#"fn add(a,b){ return a+b }
secure let x = add(10,32)
print x
print 3.5 * 2.0
repeat 2 { print "FalconCore v0.3" }
"#;let mut parser=Parser::new(Lexer::new(source));let ast=parser.parse();let mut compiler=Compiler::new();compiler.compile(ast);if let Err(e)=VM::with_functions(compiler.get_constants().to_vec(),compiler.get_code().to_vec(),compiler.get_functions().clone()).run(){eprintln!("runtime error: {e:?}");std::process::exit(1)}}
