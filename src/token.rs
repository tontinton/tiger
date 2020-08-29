#[derive(Clone)]
pub enum TokenType {
    Operation,
    Special,
    Number,
    Symbol,
    If,
    Else,
    Assignment,
    Colon,
    Let,
    Walrus,
}

#[derive(Clone)]
pub struct Token {
    pub typ: TokenType,
    pub value: String,
}
