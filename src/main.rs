use falconcore::lexer::{Lexer, TokenType};

fn main() {
    println!("FalconCore v0.1 - Lexer Test");

    let code = r#"
    secure let x = 42
    print "Hello from FalconCore!"
    if x > 10 {
        print "x is greater than 10"
    }
    "#;

    let mut lexer = Lexer::new(code);
    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token.kind == TokenType::Eof {
            break;
        }
    }

    println!("Lexer test complete!");
}
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    println!("FalconCore v0.1 - Lexer + Parser Test");

    let code = r#"
        secure let x = 42
        secure let y = "Hello from FalconCore!"
        print x + 8
        print y
    "#;

    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);

    let ast = parser.parse();

    println!("AST:");
    for node in ast {
        println!("{:?}", node);
    }

    println!("Parsing complete!");
}
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    println!("FalconCore v0.1 - Lexer + Parser Test");

    let code = r#"
        secure let x = 42
        secure const pi = 3.14
        print "Hello from FalconCore!"
        if x > 10 {
            print "x is greater than 10"
        } else {
            print "x is small"
        }
    "#;

    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);

    let ast = parser.parse();

    println!("AST:");
    for node in ast {
        println!("{:#?}", node);
    }

    println!("Parsing complete!");
}
