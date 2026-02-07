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
// src/repl.rs - FalconCore REPL (Multi-line + History)
use std::io::{self, BufRead, Write};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::compiler::Compiler;
use crate::vm::VM;

pub fn start_repl() {
    println!("FalconCore REPL v0.1");
    println!("Multi-line code supported. Type 'exit' or Ctrl+D to quit");
    println!("Type code and press Enter (multi-line: end with empty line)");

    let stdin = io::stdin();
    let mut history: Vec<String> = vec![];

    loop {
        print!("falcon> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let mut line_count = 0;

        loop {
            let mut line = String::new();
            match stdin.read_line(&mut line) {
                Ok(0) => return, // EOF
                Ok(_) => {}
                Err(_) => return,
            }

            let trimmed = line.trim();
            if trimmed.is_empty() && line_count > 0 {
                break; // end multi-line
            }

            if trimmed == "exit" {
                println!("Goodbye!");
                return;
            }

            input.push_str(&line);
            line_count += 1;
        }

        if input.trim().is_empty() {
            continue;
        }

        history.push(input.clone());

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse();

        let mut compiler = Compiler::new();
        compiler.compile(ast);

        let mut vm = VM::new(compiler.get_constants().clone(), compiler.get_code().clone());
        vm.run();

        println!();
    }
        }
