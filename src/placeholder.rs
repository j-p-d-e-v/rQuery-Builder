/// Represents the placeholder style used in value bindings for generated SQL.
///
/// - `QuestionMark` produces `?` placeholders (used by SQLite, MySQL, etc.).
/// - `DollarSequential` produces `$1`, `$2`, ... placeholders (used by PostgreSQL).
#[derive(Debug, Clone)]
pub enum PlaceholderKind {
    QuestionMark,     // Using the ? symbol as placeholder for values.
    DollarSequential, //Using $1, $2, $3... as placeholder for values.
}

impl Default for PlaceholderKind {
    fn default() -> Self {
        Self::QuestionMark
    }
}
