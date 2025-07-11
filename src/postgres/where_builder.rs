use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Condition {
    And,
    Or,
}

impl std::fmt::Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let condition = match self {
            Self::And => "AND",
            Self::Or => "OR",
        };
        write!(f, "{condition}",)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WhereClause {
    pub expression: String,
    pub condition: Option<Condition>,
    pub values: Vec<Value>,
}

impl WhereClause {
    pub fn build(
        values: Vec<WhereClauseItem>,
        condition: Option<Condition>,
    ) -> anyhow::Result<WhereClause> {
        let mut data: WhereClause = WhereClause::default();
        for item in values {
            let mut expression = WhereClauseItem::format(&item)?;
            if let Some(condition) = item.condition {
                expression = format!("{condition} {expression}");
            }
            if data.expression.is_empty() {
                data.expression = expression;
            } else {
                data.expression = format!("{} {}", data.expression, expression)
            }
            if let Some(value) = item.value {
                data.values.push(value);
            }
        }
        data.condition = condition;
        Ok(data)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhereClauseItem {
    pub field: String,
    pub operator: Operator,
    pub value: Option<Value>,
    pub condition: Option<Condition>,
}

impl WhereClauseItem {
    pub fn new(data: WhereClauseItem) -> Self {
        Self { ..data }
    }

    pub fn bind(value: &Value) -> Option<String> {
        match value {
            Value::Array(_) => Some("(?)".to_string()),
            _ => Some("?".to_string()),
        }
    }

    pub fn format(clause: &WhereClauseItem) -> anyhow::Result<String> {
        let field = &clause.field;
        let operator = &clause.operator;

        if field.is_empty() {
            return Err(anyhow!("field is empty"));
        }

        let value: Option<String> = if let Some(value) = &clause.value {
            Self::bind(value)
        } else {
            None
        };
        if let Some(value) = value
            && operator != &Operator::IsNull
            && operator != &Operator::NotNull
        {
            Ok(format!("{field} {operator} {value}"))
        } else {
            Ok(format!("{field} {operator}"))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct WhereBuilder {
    pub statement: String,
    pub values: Vec<Value>, //For Binding
}

impl WhereBuilder {
    fn format(expression: String, condition: Option<Condition>, do_grouping: bool) -> String {
        let condition = if let Some(value) = condition {
            value.to_string()
        } else {
            "".to_string()
        };
        if do_grouping {
            format!("{condition} ({expression})")
        } else {
            expression
        }
    }

    pub fn build(values: Vec<WhereClause>) -> WhereBuilder {
        let mut data: WhereBuilder = WhereBuilder::default();
        let mut clauses: Vec<String> = Vec::new();
        let do_grouping = values.len() > 1;
        for mut item in values {
            let clause = Self::format(item.expression, item.condition, do_grouping);
            if !item.values.is_empty() {
                data.values.append(&mut item.values);
            }
            clauses.push(clause);
        }
        data.statement = format!("WHERE {}", clauses.join(" ").trim());
        data
    }
}

#[cfg(test)]
pub mod test_where_builder {
    use super::*;
    use serde_json::Number;

    #[tokio::test]
    async fn test_where_builder() {
        let data = Value::String("MYVALUE".to_string());
        let result = WhereClauseItem::bind(&data);
        assert_eq!(result, Some("?".to_string()));

        let data = Value::Number(Number::from_i128(128).unwrap());
        let result = WhereClauseItem::bind(&data);
        assert_eq!(result, Some("?".to_string()));

        let values = Value::Array(vec![
            Value::String("MYVALUE".to_string()),
            Value::Number(Number::from_i128(128).unwrap()),
        ]);
        let result = WhereClauseItem::bind(&values);
        assert_eq!(result, Some("(?)".to_string()));

        let where_clause = WhereClauseItem {
            field: "myfield1".to_string(),
            operator: Operator::Eq,
            value: Some(Value::String(String::from("MYVALUE"))),
            condition: None,
        };

        let result = WhereClauseItem::format(&where_clause);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = result.unwrap();
        assert_eq!(result, "myfield1 = ?".to_string());

        let where_clause = WhereClauseItem {
            field: "myfield1".to_string(),
            operator: Operator::In,
            value: Some(values.clone()),
            condition: None,
        };

        let result = WhereClauseItem::format(&where_clause);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = result.unwrap();
        assert_eq!(result, "myfield1 IN (?)".to_string());

        let where_clauses = vec![WhereClauseItem {
            field: "".to_string(),
            operator: Operator::Eq,
            value: Some(Value::String("MYVALUE".to_string())),
            condition: None,
        }];
        let clause_error = WhereClause::build(where_clauses, None);
        assert!(clause_error.is_err(), "expecting field error");

        let where_clauses = vec![
            WhereClauseItem {
                field: "myfield1".to_string(),
                operator: Operator::Eq,
                value: Some(Value::String("MYVALUE".to_string())),
                condition: None,
            },
            WhereClauseItem {
                field: "myfield2".to_string(),
                operator: Operator::Eq,
                value: Some(Value::Number(Number::from_i128(128).unwrap())),
                condition: Some(Condition::And),
            },
        ];
        let clause1 = WhereClause::build(where_clauses, None);
        assert!(clause1.is_ok(), "{:?}", clause1.err());
        let clause1 = clause1.unwrap();
        assert_eq!(
            clause1.expression,
            "myfield1 = ? AND myfield2 = ?".to_string()
        );
        assert_eq!(clause1.condition, None);
        assert!(clause1.values.len() > 0);

        let where_clauses = vec![
            WhereClauseItem {
                field: "myfield3".to_string(),
                operator: Operator::Eq,
                value: Some(Value::String("MYVALUE".to_string())),
                condition: None,
            },
            WhereClauseItem {
                field: "myfield4".to_string(),
                operator: Operator::In,
                value: Some(values),
                condition: Some(Condition::And),
            },
            WhereClauseItem {
                field: "myfield5".to_string(),
                operator: Operator::IsNull,
                value: None,
                condition: Some(Condition::Or),
            },
        ];
        let clause2 = WhereClause::build(where_clauses, Some(Condition::And));
        assert!(clause2.is_ok(), "{:?}", clause2.err());
        let clause2 = clause2.unwrap();
        assert_eq!(
            clause2.expression,
            "myfield3 = ? AND myfield4 IN (?) OR myfield5 IS NULL".to_string()
        );
        assert_eq!(clause2.condition, Some(Condition::And));
        assert!(clause2.values.len() > 0);

        let where1 = WhereBuilder::build(vec![clause1, clause2.clone()]);
        assert_eq!(where1.statement,"WHERE (myfield1 = ? AND myfield2 = ?) AND (myfield3 = ? AND myfield4 IN (?) OR myfield5 IS NULL)".to_string());
        assert!(where1.values.len() > 0);
        let where2 = WhereBuilder::build(vec![clause2]);
        assert_eq!(
            where2.statement,
            "WHERE myfield3 = ? AND myfield4 IN (?) OR myfield5 IS NULL".to_string()
        );
        assert!(where2.values.len() > 0);
    }
}
