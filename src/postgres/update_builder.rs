use crate::placeholder::PlaceholderKind;
use crate::postgres::{ExpressionBuilder, SetBuilder, SetFieldUpdate, WhereBuilder};
use anyhow::anyhow;
use serde_json::Value;

#[derive(Clone, Debug, Default)]
pub struct UpdateBuilder {
    pub table: String,
    pub set: Vec<String>,
    pub values: Vec<Value>,
    set_statement: String,
    filter_statement: Option<String>,
    returning_statement: Option<String>,
    pub placeholder_kind: PlaceholderKind,
}

impl UpdateBuilder {
    pub fn new(placeholder: PlaceholderKind) -> Self {
        Self {
            placeholder_kind: placeholder,
            ..Default::default()
        }
    }

    pub fn table(&mut self, table: &str) -> &mut Self {
        self.table = table.to_string();
        self
    }

    pub fn filter(&mut self, values: Vec<ExpressionBuilder>) -> &mut Self {
        if !values.is_empty() {
            let mut result = WhereBuilder::build(values);
            self.filter_statement = Some(result.statement);
            if !result.values.is_empty() {
                self.values.append(&mut result.values);
            }
        }
        self
    }

    pub fn set(&mut self, values: Vec<SetFieldUpdate>) -> anyhow::Result<&mut Self> {
        if !self.set_statement.is_empty() {
            return Err(anyhow!("`.set()` can only be calld once"));
        }
        let mut builder = SetBuilder::build(values)?;
        self.set_statement = builder.statement;
        self.values.append(&mut builder.values);
        Ok(self)
    }

    pub fn get_values(&self) -> Vec<Value> {
        self.values.to_owned()
    }

    pub fn returning(&mut self, values: Vec<&str>) -> &mut Self {
        if !values.is_empty() {
            self.returning_statement = Some(format!(
                "RETURNING {}",
                values
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
        }
        self
    }

    pub fn build(&self) -> anyhow::Result<String> {
        let mut statement = format!("UPDATE {} {}", self.table, self.set_statement);
        let mut value_counter: usize = 0;

        if let Some(stmt) = &self.filter_statement {
            statement.push_str(&format!(" {stmt}"));
        };
        if let Some(stmt) = &self.returning_statement {
            statement.push_str(&format!(" {stmt}"));
        }

        let values: Vec<String> = statement
            .chars()
            .map(|c| {
                if Some(c) == char::from_u32(63) {
                    match self.placeholder_kind {
                        PlaceholderKind::QuestionMark => "?".to_string(),
                        PlaceholderKind::DollarSequential => {
                            value_counter += 1;
                            format!("${value_counter}")
                        }
                    }
                } else {
                    c.to_string()
                }
            })
            .collect();
        Ok(values.join("").trim().to_string())
    }
}

#[cfg(test)]
pub mod test_update_builder {
    use crate::postgres::{ConditionBuilder, ConditionValue, Operator, SelectBuilder, SetValue};

    use super::*;
    use serde_json::Value;

    #[tokio::test]
    async fn test_update_set_twice() {
        let mut builder = UpdateBuilder::new(PlaceholderKind::QuestionMark);
        let set_ok_result = builder.table("users").set(vec![
            SetFieldUpdate {
                field: "name".to_string(),
                value: SetValue::Value(Value::String("Test Update 1".to_string())),
            },
            SetFieldUpdate {
                field: "email".to_string(),
                value: SetValue::Value(Value::String("test1update@email.com".to_string())),
            },
        ]);
        assert!(set_ok_result.is_ok(), "{:?}", set_ok_result.err());
        let set_ok_result = set_ok_result.unwrap();
        let set_err_result = set_ok_result.set(vec![
            SetFieldUpdate {
                field: "name".to_string(),
                value: SetValue::Value(Value::String("Test Update 1".to_string())),
            },
            SetFieldUpdate {
                field: "email".to_string(),
                value: SetValue::Value(Value::String("test1update@email.com".to_string())),
            },
        ]);

        assert!(set_err_result.is_err(), "cannot call set twice");
    }
    #[tokio::test]
    async fn test_update_question_mark() {
        let mut builder = UpdateBuilder::new(PlaceholderKind::QuestionMark);
        let set_ok_result = builder.table("users").set(vec![
            SetFieldUpdate {
                field: "name".to_string(),
                value: SetValue::Value(Value::String("Test Update 1".to_string())),
            },
            SetFieldUpdate {
                field: "email".to_string(),
                value: SetValue::Value(Value::String("test1update@email.com".to_string())),
            },
        ]);
        assert!(set_ok_result.is_ok(), "{:?}", set_ok_result.err());
        let set_ok_result = set_ok_result.unwrap();
        set_ok_result.filter(vec![ExpressionBuilder::build(
            vec![ConditionBuilder {
                table_alias: None,
                field: "email".to_string(),
                logic: None,
                operator: Operator::Eq,
                value: Some(ConditionValue::Single(Value::String(
                    "test1@example.com".to_string(),
                ))),
            }],
            None,
        )
        .unwrap()]);
        let statement = set_ok_result.returning(vec!["email", "name"]).build();
        assert!(statement.is_ok(), "{:?}", statement.err());
        assert_eq!(
            statement.unwrap(),
            "UPDATE users SET name = ?, email = ? WHERE email = ? RETURNING email, name"
        );
        assert_eq!(set_ok_result.get_values().len(), 3);
    }

    #[tokio::test]
    async fn test_update_dollar_sequential() {
        let mut builder = UpdateBuilder::new(PlaceholderKind::DollarSequential);
        let set_ok_result = builder.table("users").set(vec![
            SetFieldUpdate {
                field: "name".to_string(),
                value: SetValue::Value(Value::String("Test Update 1".to_string())),
            },
            SetFieldUpdate {
                field: "email".to_string(),
                value: SetValue::Value(Value::String("test1update@email.com".to_string())),
            },
        ]);
        assert!(set_ok_result.is_ok(), "{:?}", set_ok_result.err());
        let set_ok_result = set_ok_result.unwrap();
        set_ok_result.filter(vec![ExpressionBuilder::build(
            vec![ConditionBuilder {
                table_alias: None,
                field: "email".to_string(),
                logic: None,
                operator: Operator::Eq,
                value: Some(ConditionValue::Single(Value::String(
                    "test1@example.com".to_string(),
                ))),
            }],
            None,
        )
        .unwrap()]);
        let statement = set_ok_result.returning(vec!["email", "name"]).build();
        assert!(statement.is_ok(), "{:?}", statement.err());
        assert_eq!(
            statement.unwrap(),
            "UPDATE users SET name = $1, email = $2 WHERE email = $3 RETURNING email, name"
        );
        assert_eq!(set_ok_result.get_values().len(), 3);
    }

    #[tokio::test]
    async fn test_update_dollarsequential_using_subquery_filter() {
        let mut select_builder = SelectBuilder::new(PlaceholderKind::QuestionMark);
        select_builder
            .table("users", "u")
            .columns("u", vec!["email"])
            .filter(vec![ExpressionBuilder::build(
                vec![ConditionBuilder {
                    field: "email".to_string(),
                    table_alias: Some("u".to_string()),
                    operator: Operator::Eq,
                    value: Some(ConditionValue::Single(Value::String(
                        "test1@example.com".to_string(),
                    ))),
                    logic: None,
                }],
                None,
            )
            .unwrap()]);

        let mut builder = UpdateBuilder::new(PlaceholderKind::DollarSequential);
        let set_ok_result = builder.table("users").set(vec![
            SetFieldUpdate {
                field: "name".to_string(),
                value: SetValue::Value(Value::String("Test Update 1".to_string())),
            },
            SetFieldUpdate {
                field: "email".to_string(),
                value: SetValue::Query(select_builder),
            },
        ]);
        assert!(set_ok_result.is_ok(), "{:?}", set_ok_result.err());
        let set_ok_result = set_ok_result.unwrap();
        set_ok_result.filter(vec![ExpressionBuilder::build(
            vec![ConditionBuilder {
                table_alias: None,
                field: "email".to_string(),
                logic: None,
                operator: Operator::Eq,
                value: Some(ConditionValue::Single(Value::String(
                    "test1@example.com".to_string(),
                ))),
            }],
            None,
        )
        .unwrap()]);
        let statement = set_ok_result.returning(vec!["email", "name"]).build();
        assert!(statement.is_ok(), "{:?}", statement.err());
        assert_eq!(
            statement.unwrap(),
            "UPDATE users SET name = $1, email = (SELECT u.email FROM users as u WHERE u.email = $2) WHERE email = $3 RETURNING email, name"
        );
        assert_eq!(set_ok_result.get_values().len(), 3);
    }
}
