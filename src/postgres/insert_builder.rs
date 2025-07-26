use crate::placeholder::PlaceholderKind;
use anyhow::anyhow;
use serde_json::Value;

#[derive(Clone, Debug, Default)]
pub struct InsertBuilder {
    pub table: String,
    pub fields: Vec<String>,
    pub values: Vec<Vec<Value>>,
    pub returning_statement: Option<String>,
    pub placeholder_kind: PlaceholderKind,
}

impl InsertBuilder {
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

    pub fn columns(&mut self, values: Vec<&str>) -> &mut Self {
        self.fields = values.iter().map(|value| value.to_string()).collect();
        self
    }

    pub fn values(&mut self, values: Vec<Value>) -> anyhow::Result<&mut Self> {
        if self.fields.len() != values.len() {
            return Err(anyhow!("mistched number of fields and values"));
        }
        self.values.append(&mut vec![values]);
        Ok(self)
    }

    pub fn get_values(&self) -> Vec<Vec<Value>> {
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
        let fields = self.fields.join(", ");
        let mut value_counter: usize = 0;
        let values: Vec<String> = self
            .values
            .iter()
            .map(|items| {
                let placeholders: Vec<String> = items
                    .iter()
                    .map(|_| match self.placeholder_kind {
                        PlaceholderKind::QuestionMark => "?".to_string(),
                        PlaceholderKind::DollarSequential => {
                            value_counter += 1;
                            format!("${value_counter}")
                        }
                    })
                    .collect();
                format!("({})", placeholders.join(", "))
            })
            .collect();
        let values: String = values.join(", ");
        let returning_statement: String = self
            .returning_statement
            .to_owned()
            .unwrap_or("".to_string());
        let statement = format!(
            "INSERT INTO {}({}) VALUES {} {}",
            self.table, fields, values, returning_statement
        );
        Ok(statement.trim().to_string())
    }
}

#[cfg(test)]
pub mod test_insert_builder {
    use super::*;
    use serde_json::Value;

    #[tokio::test]
    async fn test_insert_builder() {
        let mut builder = InsertBuilder::new(PlaceholderKind::DollarSequential);
        let _ = builder.table("users").columns(vec!["name", "email"]);
        let result = builder.values(vec![
            Value::String("Juan dela Cruz".to_string()),
            Value::String("jdc@test.com".to_string()),
        ]);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = builder.values(vec![
            Value::String("Jose Rizal".to_string()),
            Value::String("jr@test.com".to_string()),
        ]);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = builder.build();
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(
            result.unwrap(),
            "INSERT INTO users(name, email) VALUES ($1, $2), ($3, $4)".to_string()
        );

        let mut builder = InsertBuilder::new(PlaceholderKind::QuestionMark);
        let _ = builder.table("users").columns(vec!["name", "email"]);
        let result = builder.values(vec![
            Value::String("Juan dela Cruz".to_string()),
            Value::String("jdc@test.com".to_string()),
        ]);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = builder.values(vec![
            Value::String("Jose Rizal".to_string()),
            Value::String("jr@test.com".to_string()),
        ]);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = builder.returning(vec!["name", "email"]).build();
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(
            result.unwrap(),
            "INSERT INTO users(name, email) VALUES (?, ?), (?, ?) RETURNING name, email"
                .to_string()
        );
    }
}
