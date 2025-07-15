use crate::postgres::{ExpressionBuilder, Logic};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct WhereBuilder {
    pub statement: String,
    pub values: Vec<Value>, //For Binding
}

impl WhereBuilder {
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

    pub fn build(values: Vec<ExpressionBuilder>) -> WhereBuilder {
        let mut data: WhereBuilder = WhereBuilder::default();
        let mut expressions: Vec<String> = Vec::new();
        let do_grouping = values.len() > 1;
        for mut item in values {
            let expression = Self::format(item.condition, item.logic, do_grouping);
            if !item.values.is_empty() {
                data.values.append(&mut item.values);
            }
            expressions.push(expression);
        }
        data.statement = format!("WHERE {}", expressions.join(" ").trim());
        data
    }
}

#[cfg(test)]
pub mod test_where_builder {
    use super::*;
    use crate::postgres::{ConditionBuilder, ConditionValue, Operator};
    use serde_json::Number;

    #[tokio::test]
    async fn test_where_builder() {
        let data = ConditionValue::Value(Value::String("MYVALUE".to_string()));
        let result = ConditionBuilder::bind(&data);
        assert_eq!(result, Some("?".to_string()));

        let data = ConditionValue::Value(Value::Number(Number::from_i128(128).unwrap()));
        let result = ConditionBuilder::bind(&data);
        assert_eq!(result, Some("?".to_string()));

        let values = ConditionValue::Value(Value::Array(vec![
            Value::String("MYVALUE".to_string()),
            Value::Number(Number::from_i128(128).unwrap()),
        ]));
        let result = ConditionBuilder::bind(&values);
        assert_eq!(result, Some("(?)".to_string()));

        let where_expression = ConditionBuilder {
            table_alias: None,
            field: "myfield1".to_string(),
            operator: Operator::Eq,
            value: Some(ConditionValue::Value(Value::String(String::from(
                "MYVALUE",
            )))),
            logic: None,
        };

        let result = ConditionBuilder::build(&where_expression);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = result.unwrap();
        assert_eq!(result, "myfield1 = ?".to_string());

        let where_expression = ConditionBuilder {
            table_alias: None,
            field: "myfield1".to_string(),
            operator: Operator::In,
            value: Some(values.clone()),
            logic: None,
        };

        let result = ConditionBuilder::build(&where_expression);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = result.unwrap();
        assert_eq!(result, "myfield1 IN (?)".to_string());

        let where_expressions = vec![ConditionBuilder {
            table_alias: Some("t".to_string()),
            field: "".to_string(),
            operator: Operator::Eq,
            value: Some(ConditionValue::Value(Value::String("MYVALUE".to_string()))),
            logic: None,
        }];
        let expression_error = ExpressionBuilder::build(where_expressions, None);
        assert!(expression_error.is_err(), "expecting field error");

        let where_expressions = vec![
            ConditionBuilder {
                table_alias: Some("t".to_string()),
                field: "myfield1".to_string(),
                operator: Operator::Eq,
                value: Some(ConditionValue::Value(Value::String("MYVALUE".to_string()))),
                logic: None,
            },
            ConditionBuilder {
                table_alias: Some("t".to_string()),
                field: "myfield2".to_string(),
                operator: Operator::Eq,
                value: Some(ConditionValue::Value(Value::Number(
                    Number::from_i128(128).unwrap(),
                ))),
                logic: Some(Logic::And),
            },
        ];
        let expression1 = ExpressionBuilder::build(where_expressions, None);
        assert!(expression1.is_ok(), "{:?}", expression1.err());
        let expression1 = expression1.unwrap();
        assert_eq!(
            expression1.condition,
            "t.myfield1 = ? AND t.myfield2 = ?".to_string()
        );
        assert_eq!(expression1.logic, None);
        assert!(expression1.values.len() > 0);

        let where_expressions = vec![
            ConditionBuilder {
                table_alias: Some("t".to_string()),
                field: "myfield3".to_string(),
                operator: Operator::Eq,
                value: Some(ConditionValue::Value(Value::String("MYVALUE".to_string()))),
                logic: None,
            },
            ConditionBuilder {
                table_alias: Some("t".to_string()),
                field: "myfield4".to_string(),
                operator: Operator::In,
                value: Some(values),
                logic: Some(Logic::And),
            },
            ConditionBuilder {
                table_alias: Some("t".to_string()),
                field: "myfield5".to_string(),
                operator: Operator::IsNull,
                value: None,
                logic: Some(Logic::Or),
            },
        ];
        let expression2 = ExpressionBuilder::build(where_expressions, Some(Logic::And));
        assert!(expression2.is_ok(), "{:?}", expression2.err());
        let expression2 = expression2.unwrap();
        assert_eq!(
            expression2.condition,
            "t.myfield3 = ? AND t.myfield4 IN (?) OR t.myfield5 IS NULL".to_string()
        );
        assert_eq!(expression2.logic, Some(Logic::And));
        assert!(expression2.values.len() > 0);

        let where1 = WhereBuilder::build(vec![expression1, expression2.clone()]);
        assert_eq!(where1.statement,"WHERE (t.myfield1 = ? AND t.myfield2 = ?) AND (t.myfield3 = ? AND t.myfield4 IN (?) OR t.myfield5 IS NULL)".to_string());
        assert!(where1.values.len() > 0);
        let where2 = WhereBuilder::build(vec![expression2]);
        assert_eq!(
            where2.statement,
            "WHERE t.myfield3 = ? AND t.myfield4 IN (?) OR t.myfield5 IS NULL".to_string()
        );
        assert!(where2.values.len() > 0);
    }
}
