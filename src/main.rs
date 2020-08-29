use typed_arena::Arena;

use crate::lexer::Lexer;
use crate::parser::Parser;

mod token;
mod lexer;
mod ast;
mod parser;

fn main() {
    let arena = Arena::new();
    let mut lexer = Lexer::new("asd = 3 + 123 * 55; if asd * 5 >= 10 + 2 { asd = asd * 2; } else if asd * 2 > 3 { asd = asd + 1; }");
    let parser = Parser::new(&mut lexer, &arena);
    if let Some(expression) = parser.parse() {
        println!("{}", expression.to_string());
    }
}
