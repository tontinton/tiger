use crate::lexer::Lexer;
use crate::parser::Parser;

mod token;
mod lexer;
mod ast;
mod parser;

fn main() {
    let lexer = Lexer::new("asd = 3 + 123 * 55; if asd * 5 >= 10 + 2 { asd = asd * 2; }");
    let parser = Parser::new(lexer.collect());
    for tree in parser {
        println!("{}", tree.to_string());
    }
}
