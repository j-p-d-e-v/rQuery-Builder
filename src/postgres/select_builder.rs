use crate::postgres::{ExpressionBuilder, OrderByBuilder, OrderByItem, WhereBuilder};
use anyhow::anyhow;
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct SelectBuilder {
    pub table: String,
    pub table_alias: String,
    pub fields: Vec<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub values: Vec<Value>,
    pub where_statement: Option<String>,
    pub order_by_statement: Option<String>,
}

impl SelectBuilder {
    pub fn new(table: &str, table_alias: &str) -> Self {
        Self {
            table: table.to_string(),
            table_alias: table_alias.to_string(),
            fields: Vec::new(),
            limit: None,
            offset: None,
            values: Vec::new(),
            where_statement: None,
            order_by_statement: None,
        }
    }

    pub fn set_where(&mut self, values: Vec<ExpressionBuilder>) -> &mut Self {
        if !values.is_empty() {
            let mut result = WhereBuilder::build(values);
            self.where_statement = Some(result.statement);
            if !result.values.is_empty() {
                self.values.append(&mut result.values);
            }
        }
        self
    }

    pub fn set_columns(&mut self, values: Vec<&str>) -> anyhow::Result<&mut Self> {
        if values.is_empty() {
            return Err(anyhow!("select field value is empty"));
        }
        let table_alias = &self.table_alias;
        let mut fields: Vec<String> = values
            .into_iter()
            .map(|value| format!("{table_alias}.{value}"))
            .collect();
        self.fields.append(&mut fields);
        Ok(self)
    }

    pub fn set_order_by(&mut self, values: Vec<OrderByItem>) -> anyhow::Result<&mut Self> {
        if !values.is_empty() {
            self.order_by_statement = Some(OrderByBuilder::build(values)?);
        }
        Ok(self)
    }

    pub fn set_limit(&mut self, value: usize) -> &mut Self {
        self.limit = Some(value);
        self
    }

    pub fn set_offset(&mut self, value: usize) -> &mut Self {
        self.offset = Some(value);
        self
    }

    pub fn get_values(&self) -> Vec<Value> {
        self.values.to_owned()
    }

    pub fn build(&self) -> anyhow::Result<String> {
        let fields = if self.fields.is_empty() {
            "*".to_string()
        } else {
            self.fields.join(", ")
        };
        let mut statement: String = format!(
            "SELECT {} FROM {} as {}",
            fields, self.table, self.table_alias
        );

        if let Some(value) = &self.where_statement {
            statement = format!("{statement} {value}");
        }

        if let Some(value) = &self.order_by_statement {
            statement = format!("{statement} {value}");
        }
        if let Some(value) = &self.limit {
            statement = format!("{statement} LIMIT {value}");
        }
        if let Some(value) = &self.offset {
            statement = format!("{statement} OFFSET {value}");
        }

        Ok(statement.trim().to_string())
    }
}
#[cfg(test)]
pub mod test_select_builder {
    use serde_json::Number;

    use super::*;
    use crate::postgres::{ConditionBuilder, Logic, Operator, Sequence};

    #[tokio::test]
    async fn test_select_builder() {
        let builder = SelectBuilder::new("mytable", "t");
        let result = builder.build();
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(result.unwrap(), "SELECT * FROM mytable as t");

        let mut builder = SelectBuilder::new("mytable", "t");
        let result = builder.set_columns(vec![]);
        assert!(result.is_err(), "expecting error for fields");
        let result = builder.build();
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(result.unwrap(), "SELECT * FROM mytable as t");

        let mut builder = SelectBuilder::new("mytable", "t");
        let result = builder.set_columns(vec!["myfield1", "myfield2"]);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = builder.build();
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(
            result.unwrap(),
            "SELECT t.myfield1, t.myfield2 FROM mytable as t"
        );

        let mut builder = SelectBuilder::new("mytable", "t");
        let result = builder.set_columns(vec!["myfield1", "myfield2"]);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = builder.set_order_by(vec![OrderByItem {
            table_alias: Some("t".to_string()),
            field: "myfield1".to_string(),
            sequence: Sequence::Asc,
        }]);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = builder.build();
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(
            result.unwrap(),
            "SELECT t.myfield1, t.myfield2 FROM mytable as t ORDER BY t.myfield1 ASC"
        );

        let mut builder = SelectBuilder::new("mytable", "t");
        let result = builder.set_columns(vec!["myfield1", "myfield2"]);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = builder.set_order_by(vec![
            OrderByItem {
                table_alias: None,
                field: "myfield1".to_string(),
                sequence: Sequence::Asc,
            },
            OrderByItem {
                table_alias: None,
                field: "myfield2".to_string(),
                sequence: Sequence::Desc,
            },
        ]);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = builder.build();
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(
            result.unwrap(),
            "SELECT t.myfield1, t.myfield2 FROM mytable as t ORDER BY myfield1 ASC, myfield2 DESC"
        );

        let mut builder = SelectBuilder::new("mytable", "t");
        let result = builder.set_columns(vec!["myfield1", "myfield2"]);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = builder.set_order_by(vec![
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
        ]);
        assert!(result.is_ok(), "{:?}", result.err());
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
                field: "myfield5".to_string(),
                operator: Operator::IsNull,
                value: None,
                logic: Some(Logic::Or),
            },
        ];
        let clause1 = ExpressionBuilder::build(where_clauses.clone(), None);
        assert!(clause1.is_ok(), "{:?}", clause1.err());
        let clause2 = ExpressionBuilder::build(where_clauses, Some(Logic::And));
        assert!(clause2.is_ok(), "{:?}", clause2.err());
        let result = builder
            .set_where(vec![clause1.unwrap(), clause2.unwrap()])
            .set_limit(10)
            .set_offset(0)
            .build();
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(
            result.unwrap(),
            "SELECT t.myfield1, t.myfield2 FROM mytable as t WHERE (t.myfield3 = ? OR t.myfield5 IS NULL) AND (t.myfield3 = ? OR t.myfield5 IS NULL) ORDER BY t.myfield1 ASC, t.myfield2 DESC LIMIT 10 OFFSET 0"
        );
        assert!(builder.get_values().len() == 2);

        let mut builder = SelectBuilder::new("orders", "o");

        let where_clause = ExpressionBuilder::build(
            vec![
                ConditionBuilder {
                    table_alias: Some("o".to_string()),
                    field: "product_id".to_string(),
                    operator: Operator::Eq,
                    value: Some(Value::Number(Number::from_u128(1).unwrap())),
                    logic: None,
                },
                ConditionBuilder {
                    table_alias: Some("o".to_string()),
                    field: "user_id".to_string(),
                    operator: Operator::In,
                    value: Some(Value::Array(vec![Value::Number(
                        Number::from_u128(1).unwrap(),
                    )])),
                    logic: Some(Logic::And),
                },
            ],
            None,
        )
        .unwrap();
        let result = builder
            .set_where(vec![where_clause])
            .set_columns(vec!["id", "user_id", "product_id"])
            .unwrap()
            .set_limit(10)
            .set_offset(0)
            .build();
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(result.unwrap(),"SELECT o.id, o.user_id, o.product_id FROM orders as o WHERE o.product_id = ? AND o.user_id IN (?) LIMIT 10 OFFSET 0");
    }
}
