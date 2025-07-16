use super::Logic;
use crate::postgres::Operator;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionValue {
    Field(String, String), //(String,String) - (table alias, table field)
    Single(Value),
    Range(Value, Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionBuilder {
    pub table_alias: Option<String>,
    pub field: String,
    pub operator: Operator,
    pub value: Option<ConditionValue>,
    pub logic: Option<Logic>,
}

impl ConditionBuilder {
    pub fn bind_value(value: &Value) -> String {
        match value {
            Value::Array(_) => "(?)".to_string(),
            _ => "?".to_string(),
        }
    }

    pub fn bind(condition_value: &ConditionValue) -> Option<String> {
        let value = match condition_value {
            ConditionValue::Field(table_alias, table_field) => {
                format!("{table_alias}.{table_field}")
            }
            ConditionValue::Single(value) => Self::bind_value(value),
            ConditionValue::Range(value1, value2) => format!(
                "{} AND {}",
                Self::bind_value(value1),
                Self::bind_value(value2)
            ),
        };
        Some(value)
    }

    pub fn build(item: &ConditionBuilder) -> anyhow::Result<String> {
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
        let condition = if let Some(value) = value
            && operator != &Operator::IsNull
            && operator != &Operator::NotNull
        {
            format!("{table_alias}{field} {operator} {value}")
        } else {
            format!("{table_alias}{field} {operator}")
        };

        if let Some(logic) = &item.logic {
            Ok(format!("{logic} {condition}"))
        } else {
            Ok(condition)
        }
    }
}

#[cfg(test)]
pub mod test_condition_builder {
    use serde_json::Number;

    use super::*;

    #[tokio::test]
    async fn test_condition() {
        let result = ConditionBuilder::build(&ConditionBuilder {
            table_alias: Some("t".to_string()),
            field: "myfield1".to_string(),
            operator: Operator::Eq,
            value: Some(ConditionValue::Single(Value::String("test".to_string()))),
            logic: None,
        });
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(result.unwrap(), "t.myfield1 = ?".to_string());

        let result = ConditionBuilder::build(&ConditionBuilder {
            table_alias: Some("t".to_string()),
            field: "myfield1".to_string(),
            operator: Operator::Eq,
            value: Some(ConditionValue::Single(Value::String("test".to_string()))),
            logic: Some(Logic::And),
        });
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(result.unwrap(), "AND t.myfield1 = ?".to_string());

        let result = ConditionBuilder::build(&ConditionBuilder {
            table_alias: Some("t".to_string()),
            field: "myfield1".to_string(),
            operator: Operator::Eq,
            value: Some(ConditionValue::Field(
                "p".to_string(),
                "myfield2".to_string(),
            )),
            logic: Some(Logic::And),
        });
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(result.unwrap(), "AND t.myfield1 = p.myfield2".to_string());
        let result = ConditionBuilder::build(&ConditionBuilder {
            table_alias: Some("t".to_string()),
            field: "myfield1".to_string(),
            operator: Operator::Between,
            value: Some(ConditionValue::Range(
                Value::Number(Number::from_u128(10).unwrap()),
                Value::Number(Number::from_u128(20).unwrap()),
            )),
            logic: Some(Logic::And),
        });
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(
            result.unwrap(),
            "AND t.myfield1 BETWEEN ? AND ?".to_string()
        );
    }
}
