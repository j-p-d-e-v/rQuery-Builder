use super::Logic;
use crate::postgres::Operator;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionBuilder {
    pub table_alias: Option<String>,
    pub field: String,
    pub operator: Operator,
    pub value: Option<Value>,
    pub logic: Option<Logic>,
}

impl ConditionBuilder {
    pub fn new(data: ConditionBuilder) -> Self {
        Self { ..data }
    }

    pub fn bind(value: &Value) -> Option<String> {
        match value {
            Value::Array(_) => Some("(?)".to_string()),
            _ => Some("?".to_string()),
        }
    }

    pub fn format(item: &ConditionBuilder) -> anyhow::Result<String> {
        let field = &item.field;
        let table_alias = if let Some(value) = &item.table_alias {
            &format!("{value}.")
        } else {
            ""
        };
        let operator = &item.operator;
        if field.is_empty() {
            return Err(anyhow!("field is empty"));
        }
        let value: Option<String> = if let Some(value) = &item.value {
            Self::bind(value)
        } else {
            None
        };
        if let Some(value) = value
            && operator != &Operator::IsNull
            && operator != &Operator::NotNull
        {
            Ok(format!("{table_alias}{field} {operator} {value}"))
        } else {
            Ok(format!("{table_alias}{field} {operator}"))
        }
    }
}
