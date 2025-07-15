use anyhow::anyhow;
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug)]
pub struct JoinBuilder;

impl JoinBuilder {
    pub fn build(values: Vec<JoinItem>) -> anyhow::Result<String> {
        if values.is_empty() {
            return Err(anyhow!("order by item is empty"));
        }
        let mut order_by: Vec<String> = Vec::new();
        for item in values.into_iter() {
            if item.field.is_empty() {
                return Err(anyhow!("order by field is empty"));
            }
            let value = format!("{} {}", item.field, item.sequence);
            if !order_by.contains(&value) {
                order_by.push(value);
            }
        }
        Ok(format!("ORDER BY {}", order_by.join(", ").trim()))
    }
}

#[cfg(test)]
pub mod test_order_by_builder {
    use super::*;

    #[tokio::test]
    async fn test_order_by_builder() {
        let order_by = JoinItem {
            field: "".to_string(),
            sequence: Sequence::Asc,
        };
        let result = JoinBuilder::build(vec![order_by]);
        assert!(result.is_err(), "expected error");

        let order_by_items = vec![JoinItem {
            field: "myfield1".to_string(),
            sequence: Sequence::Asc,
        }];
        let result = JoinBuilder::build(order_by_items);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = result.unwrap();
        assert_eq!(result, "ORDER BY myfield1 ASC");

        let order_by_items = vec![
            JoinItem {
                field: "myfield1".to_string(),
                sequence: Sequence::Asc,
            },
            JoinItem {
                field: "myfield2".to_string(),
                sequence: Sequence::Desc,
            },
        ];
        let result = JoinBuilder::build(order_by_items);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = result.unwrap();
        assert_eq!(result, "ORDER BY myfield1 ASC, myfield2 DESC");
    }
}
