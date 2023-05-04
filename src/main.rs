use colored::Colorize;
use lc3_language_server::lexer::Lexer;
use lc3_language_server::parser::Parser;
use lc3_language_server::passes;
use std::{env, fs, process};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Expected file name");
        process::exit(1);
    }

    let file_name = &args[1];
    let file_text = fs::read_to_string(&file_name).unwrap();
    let file_lines = file_text.split("\n").collect::<Vec<&str>>();

    let mut lexer = Lexer::from_text(&file_text);

    let tokens = lexer.analyze();

    if args.contains(&"--print-tokens".to_owned()) {
        println!("{:#?}", tokens);
    }

    let mut parser = Parser::new(tokens);
    let mut nodes = parser.parse_ast();

    if args.contains(&"--print-ast".to_owned()) {
        println!("{:#?}", nodes);
    }

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
