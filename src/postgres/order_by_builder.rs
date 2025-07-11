use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Sequence {
    Asc,
    Desc,
}

impl std::fmt::Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sequence = match self {
            Self::Asc => "ASC",
            Self::Desc => "DESC",
        };
        write!(f, "{sequence}")
    }
}

#[derive(Clone, Debug)]
pub struct OrderByItem {
    pub field: String,
    pub sequence: Sequence,
}

#[derive(Clone, Debug)]
pub struct OrderByBuilder;

impl OrderByBuilder {
    pub fn build(values: Vec<OrderByItem>) -> anyhow::Result<String> {
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
        let order_by = OrderByItem {
            field: "".to_string(),
            sequence: Sequence::Asc,
        };
        let result = OrderByBuilder::build(vec![order_by]);
        assert!(result.is_err(), "expected error");

        let order_by_items = vec![OrderByItem {
            field: "myfield1".to_string(),
            sequence: Sequence::Asc,
        }];
        let result = OrderByBuilder::build(order_by_items);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = result.unwrap();
        assert_eq!(result, "ORDER BY myfield1 ASC");

        let order_by_items = vec![
            OrderByItem {
                field: "myfield1".to_string(),
                sequence: Sequence::Asc,
            },
            OrderByItem {
                field: "myfield2".to_string(),
                sequence: Sequence::Desc,
            },
        ];
        let result = OrderByBuilder::build(order_by_items);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = result.unwrap();
        assert_eq!(result, "ORDER BY myfield1 ASC, myfield2 DESC");
    }
}
