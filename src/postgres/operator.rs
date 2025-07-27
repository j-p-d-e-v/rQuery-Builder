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
    NotNull, // Value is NOT NULL,

    // Range
    Between,

    // Reference: https://neon.com/postgresql/postgresql-json-functions/postgresql-jsonb-operators
    JsonbValue,       // ->
    JsonbValueAsText, // ->>
    JsonbContains,    // @>
    JsonbContained,   // <@
    JsonbHasKey,      // ?
    JsonbHasAnyKeys,  // |?
    JsonbHasAllKeys,  // ?&
    JsonbConcatenate, // ||
    JsonbRemoveKey,   // -
    JsonbRemovePath,  // #-
    JsonbHasPath,     // @?
    JsonbPathExists,  // @@
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
            Self::Between => "BETWEEN",
            //JSONB Operators
            Self::JsonbValue => "->",
            Self::JsonbValueAsText => "->>",
            Self::JsonbContains => "@>",
            Self::JsonbContained => "<@",
            Self::JsonbHasKey => "?",
            Self::JsonbHasAnyKeys => "?|",
            Self::JsonbHasAllKeys => "?&",
            Self::JsonbConcatenate => "||",
            Self::JsonbRemoveKey => "-",
            Self::JsonbRemovePath => "#-",
            Self::JsonbHasPath => "@?",
            Self::JsonbPathExists => "@@",
        };
        write!(f, "{operator}")
    }
}
