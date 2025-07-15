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
    pub table_alias: Option<String>,
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
            let table_alias = if let Some(value) = item.table_alias {
                &format!("{value}.")
            } else {
                ""
            };
            if item.field.is_empty() {
                return Err(anyhow!("order by field is empty"));
            }
            let value = format!("{}{} {}", table_alias, item.field, item.sequence);
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
            table_alias: None,
            field: "".to_string(),
            sequence: Sequence::Asc,
        };
        let result = OrderByBuilder::build(vec![order_by]);
        assert!(result.is_err(), "expected error");

        let order_by_items = vec![OrderByItem {
            table_alias: None,
            field: "myfield1".to_string(),
            sequence: Sequence::Asc,
        }];
        let result = OrderByBuilder::build(order_by_items);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = result.unwrap();
        assert_eq!(result, "ORDER BY myfield1 ASC");

        let order_by_items = vec![
            OrderByItem {
                table_alias: Some("t".to_string()),
                field: "myfield1".to_string(),
                sequence: Sequence::Asc,
            },
            OrderByItem {
                table_alias: Some("t".to_string()),
                field: "myfield2".to_string(),
                sequence: Sequence::Desc,
            },
        ];
        let result = OrderByBuilder::build(order_by_items);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = result.unwrap();
        assert_eq!(result, "ORDER BY t.myfield1 ASC, t.myfield2 DESC");
    }
}
