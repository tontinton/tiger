#[macro_use]
extern crate lazy_static;

use std::path::Path;

use clap::{App, Arg, crate_authors, crate_name, crate_version};

use crate::ast::Expression;
use crate::lexer::Lexer;
use crate::parser::Parser;


// use crate::typing::TypeInference;

mod ast;
mod lexer;
mod parser;
mod token;
mod types;
// mod typing;

fn main() -> Result<(), String> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about("The tiger language compiler")
        .author(crate_authors!())
        .arg(
            Arg::with_name("input")
                .index(1)
                .required(true),
        )
        .get_matches();

    let input_file = matches.value_of("input").unwrap();

    let ast = match Lexer::from_file(Path::new(input_file)) {
        Ok(mut lexer) => {
            let parser = Parser::new(&mut lexer);
            match parser.parse() {
                Ok(expression) => Ok(expression),
                Err(e) => Err(format!("{}", e)),
            }
        }
        Err(e) => Err(format!("{}", e)),
    }?;

    println!("{}", Expression::from_index_to_string(&ast));

    // let type_inference = TypeInference::new();
    // type_inference.infer_ast_of_file(&ast)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_test_file() -> Result<(), String> {
        let input_file = "test.tg";
        let arena = Arena::new();
        match Lexer::from_file(Path::new(input_file)) {
            Ok(mut lexer) => {
                let parser = Parser::new(&mut lexer, &arena, arena.alloc(Expression::Empty));
                match parser.parse() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("{}", e)),
                }
            }
            Err(e) => Err(format!("{}", e)),
        }
    }
}
