use crate::postgres::{ConditionBuilder, ConditionValue, Logic};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExpressionBuilder {
    pub condition: String,
    pub logic: Option<Logic>,
    pub values: Vec<Value>,
}

impl ExpressionBuilder {
    pub fn build(
        values: Vec<ConditionBuilder>,
        logic: Option<Logic>,
    ) -> anyhow::Result<ExpressionBuilder> {
        let mut data: ExpressionBuilder = ExpressionBuilder::default();
        for item in values {
            let condition = ConditionBuilder::build(&item)?;
            if data.condition.is_empty() {
                data.condition = condition;
            } else {
                data.condition = format!("{} {}", data.condition, condition)
            }
            if let Some(condition_value) = item.value {
                // Dont use _ => {} in match because we want to make sure that values are push correctly
                match condition_value {
                    ConditionValue::Single(value) => {
                        data.values.push(value);
                    }
                    ConditionValue::Range(value1, value2) => {
                        data.values.push(value1);
                        data.values.push(value2);
                    }
                    ConditionValue::Field(_, _) => {}
                }
            }
                
        }
        data.logic = logic;
        Ok(data)
    }
}

#[cfg(test)]
pub mod test_expression_builder {
    use serde_json::Number;

    use crate::postgres::{ConditionValue, Operator};

    use super::*;

    #[tokio::test]
    async fn test_expression() {
        let condition1 = ConditionBuilder {
            table_alias: Some("t".to_string()),
            field: "myfield1".to_string(),
            operator: Operator::Eq,
            value: Some(ConditionValue::Single(Value::String("test".to_string()))),
            logic: None,
        };

        let condition2 = ConditionBuilder {
            table_alias: Some("t".to_string()),
            field: "myfield2".to_string(),
            operator: Operator::Eq,
            value: Some(ConditionValue::Single(Value::String("test".to_string()))),
            logic: Some(Logic::And),
        };

        let result = ExpressionBuilder::build(vec![condition1, condition2], None);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = result.unwrap();
        assert_eq!(
            result.condition,
            "t.myfield1 = ? AND t.myfield2 = ?".to_string()
        );
        assert_eq!(result.logic, None);
        assert!(result.values.len() > 0);

        let condition1 = ConditionBuilder {
            table_alias: Some("t".to_string()),
            field: "myfield2".to_string(),
            operator: Operator::Between,
            value: Some(ConditionValue::Range(Value::Number(Number::from_u128(10).unwrap()),Value::Number(Number::from_u128(20).unwrap()))),
            logic: Some(Logic::And),
        };
        let result = ExpressionBuilder::build(vec![condition1.clone()], None);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = result.unwrap();
        assert_eq!(
            result.condition,
            "AND t.myfield2 BETWEEN ? AND ?".to_string()
        );
        assert_eq!(result.logic, None);
        assert_eq!(result.values.len(),2);
    }
}
