#[derive(Clone, Debug)]
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
    Func,
    Return,
    SmallArrow,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub typ: TokenType,
    pub value: String,
}
