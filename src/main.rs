use std::path::Path;

use clap::{App, Arg, crate_authors, crate_name, crate_version};
use typed_arena::Arena;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::parser_scope::ParserScope;

mod token;
mod lexer;
mod ast;
mod parser_scope;
mod parser;


fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about("The tiger language compiler")
        .author(crate_authors!())
        .arg(
            Arg::with_name("input")
                .about("the input file to use")
                .index(1)
                .required(true),
        )
        .get_matches();

    let input_file = matches.value_of("input").unwrap();
    let arena = Arena::new();
    match Lexer::from_file(Path::new(input_file)) {
        Ok(mut lexer) => {
            let mut scope = ParserScope::new();
            let parser = Parser::new(&mut lexer, &mut scope, &arena);
            match parser.parse() {
                Ok(expression) => {
                    println!("{}", expression.to_string());
                }
                Err(e) => println!("Parse error: {}", e)
            }
        }
        Err(e) => println!("Failed to read file: {}: {}", input_file, e)
    }
}
