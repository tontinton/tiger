use std::collections::HashMap;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::types::Type;

mod token;
mod lexer;
mod ast;
mod parser;
mod types;

fn main() {
    let lexer = Lexer::new("let asd : u32 = 3 + 123 * 55; if asd * 5 >= 10 + 2 { asd = asd * 2; } else if asd * 2 > 3 { asd = asd + 1; }");

    let mut types: HashMap<&str, Type> = HashMap::new();
    types.insert("s8", Type::Number(1, false));
    types.insert("u8", Type::Number(1, true));
    types.insert("s16", Type::Number(2, false));
    types.insert("u16", Type::Number(2, true));
    types.insert("s32", Type::Number(4, false));
    types.insert("u32", Type::Number(4, true));

    let mut variables: HashMap<&str, Type> = HashMap::new();

    let mut parser = Parser::new(lexer.collect(), &mut types, &mut variables);

    for tree in parser.collect() {
        println!("{}", tree.to_string());
    }
}
