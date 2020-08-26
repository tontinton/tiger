#[derive(Clone)]
pub enum TokenType {
    Operation,
    Special,
    Number,
    Symbol,
    If,
    Else,
    Let,
    Colon,
    Assignment,
}

#[derive(Clone)]
pub struct Token {
    pub typ: TokenType,
    pub value: String,
}
