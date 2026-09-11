use falconcore::{bytecode::{deserialize,disassemble,serialize},compiler::Compiler,diagnostics::{format_diagnostics,Diagnostic},lexer::Lexer,linker::link_modules,module::ModuleResolver,parser::Parser,typecheck::TypeChecker,vm::VM,VERSION};
use std::{env,fs,path::{Path,PathBuf},process};
const HELP:&str=r#"FalconCore — security-minded programming language runtime

USAGE:
  falconcore <COMMAND> [OPTIONS]

COMMANDS:
  run <SOURCE.falcon>        Execute source and resolved modules
  build <SOURCE.falcon>      Compile source + resolved modules into SOURCE.fbc
  run-fbc <FILE.fbc>         Execute a compiled FCBC artifact
  inspect <FILE.fbc>         Inspect a compiled FCBC artifact
  check <SOURCE.falcon>      Type-check source + resolved modules
  eval <SOURCE>              Execute inline source
  tokens <SOURCE>             Print lexer output
  ast <SOURCE>                Print parsed AST
  bytecode <SOURCE>           Compile and disassemble inline source
  --version                   Print the compiler version
  --help                      Show this help

NAMESPACE EXAMPLE:
  import "lib/math" as math
  print math::add(2, 3)
"#;
fn parse_source(source:&str)->(Vec<falconcore::parser::Expr>,Compiler){let mut p=Parser::new(Lexer::new(source));let ast=p.parse();let mut c=Compiler::new();c.compile(ast.clone());(ast,c)}
fn compile_ast(ast:Vec<falconcore::parser::Expr>)->Compiler{let mut c=Compiler::new();c.compile(ast);c}
fn check_ast(ast:&[falconcore::parser::Expr],path:&str)->Result<(),String>{let mut c=TypeChecker::new();c.check(ast).map_err(|es|{let ds:Vec<_>=es.into_iter().map(|e|Diagnostic::error("E2001",e.message)).collect();format!("{path}:\n{}",format_diagnostics(&ds))})}
fn check_source(source:&str)->Result<(),String>{let(ast,_)=parse_source(source);check_ast(&ast,"<inline>")}
fn resolve_and_link(path:&Path)->Result<Vec<falconcore::parser::Expr>,String>{let mut r=ModuleResolver::new();let modules=r.resolve(path).map_err(|e|format!("module resolution error: {e:?}"))?;link_modules(&modules,path).map_err(|e|format!("link error: {e:?}"))}
fn check_file(path:&Path)->Result<Vec<falconcore::parser::Expr>,String>{let ast=resolve_and_link(path)?;check_ast(&ast,&path.display().to_string())?;Ok(ast)}
fn execute_ast(ast:Vec<falconcore::parser::Expr>)->Result<(),String>{let c=compile_ast(ast);VM::with_functions(c.get_constants().to_vec(),c.get_code().to_vec(),c.get_functions().clone()).run().map_err(|e|format!("runtime error: {e:?}"))}
fn run_file(path:&Path)->Result<(),String>{let ast=check_file(path)?;execute_ast(ast)}
fn build_file(path:&Path,output:Option<&Path>)->Result<PathBuf,String>{let ast=check_file(path)?;let c=compile_ast(ast);let bytes=serialize(c.get_constants(),c.get_code(),c.get_functions()).map_err(|e|format!("bytecode serialization error: {e}"))?;let out=output.map(PathBuf::from).unwrap_or_else(||path.with_extension("fbc"));fs::write(&out,bytes).map_err(|e|format!("cannot write {}: {e}",out.display()))?;println!("built {}",out.display());Ok(out)}
fn run_fbc(path:&Path)->Result<(),String>{let data=fs::read(path).map_err(|e|format!("cannot read {}: {e}",path.display()))?;let(c,code,functions)=deserialize(&data).map_err(|e|format!("bytecode error: {e}"))?;VM::with_functions(c,code,functions).run().map_err(|e|format!("runtime error: {e:?}"))}
fn inspect_fbc(path:&Path)->Result<(),String>{let data=fs::read(path).map_err(|e|format!("cannot read {}: {e}",path.display()))?;let(c,code,functions)=deserialize(&data).map_err(|e|format!("bytecode error: {e}"))?;println!("Artifact: {}",path.display());println!("Functions: {}",functions.len());for(n,i)in&functions{println!("  {n} @ {}({})",i.entry,i.params.join(", "));}print!("{}",disassemble(&c,&code));Ok(())}
fn print_tokens(source:&str){let mut l=Lexer::new(source);loop{let t=l.next_token();println!("{}:{}  {:?}",t.line,t.column,t.kind);if matches!(t.kind,falconcore::lexer::TokenType::Eof){break}}}
fn arg(args:&[String],i:usize,u:&str)->Result<String,String>{args.get(i).cloned().ok_or_else(||format!("missing argument; usage: {u}\n\n{HELP}"))}
fn flag(args:&[String],f:&str)->Result<String,String>{args.iter().position(|a|a==f).and_then(|i|args.get(i+1)).cloned().ok_or_else(||format!("missing value for {f}\n\n{HELP}"))}
fn main(){let a:Vec<String>=env::args().skip(1).collect();if a.is_empty()||a.iter().any(|x|x=="--help"||x=="-h"){print!("{HELP}");return}if a.iter().any(|x|x=="--version"||x=="-V"){println!("FalconCore {VERSION}");return}let r=match a[0].as_str(){"run"=>arg(&a,1,"falconcore run <SOURCE.falcon>").and_then(|p|run_file(Path::new(&p))),"build"=>arg(&a,1,"falconcore build <SOURCE.falcon> [OUTPUT.fbc]").and_then(|p|{let o=a.get(2).map(PathBuf::from);build_file(Path::new(&p),o.as_deref()).map(|_|())}),"run-fbc"=>arg(&a,1,"falconcore run-fbc <FILE.fbc>").and_then(|p|run_fbc(Path::new(&p))),"inspect"=>arg(&a,1,"falconcore inspect <FILE.fbc>").and_then(|p|inspect_fbc(Path::new(&p))),"check"=>arg(&a,1,"falconcore check <SOURCE.falcon>").and_then(|p|check_file(Path::new(&p)).map(|_|())),"eval"=>arg(&a,1,"falconcore eval <SOURCE>").and_then(|s|check_source(&s).and_then(|_|{let(ast,_)=parse_source(&s);execute_ast(ast)})),"tokens"=>arg(&a,1,"falconcore tokens <SOURCE>").map(|s|print_tokens(&s)),"ast"=>arg(&a,1,"falconcore ast <SOURCE>").map(|s|{let(ast,_)=parse_source(&s);println!("{ast:#?}")}),"bytecode"=>arg(&a,1,"falconcore bytecode <SOURCE>").map(|s|{let(_,c)=parse_source(&s);print!("{}",disassemble(c.get_constants(),c.get_code()))}),_=>if a.iter().any(|x|x=="--tokens"){flag(&a,"--tokens").map(|s|print_tokens(&s))}else if a.iter().any(|x|x=="--ast"){flag(&a,"--ast").map(|s|{let(ast,_)=parse_source(&s);println!("{ast:#?}")})}else if a.iter().any(|x|x=="--bytecode"){flag(&a,"--bytecode").map(|s|{let(_,c)=parse_source(&s);print!("{}",disassemble(c.get_constants(),c.get_code()))})}else if a.iter().any(|x|x=="--check"){flag(&a,"--check").and_then(|s|check_source(&s))}else if a.iter().any(|x|x=="--check-file"){flag(&a,"--check-file").and_then(|p|check_file(Path::new(&p)).map(|_|()))}else if a.iter().any(|x|x=="--eval"){flag(&a,"--eval").and_then(|s|check_source(&s).and_then(|_|{let(ast,_)=parse_source(&s);execute_ast(ast)}))}else if a.iter().any(|x|x=="--file"){flag(&a,"--file").and_then(|p|run_file(Path::new(&p)))}else{Err(format!("unknown command or option\n\n{HELP}"))};if let Err(m)=r{eprintln!("{m}");process::exit(2)}}
