use crate::token::Token;

#[derive(Clone)]
pub enum Expression {
    Literal(Token),
    Operation(Box<Expression>, Token, Box<Expression>),
    IfElse(Box<Expression>, Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn get_token(&self) -> Option<Token> {
        match self {
            Expression::Literal(token) => Some(token.clone()),
            Expression::Operation(_left, token, _right) => Some(token.clone()),
            _ => None,
        }
    }

    fn get_tree_string(&self, tabs: usize) -> String {
        match self {
            Expression::Literal(token) => token.value.clone(),
            Expression::Operation(left, token, right) => {
                let mut tab_str = "".to_string();
                for _ in (0..tabs).step_by(1) {
                    tab_str.push_str("  ");
                }

                let left = format!("{}", left.get_tree_string(tabs + 2));
                let right = format!("{}", right.get_tree_string(tabs + 2));

                format!("\n{3}{0}: \n  {3}left: {1}\n  {3}right: {2}",
                        token.value,
                        left,
                        right,
                        tab_str)
            }
            _ => { "".to_string() }
        }
    }
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        self.get_tree_string(0)
    }
}
