// src/repl.rs - FalconCore REPL (Read-Eval-Print Loop)
use std::io::{self, BufRead};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::compiler::Compiler;
use crate::vm::VM;

pub fn start_repl() {
    println!("FalconCore REPL v0.1");
    println!("Type Falcon code and press Enter (type 'exit' to quit)");

    let stdin = io::stdin();
    for line in stdin.lines() {
        let input = line.unwrap();
        if input.trim() == "exit" {
            println!("Goodbye!");
            break;
        }

        if input.trim().is_empty() {
            continue;
        }

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse();

        let mut compiler = Compiler::new();
        compiler.compile(ast);

        let mut vm = VM::new(compiler.get_constants().clone(), compiler.get_code().clone());
        vm.run();

        println!("> ");
    }
}
