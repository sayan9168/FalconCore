use falconcore::{bytecode::{deserialize, disassemble, serialize}, compiler::Compiler, diagnostics::{format_diagnostics, Diagnostic}, lexer::Lexer, module::ModuleResolver, parser::Parser, typecheck::TypeChecker, vm::VM, VERSION};
use std::{env, fs, path::{Path, PathBuf}, process};

const HELP: &str = r#"FalconCore — security-minded programming language runtime

USAGE:
  falconcore <COMMAND> [OPTIONS]

COMMANDS:
  run <SOURCE.falcon>        Execute a source file
  build <SOURCE.falcon>      Compile source + modules into SOURCE.fbc
  run-fbc <FILE.fbc>         Execute a compiled FCBC artifact
  inspect <FILE.fbc>         Inspect a compiled FCBC artifact
  check <SOURCE.falcon>      Type-check a source file and its modules
  eval <SOURCE>              Execute inline FalconCore source
  tokens <SOURCE>            Print lexer output
  ast <SOURCE>               Print parsed AST
  bytecode <SOURCE>          Compile and disassemble inline source
  --version                  Print the compiler version
  --help                     Show this help

LEGACY FLAGS:
  --eval <SOURCE>            Execute inline source
  --file <PATH>              Execute a source file
  --check <SOURCE>           Type-check inline source
  --check-file <PATH>        Type-check a source file
  --tokens <SOURCE>          Print lexer output
  --ast <SOURCE>             Print parsed AST
  --bytecode <SOURCE>        Compile and disassemble inline source

EXAMPLES:
  falconcore eval 'print 2 + 3 * 4'
  falconcore run examples/hello.falcon
  falconcore build examples/hello.falcon
  falconcore run-fbc examples/hello.fbc
  falconcore inspect examples/hello.fbc
  falconcore check examples/hello.falcon
"#;

fn parse_source(source: &str) -> (Vec<falconcore::parser::Expr>, Compiler) {
    let mut parser = Parser::new(Lexer::new(source));
    let ast = parser.parse();
    let mut compiler = Compiler::new();
    compiler.compile(ast.clone());
    (ast, compiler)
}

fn check_ast(ast: &[falconcore::parser::Expr], path: &str) -> Result<(), String> {
    let mut checker = TypeChecker::new();
    checker.check(ast).map_err(|errors| {
        let diagnostics: Vec<_> = errors.into_iter().map(|e| Diagnostic::error("E2001", e.message)).collect();
        format!("{path}:\n{}", format_diagnostics(&diagnostics))
    })
}

fn check_source(source: &str) -> Result<(), String> {
    let (ast, _) = parse_source(source);
    check_ast(&ast, "<inline>")
}

fn check_file_with_modules(path: &Path) -> Result<Vec<falconcore::module::Module>, String> {
    let mut resolver = ModuleResolver::new();
    let modules = resolver.resolve(path).map_err(|e| format!("module resolution error: {e:?}"))?;
    for module in &modules { check_ast(&module.ast, &module.path.display().to_string())?; }
    Ok(modules)
}

fn run_source(source: &str) -> Result<(), String> {
    check_source(source)?;
    let (_, compiler) = parse_source(source);
    VM::with_functions(compiler.get_constants().to_vec(), compiler.get_code().to_vec(), compiler.get_functions().clone())
        .run().map_err(|e| format!("runtime error: {e:?}"))
}

fn build_file(path: &Path, output: Option<&Path>) -> Result<PathBuf, String> {
    check_file_with_modules(path)?;
    let source = fs::read_to_string(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    let (_, compiler) = parse_source(&source);
    let bytes = serialize(compiler.get_constants(), compiler.get_code(), compiler.get_functions())
        .map_err(|e| format!("bytecode serialization error: {e}"))?;
    let out = output.map(PathBuf::from).unwrap_or_else(|| path.with_extension("fbc"));
    fs::write(&out, bytes).map_err(|e| format!("cannot write {}: {e}", out.display()))?;
    println!("built {}", out.display());
    Ok(out)
}

fn run_fbc(path: &Path) -> Result<(), String> {
    let data = fs::read(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    let (constants, code, functions) = deserialize(&data).map_err(|e| format!("bytecode error: {e}"))?;
    VM::with_functions(constants, code, functions).run().map_err(|e| format!("runtime error: {e:?}"))
}

fn inspect_fbc(path: &Path) -> Result<(), String> {
    let data = fs::read(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    let (constants, code, functions) = deserialize(&data).map_err(|e| format!("bytecode error: {e}"))?;
    println!("Artifact: {}", path.display());
    println!("Functions: {}", functions.len());
    for (name, info) in &functions { println!("  {name} @ {}({})", info.entry, info.params.join(", ")); }
    print!("{}", disassemble(&constants, &code));
    Ok(())
}

fn print_tokens(source: &str) {
    let mut lexer = Lexer::new(source);
    loop { let token = lexer.next_token(); println!("{}:{}  {:?}", token.line, token.column, token.kind); if matches!(token.kind, falconcore::lexer::TokenType::Eof) { break; } }
}

fn value_after_flag(args: &[String], flag: &str) -> Result<String, String> {
    args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1)).cloned().ok_or_else(|| format!("missing value for {flag}\n\n{HELP}"))
}

fn required_arg(args: &[String], index: usize, usage: &str) -> Result<String, String> {
    args.get(index).cloned().ok_or_else(|| format!("missing argument; usage: {usage}\n\n{HELP}"))
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") { print!("{HELP}"); return; }
    if args.iter().any(|a| a == "--version" || a == "-V") { println!("FalconCore {VERSION}"); return; }

    let result: Result<(), String> = match args[0].as_str() {
        "run" => required_arg(&args, 1, "falconcore run <SOURCE.falcon>").and_then(|p| fs::read_to_string(&p).map_err(|e| format!("cannot read {p}: {e}")).and_then(|s| run_source(&s))),
        "build" => required_arg(&args, 1, "falconcore build <SOURCE.falcon> [OUTPUT.fbc]").and_then(|p| { let out = args.get(2).map(PathBuf::from); build_file(Path::new(&p), out.as_deref()).map(|_| ()) }),
        "run-fbc" => required_arg(&args, 1, "falconcore run-fbc <FILE.fbc>").and_then(|p| run_fbc(Path::new(&p))),
        "inspect" => required_arg(&args, 1, "falconcore inspect <FILE.fbc>").and_then(|p| inspect_fbc(Path::new(&p))),
        "check" => required_arg(&args, 1, "falconcore check <SOURCE.falcon>").and_then(|p| check_file_with_modules(Path::new(&p)).map(|_| ())),
        "eval" => required_arg(&args, 1, "falconcore eval <SOURCE>").and_then(|s| run_source(&s)),
        "tokens" => required_arg(&args, 1, "falconcore tokens <SOURCE>").map(|s| print_tokens(&s)),
        "ast" => required_arg(&args, 1, "falconcore ast <SOURCE>").map(|s| { let (ast, _) = parse_source(&s); println!("{ast:#?}"); }),
        "bytecode" => required_arg(&args, 1, "falconcore bytecode <SOURCE>").map(|s| { let (_, c) = parse_source(&s); print!("{}", disassemble(c.get_constants(), c.get_code())); }),
        _ => {
            if args.iter().any(|a| a == "--tokens") { value_after_flag(&args, "--tokens").map(|s| print_tokens(&s)) }
            else if args.iter().any(|a| a == "--ast") { value_after_flag(&args, "--ast").map(|s| { let (ast, _) = parse_source(&s); println!("{ast:#?}"); }) }
            else if args.iter().any(|a| a == "--bytecode") { value_after_flag(&args, "--bytecode").map(|s| { let (_, c) = parse_source(&s); print!("{}", disassemble(c.get_constants(), c.get_code())); }) }
            else if args.iter().any(|a| a == "--check") { value_after_flag(&args, "--check").and_then(|s| check_source(&s)) }
            else if args.iter().any(|a| a == "--check-file") { value_after_flag(&args, "--check-file").and_then(|p| fs::read_to_string(&p).map_err(|e| format!("cannot read {p}: {e}")).and_then(|s| check_source(&s))) }
            else if args.iter().any(|a| a == "--eval") { value_after_flag(&args, "--eval").and_then(|s| run_source(&s)) }
            else if args.iter().any(|a| a == "--file") { value_after_flag(&args, "--file").and_then(|p| fs::read_to_string(&p).map_err(|e| format!("cannot read {p}: {e}")).and_then(|s| run_source(&s))) }
            else { Err(format!("unknown command or option\n\n{HELP}")) }
        }
    };
    if let Err(message) = result { eprintln!("{message}"); process::exit(2); }
}
