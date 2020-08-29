pub struct ParserScope {
    pub variables: Vec<String>,
}

impl ParserScope {
    pub fn new() -> Self {
        Self {
            variables: vec![]
        }
    }
}
