use colored::Colorize;
use lc3_language_server::lexer::Lexer;
use lc3_language_server::parser::Parser;
use lc3_language_server::passes;
use std::{env, fs};

fn main() {
    if let Some(file_name) = env::args().nth(1) {
        let file_text = fs::read_to_string(&file_name).unwrap();
        let file_lines = file_text.split("\n").collect::<Vec<&str>>();

        let mut lexer = Lexer::from_text(&file_text);
        let tokens = lexer.analyze();

        let mut parser = Parser::new(tokens);
        let mut nodes = parser.parse_ast();

        passes::verify_labels(&mut nodes);

        for node in &nodes {
            for error in &node.errors {
                println!("{}: {}", "error".red().bold(), error.bold());
                println!(
                    "{}:{}:{}\n",
                    file_name, node.start_loc.line, node.start_loc.col
                );
                println!("\t{}", file_lines[node.start_loc.line - 1]);
                println!(
                    "\t{}{}",
                    " ".repeat(node.start_loc.col - 1),
                    "^".repeat(node.end_loc.col - node.start_loc.col + 1).red()
                );
                println!("");
            }
        }
    }
}
