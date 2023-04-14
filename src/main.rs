use lc3_language_server::lexer::Lexer;
use std::env;

fn main() {
    if let Some(file) = env::args().nth(1) {
        let mut lexer = Lexer::from_file(&file).unwrap();
        let tokens = lexer.analyze();

        println!("{:#?}", tokens);
    }
}
