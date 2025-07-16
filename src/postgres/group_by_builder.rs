use anyhow::anyhow;

#[derive(Clone, Debug)]
pub struct GroupByItem {
    pub table_alias: Option<String>,
    pub field: String,
}

#[derive(Clone, Debug)]
pub struct GroupByBuilder;

impl GroupByBuilder {
    pub fn build(values: Vec<GroupByItem>) -> anyhow::Result<String> {
        if values.is_empty() {
            return Err(anyhow!("group by item is empty"));
        }
        let mut group_by: Vec<String> = Vec::new();
        for item in values.into_iter() {
            let table_alias = if let Some(value) = item.table_alias {
                &format!("{value}.")
            } else {
                ""
            };
            if item.field.is_empty() {
                return Err(anyhow!("group by field is empty"));
            }
            let value = format!("{}{}", table_alias, item.field);
            if !group_by.contains(&value) {
                group_by.push(value);
            }
        }
        Ok(format!("GROUP BY {}", group_by.join(", ").trim()))
    }
}

#[cfg(test)]
pub mod test_group_by_builder {
    use super::*;

    #[tokio::test]
    async fn test_group_by_builder() {
        let group_by = GroupByItem {
            table_alias: None,
            field: "".to_string(),
        };
        let result = GroupByBuilder::build(vec![group_by]);
        assert!(result.is_err(), "expected error");

        let group_by_items = vec![GroupByItem {
            table_alias: None,
            field: "myfield1".to_string(),
        }];
        let result = GroupByBuilder::build(group_by_items);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = result.unwrap();
        assert_eq!(result, "GROUP BY myfield1");

        let group_by_items = vec![
            GroupByItem {
                table_alias: Some("t".to_string()),
                field: "myfield1".to_string(),
            },
            GroupByItem {
                table_alias: Some("t".to_string()),
                field: "myfield2".to_string(),
            },
        ];
        let result = GroupByBuilder::build(group_by_items);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = result.unwrap();
        assert_eq!(result, "GROUP BY t.myfield1, t.myfield2");
    }
}
