use crate::placeholder::PlaceholderKind;
use crate::postgres::{ExpressionBuilder, WhereBuilder};
use anyhow::anyhow;
use serde_json::Value;

#[derive(Clone, Debug, Default)]
pub struct DeleteBuilder {
    pub table: String,
    pub set: Vec<String>,
    pub values: Vec<Value>,
    using_table: Option<String>,
    filter_statement: Option<String>,
    returning_statement: Option<String>,
    pub placeholder_kind: PlaceholderKind,
}

impl DeleteBuilder {
    pub fn new(placeholder: PlaceholderKind) -> Self {
        Self {
            placeholder_kind: placeholder,
            ..Default::default()
        }
    }

    pub fn table(&mut self, table: &str, table_alias: Option<&str>) -> &mut Self {
        self.table = if let Some(alias) = table_alias {
            format!("{} as {}", table, alias)
        } else {
            table.to_string()
        };
        self
    }

    pub fn using(&mut self, table: &str, table_alias: Option<&str>) -> &mut Self {
        self.using_table = if let Some(alias) = table_alias {
            Some(format!("{} as {}", table, alias))
        } else {
            Some(table.to_string())
        };
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
        let mut statement = format!("DELETE FROM {}", self.table);
        let mut value_counter: usize = 0;

        if let Some(stmt) = &self.using_table {
            statement.push_str(&format!(" {stmt}"));
        };
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
pub mod test_delete_builder {
    use crate::postgres::{ConditionBuilder, ConditionValue, Operator};

    use super::*;
    use serde_json::Value;

    #[tokio::test]
    async fn test_delete_question_mark() {
        let mut builder = DeleteBuilder::new(PlaceholderKind::QuestionMark);
        builder
            .table("users", Some("u"))
            .filter(vec![ExpressionBuilder::build(
                vec![ConditionBuilder {
                    table_alias: Some("u".to_string()),
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
        let statement = builder.returning(vec!["u.email", "u.name"]).build();
        assert!(statement.is_ok(), "{:?}", statement.err());
        assert_eq!(
            statement.unwrap(),
            "DELETE FROM users as u WHERE u.email = ? RETURNING u.email, u.name"
        );
        assert_eq!(builder.get_values().len(), 1);
    }
}
