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
// src/repl.rs - FalconCore REPL (Enhanced: multi-line + basic history navigation)
use std::io::{self, BufRead, Write};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::compiler::Compiler;
use crate::vm::VM;

pub fn start_repl() {
    println!("FalconCore REPL v0.1 - Multi-line + History");
    println!("Type code (multi-line: end with empty line), 'exit' to quit, 'history' to see past commands");

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

            let trimmed = line.trim().to_string();

            if trimmed == "exit" {
                println!("Goodbye!");
                return;
            }

            if trimmed == "history" {
                println!("History:");
                for (i, cmd) in history.iter().enumerate() {
                    println!("{:3}: {}", i + 1, cmd);
                }
                continue;
            }

            if trimmed.is_empty() && line_count > 0 {
                break; // end multi-line input
            }

            input.push_str(&line);
            line_count += 1;
        }

        if input.trim().is_empty() {
            continue;
        }

        history.push(input.trim().to_string());

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
// src/repl.rs - FalconCore REPL (Enhanced: multi-line, history navigation placeholder, basic syntax highlight)
use std::io::{self, BufRead, Write};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::compiler::Compiler;
use crate::vm::VM;

pub fn start_repl() {
    println!("FalconCore REPL v0.1 - Multi-line + History + Basic Syntax Highlight");
    println!("Type code (multi-line: end with empty line), 'exit' to quit, 'history' to see past commands");

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

            let trimmed = line.trim().to_string();

            if trimmed == "exit" {
                println!("Goodbye!");
                return;
            }

            if trimmed == "history" {
                println!("History:");
                for (i, cmd) in history.iter().enumerate() {
                    println!("{:3}: {}", i + 1, cmd);
                }
                continue;
            }

            if trimmed.is_empty() && line_count > 0 {
                break; // end multi-line input
            }

            // Basic syntax highlight placeholder (just color keywords in terminal)
            let highlighted = highlight_syntax(&line);
            print!("{}", highlighted);

            input.push_str(&line);
            line_count += 1;
        }

        if input.trim().is_empty() {
            continue;
        }

        history.push(input.trim().to_string());

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

// Basic syntax highlight (terminal colors)
fn highlight_syntax(line: &str) -> String {
    let keywords = vec!["secure", "let", "const", "fn", "return", "if", "else", "repeat", "print", "network", "scan", "crypto", "random", "time", "now", "wait"];
    let mut highlighted = line.to_string();

    for kw in keywords {
        highlighted = highlighted.replace(kw, &format!("\x1b[1;34m{}\x1b[0m", kw));
    }

    highlighted.replace("print", "\x1b[1;32mprint\x1b[0m")
        .replace("\"", "\x1b[1;33m\"\x1b[0m")
        .replace("=", "\x1b[1;31m=\x1b[0m")
    }
