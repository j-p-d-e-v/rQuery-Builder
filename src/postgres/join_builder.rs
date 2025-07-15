use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{ExpressionBuilder, Logic};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JoinKind {
    Inner,
    Left,
    Right,
    Full,
    Cross,
}

impl std::fmt::Display for JoinKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Inner => "INNER",
            Self::Left => "LEFT",
            Self::Right => "RIGHT",
            Self::Full => "FULL",
            Self::Cross => "CROSS",
        };
        write!(f, "{value}")
    }
}

#[derive(Clone, Debug)]
pub struct JoinItem {
    pub field: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct JoinBuilder {
    pub statement: String,
    pub values: Vec<Value>, //For Binding
}

impl JoinBuilder {
    fn format(condition: String, logic: Option<Logic>, do_grouping: bool) -> String {
        let logic = if let Some(value) = logic {
            value.to_string()
        } else {
            "".to_string()
        };
        if do_grouping {
            format!("{logic} ({condition})")
        } else {
            condition
        }
    }

    pub fn build(
        kind: JoinKind,
        table: &str,
        table_alias: &str,
        values: Vec<ExpressionBuilder>,
    ) -> JoinBuilder {
        let mut data: JoinBuilder = JoinBuilder::default();
        let mut expressions: Vec<String> = Vec::new();
        let do_grouping = values.len() > 1;
        for mut item in values {
            let expression = Self::format(item.condition, item.logic, do_grouping);
            if !item.values.is_empty() {
                data.values.append(&mut item.values);
            }
            expressions.push(expression);
        }
        data.statement = format!(
            "{} JOIN {} as {} ON {}",
            &kind,
            table,
            table_alias,
            expressions.join(" ").trim()
        );
        data
    }
}

#[cfg(test)]
pub mod test_join_builder {
    use crate::postgres::{ConditionBuilder, Operator};

    use super::*;

    #[tokio::test]
    async fn test_join() {
        let condition1 = ConditionBuilder {
            table_alias: Some("p".to_string()),
            field: "id".to_string(),
            operator: Operator::Eq,
            value: Some(Value::String("test".to_string())),
            logic: None,
        };

        let expression1 = ExpressionBuilder::build(vec![condition1], None);
        assert!(expression1.is_ok(), "{:?}", expression1.err());
        let expression1 = expression1.unwrap();
        let result = JoinBuilder::build(JoinKind::Left, "products", "p", vec![expression1]);
        assert!(result.is_ok(), "{:?}", result.err());
    }
}
