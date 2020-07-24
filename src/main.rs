use std::any::Any;

#[derive(Clone)]
enum TokenType {
    Operation,
    Special,
    Number,
    Symbol,
    Assignment,
    Unknown,
}

#[derive(Clone)]
struct Token {
    typ: TokenType,
    value: String,
}

struct Lexer {
    text: String,
    length: usize,
    index: usize,
}

impl Lexer {
    pub fn new(text: &str) -> Self {
        let length = text.len();
        Self {
            text: text.to_string(),
            length,
            index: 0,
        }
    }

    fn peek_char(&mut self) -> char {
        self.text.as_bytes()[self.index] as char
    }

    fn eat_char(&mut self) -> Option<char> {
        if self.index >= self.length {
            return None;
        }

        let current_char = self.peek_char();
        self.index += 1;
        Some(current_char)
    }

    fn eat_number(&mut self) -> Option<String> {
        if self.index >= self.length {
            return None;
        }

        let start_index = self.index;

        let mut current_char = self.peek_char();
        loop {
            match current_char {
                '0'..='9' | '.' => self.index += 1,
                _ => break,
            }
            current_char = self.peek_char();
        }

        Some(self.text[start_index..self.index].to_string())
    }

    fn eat_string(&mut self) -> Option<String> {
        if self.index >= self.length {
            return None;
        }

        let start_index = self.index;

        let mut current_char = self.peek_char();
        loop {
            match current_char {
                'A'..='Z' | 'a'..='z' | '0'..='9' => self.index += 1,
                _ => break,
            }
            current_char = self.peek_char();
        }

        Some(self.text[start_index..self.index].to_string())
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.length {
            return None;
        }
        let char_eaten = self.eat_char();
        match char_eaten {
            Some(c) => {
                match c {
                    ' ' | '\n' => self.next(),
                    '+' | '-' | '/' | '*' => Some(Token { typ: TokenType::Operation, value: c.to_string() }),
                    ';' | ':' | '=' => Some(Token { typ: TokenType::Special, value: c.to_string() }),
                    '0'..='9' | '.' => {
                        self.index -= 1;
                        let value = self.eat_number();
                        match value {
                            Some(x) => Some(Token { typ: TokenType::Number, value: x }),
                            None => None,
                        }
                    }
                    'A'..='Z' | 'a'..='z' => {
                        self.index -= 1;
                        let value = self.eat_string();
                        match value {
                            Some(x) => Some(Token { typ: TokenType::Symbol, value: x }),
                            None => None,
                        }
                    }
                    _ => None,
                }
            }
            None => None,
        }
    }
}

struct Tree {
    token: Token,
    left: TreeNode,
    right: TreeNode,
}

impl Tree {
    fn rotate_left(self) -> Tree {
        match self.right {
            None => self,
            Some(mut y) => {
                Self {
                    token: y.token,
                    left: Some(Box::new(Self {
                        token: self.token,
                        left: self.left,
                        right: y.left.take(),
                    })),
                    right: y.right.take(),
                }
            }
        }
    }

    fn get_subtree_string(tree: &TreeNode, tab_str: &String, tabs: usize) -> String {
        match tree {
            Some(x) => format!("{0}{1}", tab_str, x.get_tree_string(tabs)),
            None => "~END~".to_string(),
        }
    }

    fn get_tree_string(&self, tabs: usize) -> String {
        let mut tab_str = "".to_string();
        for _ in (0..tabs).step_by(1) {
            tab_str.push_str("  ");
        }
        let left = Tree::get_subtree_string(&self.left, &tab_str, tabs + 2);
        let right = Tree::get_subtree_string(&self.right, &tab_str, tabs + 2);

        format!("\n{3}{0}: \n  {3}left: {1}\n  {3}right: {2}",
                self.token.value,
                left,
                right,
                tab_str)
    }
}

impl ToString for Tree {
    fn to_string(&self) -> String {
        self.get_tree_string(0)
    }
}

type TreeNode = Option<Box<Tree>>;

struct Parser {
    tokens: Vec<Token>,
    length: usize,
    index: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        let length = tokens.len();
        Self {
            tokens,
            length,
            index: 0,
        }
    }

    fn get_operation_priority(token: &Token) -> usize {
        let c = token.value.as_bytes()[0] as char;
        match c {
            '+' | '=' => 1,
            '*' | '/' => 2,
            _ => 0,
        }
    }

    fn eat_token(&mut self) -> Option<Token> {
        if self.index >= self.length {
            return None;
        }
        let token = self.tokens[self.index].clone();
        self.index += 1;
        Some(token)
    }

    fn next_expression(&mut self, prev: TreeNode) -> TreeNode {
        let token = match self.eat_token() {
            Some(x) => x,
            None => return None,
        };

        let type_id = token.typ.type_id();
        if prev.is_none() && (type_id == TokenType::Number.type_id() || type_id == TokenType::Symbol.type_id()) {
            let simple_node = Tree { token, left: None, right: None };
            return self.next_expression(Some(Box::new(simple_node)));
        };

        let token_value = token.value.clone();

        let result_tree = match token.typ {
            TokenType::Special => {
                let c = token.value.as_bytes()[0] as char;
                match c {
                    ';' => prev,
                    '=' => {
                        let prev_token = match &prev {
                            Some(x) => x.token.clone(),
                            None => Token { typ: TokenType::Unknown, value: "?".to_string() },
                        };

                        if prev_token.typ.type_id() != TokenType::Symbol.type_id() {
                            println!("Error: assignment: {} is not a valid symbol", prev_token.value);
                            None
                        } else {
                            Some(Box::new(Tree {
                                token: Token { typ: TokenType::Assignment, value: token.value },
                                left: prev,
                                right: self.next_expression(None),
                            }))
                        }
                    }
                    _ => {
                        println!("Error: special: found an unknown character");
                        None
                    }
                }
            }

            TokenType::Operation => {
                let operation_subtree = self.next_expression(None);
                let operation_tree = Tree { token, left: prev, right: operation_subtree };

                match &operation_tree.right {
                    Some(x) => {
                        let priority = Parser::get_operation_priority(&operation_tree.token);
                        let subtree_priority = Parser::get_operation_priority(&x.token);
                        if priority > 0 && subtree_priority > 0 && priority > subtree_priority {
                            Some(Box::new(operation_tree.rotate_left()))
                        } else {
                            Some(Box::new(operation_tree))
                        }
                    }
                    None => Some(Box::new(operation_tree)),
                }
            }
            _ => {
                println!("Error: found an unknown token");
                None
            }
        };

        if result_tree.is_none() {
            println!("Error: on token: {}", token_value);
        }
        result_tree
    }
}

impl Iterator for Parser {
    type Item = Box<Tree>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_expression(None)
    }
}

fn main() {
    let lexer = Lexer::new("asd = 3 * 6 + 123 * 55;");
    let parser = Parser::new(lexer.collect());
    for tree in parser {
        println!("{}", tree.to_string());
    }
}
