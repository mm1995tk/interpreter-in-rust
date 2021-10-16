use interpreter_in_rust::{lexer::Lexer};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let statement = args.get(1);

    let mut lexer = Lexer::new(statement.unwrap());
    for _ in 0..statement.unwrap().len()+1 {
        let token = lexer.next_token();
        println!("{:?}", token)
    }
}
