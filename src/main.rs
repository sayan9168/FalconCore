use falconcore::{bytecode::disassemble, compiler::Compiler, diagnostics::{format_diagnostics, Diagnostic}, lexer::Lexer, parser::Parser, typecheck::TypeChecker, vm::VM, VERSION};
use std::{env, fs, process};

const HELP: &str = r#"FalconCore — security-minded programming language runtime

USAGE:
  falconcore [OPTIONS]

OPTIONS:
  --eval <SOURCE>       Execute inline FalconCore source
  --file <PATH>         Execute a .falcon source file
  --check <SOURCE>      Type-check inline source without executing it
  --check-file <PATH>   Type-check a .falcon source file
  --tokens <SOURCE>     Print lexer output for inline source
  --ast <SOURCE>        Print parsed AST for inline source
  --bytecode <SOURCE>   Compile and disassemble inline source
  --version             Print the compiler version
  --help                Show this help

EXAMPLES:
  falconcore --eval 'print 2 + 3 * 4'
  falconcore --file examples/hello.falcon
  falconcore --check 'print missing(1)'
  falconcore --bytecode 'print 42'
"#;

fn parse_source(source: &str) -> (Vec<falconcore::parser::Expr>, Compiler) {
    let mut parser = Parser::new(Lexer::new(source));
    let ast = parser.parse();
    let mut compiler = Compiler::new();
    compiler.compile(ast.clone());
    (ast, compiler)
}

fn check_source(source: &str) -> Result<(), String> {
    let (ast, _) = parse_source(source);
    let mut checker = TypeChecker::new();
    checker.check(&ast).map_err(|errors| {
        let diagnostics: Vec<_> = errors.into_iter().map(|e| Diagnostic::error("E2001", e.message)).collect();
        format_diagnostics(&diagnostics)
    })
}

fn run_source(source: &str) -> Result<(), String> {
    check_source(source)?;
    let (_, compiler) = parse_source(source);
    VM::with_functions(
        compiler.get_constants().to_vec(),
        compiler.get_code().to_vec(),
        compiler.get_functions().clone(),
    )
    .run()
    .map_err(|e| format!("runtime error: {e:?}"))
}

fn print_tokens(source: &str) {
    let mut lexer = Lexer::new(source);
    loop {
        let token = lexer.next_token();
        println!("{}:{}  {:?}", token.line, token.column, token.kind);
        if matches!(token.kind, falconcore::lexer::TokenType::Eof) { break; }
    }
}

fn value_after_flag(args: &[String], flag: &str) -> Result<String, String> {
    args.iter().position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .cloned()
        .ok_or_else(|| format!("missing value for {flag}\n\n{HELP}"))
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        print!("{HELP}");
        return;
    }
    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("FalconCore {VERSION}");
        return;
    }

    let result = if args.iter().any(|a| a == "--tokens") {
        value_after_flag(&args, "--tokens").map(|s| print_tokens(&s))
    } else if args.iter().any(|a| a == "--ast") {
        value_after_flag(&args, "--ast").map(|s| { let (ast, _) = parse_source(&s); println!("{ast:#?}"); })
    } else if args.iter().any(|a| a == "--bytecode") {
        value_after_flag(&args, "--bytecode").map(|s| { let (_, c) = parse_source(&s); print!("{}", disassemble(c.get_constants(), c.get_code())); })
    } else if args.iter().any(|a| a == "--check") {
        value_after_flag(&args, "--check").and_then(|s| check_source(&s))
    } else if args.iter().any(|a| a == "--check-file") {
        value_after_flag(&args, "--check-file").and_then(|path| fs::read_to_string(&path).map_err(|e| format!("cannot read {path}: {e}"))).and_then(|s| check_source(&s))
    } else if args.iter().any(|a| a == "--eval") {
        value_after_flag(&args, "--eval").and_then(|s| run_source(&s))
    } else if args.iter().any(|a| a == "--file") {
        value_after_flag(&args, "--file").and_then(|path| fs::read_to_string(&path).map_err(|e| format!("cannot read {path}: {e}"))).and_then(|s| run_source(&s))
    } else {
        Err(format!("unknown option\n\n{HELP}"))
    };

    if let Err(message) = result {
        eprintln!("{message}");
        process::exit(2);
    }
}
