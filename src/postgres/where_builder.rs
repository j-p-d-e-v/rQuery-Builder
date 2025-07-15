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
        let mut clauses: Vec<String> = Vec::new();
        let do_grouping = values.len() > 1;
        for mut item in values {
            let clause = Self::format(item.condition, item.logic, do_grouping);
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
    use crate::postgres::{ConditionBuilder, Operator};
    use serde_json::Number;

    #[tokio::test]
    async fn test_where_builder() {
        let data = Value::String("MYVALUE".to_string());
        let result = ConditionBuilder::bind(&data);
        assert_eq!(result, Some("?".to_string()));

        let data = Value::Number(Number::from_i128(128).unwrap());
        let result = ConditionBuilder::bind(&data);
        assert_eq!(result, Some("?".to_string()));

        let values = Value::Array(vec![
            Value::String("MYVALUE".to_string()),
            Value::Number(Number::from_i128(128).unwrap()),
        ]);
        let result = ConditionBuilder::bind(&values);
        assert_eq!(result, Some("(?)".to_string()));

        let where_clause = ConditionBuilder {
            table_alias: None,
            field: "myfield1".to_string(),
            operator: Operator::Eq,
            value: Some(Value::String(String::from("MYVALUE"))),
            logic: None,
        };

        let result = ConditionBuilder::build(&where_clause);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = result.unwrap();
        assert_eq!(result, "myfield1 = ?".to_string());

        let where_clause = ConditionBuilder {
            table_alias: None,
            field: "myfield1".to_string(),
            operator: Operator::In,
            value: Some(values.clone()),
            logic: None,
        };

        let result = ConditionBuilder::build(&where_clause);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = result.unwrap();
        assert_eq!(result, "myfield1 IN (?)".to_string());

        let where_clauses = vec![ConditionBuilder {
            table_alias: Some("t".to_string()),
            field: "".to_string(),
            operator: Operator::Eq,
            value: Some(Value::String("MYVALUE".to_string())),
            logic: None,
        }];
        let clause_error = ExpressionBuilder::build(where_clauses, None);
        assert!(clause_error.is_err(), "expecting field error");

        let where_clauses = vec![
            ConditionBuilder {
                table_alias: Some("t".to_string()),
                field: "myfield1".to_string(),
                operator: Operator::Eq,
                value: Some(Value::String("MYVALUE".to_string())),
                logic: None,
            },
            ConditionBuilder {
                table_alias: Some("t".to_string()),
                field: "myfield2".to_string(),
                operator: Operator::Eq,
                value: Some(Value::Number(Number::from_i128(128).unwrap())),
                logic: Some(Logic::And),
            },
        ];
        let clause1 = ExpressionBuilder::build(where_clauses, None);
        assert!(clause1.is_ok(), "{:?}", clause1.err());
        let clause1 = clause1.unwrap();
        assert_eq!(
            clause1.condition,
            "t.myfield1 = ? AND t.myfield2 = ?".to_string()
        );
        assert_eq!(clause1.logic, None);
        assert!(clause1.values.len() > 0);

        let where_clauses = vec![
            ConditionBuilder {
                table_alias: Some("t".to_string()),
                field: "myfield3".to_string(),
                operator: Operator::Eq,
                value: Some(Value::String("MYVALUE".to_string())),
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
        let clause2 = ExpressionBuilder::build(where_clauses, Some(Logic::And));
        assert!(clause2.is_ok(), "{:?}", clause2.err());
        let clause2 = clause2.unwrap();
        assert_eq!(
            clause2.condition,
            "t.myfield3 = ? AND t.myfield4 IN (?) OR t.myfield5 IS NULL".to_string()
        );
        assert_eq!(clause2.logic, Some(Logic::And));
        assert!(clause2.values.len() > 0);

        let where1 = WhereBuilder::build(vec![clause1, clause2.clone()]);
        assert_eq!(where1.statement,"WHERE (t.myfield1 = ? AND t.myfield2 = ?) AND (t.myfield3 = ? AND t.myfield4 IN (?) OR t.myfield5 IS NULL)".to_string());
        assert!(where1.values.len() > 0);
        let where2 = WhereBuilder::build(vec![clause2]);
        assert_eq!(
            where2.statement,
            "WHERE t.myfield3 = ? AND t.myfield4 IN (?) OR t.myfield5 IS NULL".to_string()
        );
        assert!(where2.values.len() > 0);
    }
}
