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
