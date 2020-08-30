use std::path::Path;

use clap::{App, Arg, crate_authors, crate_name, crate_version};
use typed_arena::Arena;

use crate::ast::Expression;
use crate::lexer::Lexer;
use crate::parser::Parser;

mod token;
mod lexer;
mod ast;
mod parser;

fn parse(input_file_path: &str) -> Result<String, String> {
    let arena = Arena::new();
    match Lexer::from_file(Path::new(input_file_path)) {
        Ok(mut lexer) => {
            let parser = Parser::new(&mut lexer, &arena, arena.alloc(Expression::Empty));
            match parser.parse() {
                Ok(expression) => Ok(expression.to_string()),
                Err(e) => Err(format!("{}", e))
            }
        }
        Err(e) => Err(format!("Failed to read file: {}: {}", input_file_path, e))
    }
}

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
    println!("{}", match parse(input_file) {
        Ok(parsed_ast) => parsed_ast,
        Err(e) => e
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_test_file() -> Result<(), String> {
        let _result = parse("test.tg")?;
        Ok(())
    }
}
