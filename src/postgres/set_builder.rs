use crate::{placeholder::PlaceholderKind, postgres::SelectBuilder};
use anyhow::anyhow;
use serde_json::Value;

#[derive(Clone, Debug)]
pub enum SetValue {
    Value(Value),
    Query(SelectBuilder),
}

#[derive(Debug, Clone)]
pub struct SetBuilder {
    pub statement: String,
    pub values: Vec<Value>,
}

#[derive(Debug, Clone)]
pub struct SetFieldUpdate {
    pub field: String,
    pub value: SetValue,
}

impl SetBuilder {
    pub fn build(items: Vec<SetFieldUpdate>) -> anyhow::Result<Self> {
        let mut expressions: Vec<String> = Vec::new();
        let mut values: Vec<Value> = Vec::new();

        for item in &items {
            match &item.value {
                SetValue::Value(value) => {
                    expressions.push(format!("{} = ?", item.field));
                    values.push(value.to_owned());
                }
                SetValue::Query(selected_builder) => {
                    if selected_builder.placeholder_kind != PlaceholderKind::QuestionMark {
                        return Err(anyhow!(
                            "select builder should be using the question mark placeholder kind"
                        ));
                    }
                    let result = selected_builder.build()?;
                    expressions.push(format!("{} = ({})", item.field, result));
                    values.append(&mut selected_builder.get_values());
                }
            }
        }
        let statement = format!("SET {}", expressions.join(", "));
        Ok(Self { statement, values })
    }
}

#[cfg(test)]
pub mod test_set_builder {
    use super::*;

    #[tokio::test]
    async fn test_builder() {
        let items: Vec<SetFieldUpdate> = vec![
            SetFieldUpdate {
                field: "email".to_string(),
                value: SetValue::Value(Value::String("joserizal@ph.com".to_string())),
            },
            SetFieldUpdate {
                field: "password".to_string(),
                value: SetValue::Value(Value::String("joserizal@ph.com".to_string())),
            },
        ];
        let result = SetBuilder::build(items);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = result.unwrap();
        assert_eq!(result.values.len(), 2);
        assert_eq!(result.statement, "SET email = ?, password = ?");
    }
}
