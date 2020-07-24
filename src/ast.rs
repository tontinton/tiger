use crate::token::Token;

pub type TreeNode = Option<Box<Tree>>;

pub struct Tree {
    pub token: Token,
    pub left: TreeNode,
    pub right: TreeNode,
}

impl Tree {
    pub fn rotate_left(self) -> Tree {
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
