use interpreter_in_rust::lexer::Lexer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    for arg in &args[1..] {
        let mut lexer = Lexer::new(arg);
        for _ in 0..arg.len() + 1 {
            let token = lexer.next_token();
            println!("{:?}", token)
        }
    }
}
