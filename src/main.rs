use falconcore::{compiler::Compiler, lexer::Lexer, parser::Parser, vm::VM};

fn main() {
    let source = r#"
        secure let x = 10
        secure let y = 32
        print x + y

        repeat 3 {
            print "FalconCore"
        }

        if x < y {
            print "comparison: true"
        } else {
            print "comparison: false"
        }
    "#;

    if let Err(error) = run_source(source) {
        eprintln!("FalconCore runtime error: {error:?}");
        std::process::exit(1);
    }
}

fn run_source(source: &str) -> Result<(), falconcore::vm::RuntimeError> {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();

    let mut compiler = Compiler::new();
    compiler.compile(ast);

    let mut vm = VM::new(compiler.get_constants().to_vec(), compiler.get_code().to_vec());
    vm.run()
}
