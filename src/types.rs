use std::fmt;

#[derive(Debug, Clone)]
pub enum Type {
    Auto,
    Undetermined { name: String },
    Number { size: usize, signed: bool },
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Type::Auto => "auto".to_string(),
            Type::Undetermined { name } => format!("undetermined: {}", name),
            Type::Number { size, signed } => format!(
                "{} number of size: {}",
                if *signed { "signed" } else { "unsigned" },
                size
            ),
        };

        write!(f, "{}", str)
    }
}
