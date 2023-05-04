use colored::{Color, Colorize};
use lc3_language_server::ast::NodeError;
use lc3_language_server::lexer;
use lc3_language_server::parser;
use lc3_language_server::passes;
use std::{env, fs, process};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("erro: Expected file name");
        process::exit(1);
    }

    let file_name = &args[1];
    let file_text = fs::read_to_string(&file_name).unwrap();
    let file_lines = file_text.split("\n").collect::<Vec<&str>>();

    let tokens = lexer::analyze(&file_text);

    if args.contains(&"--print-tokens".to_owned()) {
        println!("{:#?}", tokens);
    }

    let mut nodes = parser::parse_ast(&tokens);

    if args.contains(&"--print-ast".to_owned()) {
        println!("{:#?}", nodes);
    }

    passes::verify_labels(&mut nodes);
    passes::verify_number_literals_within_range(&mut nodes);

    for node in &nodes {
        for error in &node.errors {
            let color = match error {
                NodeError::Error(_) => Color::Red,
                NodeError::Warning(_) => Color::Yellow,
            };

            match error {
                NodeError::Error(error) => println!("{}: {}", "error".red().bold(), error.bold()),
                NodeError::Warning(warning) => {
                    println!("{}: {}", "warning".yellow().bold(), warning.bold())
                }
            };

            println!(
                "{}:{}:{}\n",
                file_name, node.start_loc.line, node.start_loc.col
            );
            println!("\t{}", file_lines[node.start_loc.line - 1]);
            println!(
                "\t{}{}",
                " ".repeat(node.start_loc.col - 1),
                "^".repeat(node.end_loc.col - node.start_loc.col + 1)
                    .color(color)
            );
            println!("");
        }
    }
}
