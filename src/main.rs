use std::any::Any;

#[derive(Clone)]
enum TokenType {
    Operation,
    Special,
    Number,
    String,
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
                            Some(x) => Some(Token { typ: TokenType::String, value: x }),
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

fn get_subtree_string(tree: &TreeNode, tab_str: &String, tabs: usize) -> String {
    match tree {
        Some(x) => format!("{0}{1}", tab_str, x.get_tree_string(tabs)),
        None => "~END~".to_string(),
    }
}

impl Tree {
    fn get_tree_string(&self, tabs: usize) -> String {
        let mut tab_str = "".to_string();
        for _ in (0..tabs).step_by(1) {
            tab_str.push_str("  ");
        }
        let left = get_subtree_string(&self.left, &tab_str, tabs + 2);
        let right = get_subtree_string(&self.right, &tab_str, tabs + 2);

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
    pub fn new(tokens: Vec<Token>) -> Self {
        let length = tokens.len();
        Self {
            tokens,
            length,
            index: 0,
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
        if prev.is_none() && (type_id == TokenType::Number.type_id() || type_id == TokenType::String.type_id()) {
            let simple_node = Tree { token, left: None, right: None };
            return self.next_expression(Some(Box::new(simple_node)));
        };

        match token.typ {
            TokenType::Special => {
                let c = token.value.as_bytes()[0] as char;
                match c {
                    ';' => prev,
                    '=' => {
                        let next = self.next_expression(None);
                        let assignment_node = Tree { token, left: prev, right: next };
                        Some(Box::new(assignment_node))
                    }
                    _ => None,
                }
            }

            TokenType::Operation => {
                let next = self.next_expression(None);
                let operation_node = Tree { token, left: prev, right: next };
                Some(Box::new(operation_node))
            }

            _ => {
                println!("PARSING ERROR");
                None
            }
        }
    }
}

impl Iterator for Parser {
    type Item = Box<Tree>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_expression(None)
    }
}

fn main() {
    let lexer = Lexer::new("3 + 5.5 * 15 + 123;");
    let parser = Parser::new(lexer.collect());
    for tree in parser {
        println!("{}", tree.to_string());
    }
}
