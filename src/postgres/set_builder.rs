use crate::postgres::SelectBuilder;
use serde_json::Value;

#[derive(Clone, Debug)]
pub enum SetValue {
    Value(Value),
    Query(SelectBuilder),
}

#[derive(Debug, Clone)]
pub struct SetBuilder {
    pub statement: String,
    pub values: Vec<SetValue>,
}

#[derive(Debug, Clone)]
pub struct SetFieldUpdate {
    pub field: String,
    pub value: SetValue,
}

impl SetBuilder {
    pub fn build(items: Vec<SetFieldUpdate>) -> Self {
        let mut expressions: Vec<String> = Vec::new();
        let mut values: Vec<SetValue> = Vec::new();

        for item in &items {
            expressions.push(format!("{} = ?", item.field));
            values.push(item.value.to_owned());
        }
        let statement = format!("SET {}", expressions.join(", "));
        Self { statement, values }
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
        assert_eq!(result.values.len(), 2);
        assert_eq!(result.statement, "SET email = ?, password = ?");
    }
}
