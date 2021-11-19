use interpreter_in_rust::lexer::Lexer;
use interpreter_in_rust::token::Token;
use std::io::Write;

fn main() {
    println!("\nWelcome to Monkey Language!");

    loop {
        let mut input = String::new();
        print!(">> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let mut lexer = Lexer::new(&input);
        println!();
        loop {
            let token = lexer.next_token();
            if token == Token::EOF {
                break;
            }
            println!("{}", &token);
        }
        println!();
    }
}
