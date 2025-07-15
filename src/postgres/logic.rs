use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Logic {
    And,
    Or,
}

impl std::fmt::Display for Logic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let condition = match self {
            Self::And => "AND",
            Self::Or => "OR",
        };
        write!(f, "{condition}",)
    }
}
