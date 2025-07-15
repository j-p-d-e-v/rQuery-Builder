use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Operator {
    // Equality
    Eq,  // Equal (=)
    Neq, // Not Equal (!=)

    // Comparison
    Gt,  // Greater Than (>)
    Gte, // Greater Than or Equal (>=)
    Lt,  // Less Than (<)
    Lte, // Less Than or Equal (<=)

    // Pattern Matching
    Like, // Case-sensitive pattern match (LIKE)

    // List/Array Operations
    In,    // Value is in a list of items (IN)
    NotIn, // Value is not in a list (NOT IN)

    // Null Checks
    IsNull,  // Value is NULL
    NotNull, // Value is NOT NULL
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operator = match self {
            Self::Eq => "=",
            Self::Neq => "!=",
            Self::Gt => ">",
            Self::Gte => ">=",
            Self::Lt => "<",
            Self::Lte => "<=",
            Self::Like => "LIKE",
            Self::In => "IN",
            Self::NotIn => "NOT IN",
            Self::IsNull => "IS NULL",
            Self::NotNull => "IS NOT NULL",
        };
        write!(f, "{operator}")
    }
}
