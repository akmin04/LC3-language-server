use lc3_language_server::lexer::Lexer;
use lc3_language_server::parser::Parser;
use std::env;

fn main() {
    if let Some(file) = env::args().nth(1) {
        let mut lexer = Lexer::from_file(&file).unwrap();
        let tokens = lexer.analyze();

        // println!("{:#?}", tokens);

        let mut parser = Parser::new(tokens);
        let nodes = parser.parse_ast();
        // println!("{:#?}", nodes);

        println!("------------------");

        for node in &nodes {
            if let Some(errors) = &node.errors {
                println!("Line {}:{}", node.start_loc.line, node.start_loc.col);
                for error in errors {
                    println!("\t{}", error);
                }
            }
        }
    }
}
