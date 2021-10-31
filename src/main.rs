use interpreter_in_rust::lexer::Lexer;
use interpreter_in_rust::token::TokenType::EOF;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    for arg in &args[1..] {
        let mut lexer = Lexer::new(arg);
        for _ in 0..arg.len() + 1 {
            let token = lexer.next_token();
            match token {
                Ok(ok_token) if ok_token.token_type == EOF => {
                    if ok_token.token_type == EOF {
                        break;
                    }
                }
                Ok(ok_token) => {
                    println!("{:?}", ok_token)
                }
                Err(_) => panic!(),
            }
        }
    }
}
