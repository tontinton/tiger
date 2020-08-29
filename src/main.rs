use std::path::Path;

use typed_arena::Arena;

use crate::lexer::Lexer;
use crate::parser::Parser;

mod token;
mod lexer;
mod ast;
mod parser;

const FIRST_FILE_TO_PARSE: &str = "test.tg";

fn main() {
    let arena = Arena::new();
    match Lexer::from_file(Path::new(FIRST_FILE_TO_PARSE)) {
        Ok(mut lexer) => {
            let parser = Parser::new(&mut lexer, &arena);
            if let Some(expression) = parser.parse() {
                println!("{}", expression.to_string());
            }
        }
        Err(e) => println!("Failed to read file: {}: {}", FIRST_FILE_TO_PARSE, e)
    }
}
