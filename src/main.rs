use crate::lexer::Lexer;
use crate::parser::Parser;

mod token;
mod lexer;
mod ast;
mod parser;

fn main() {
    let lexer = Lexer::new("main { asd = 3 * 6 + 123 * 55; asd = asd * 2; }");
    let parser = Parser::new(lexer.collect());
    for tree in parser {
        println!("{}", tree.to_string());
    }
}
