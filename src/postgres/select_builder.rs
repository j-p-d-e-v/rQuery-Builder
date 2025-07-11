use crate::postgres::{OrderByBuilder, OrderByItem, WhereBuilder, WhereClause};
use anyhow::anyhow;
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct SelectBuilder {
    pub table: String,
    pub fields: Vec<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub values: Vec<Value>,
    pub where_statement: Option<String>,
    pub order_by_statement: Option<String>,
}

impl SelectBuilder {
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            fields: Vec::new(),
            limit: None,
            offset: None,
            values: Vec::new(),
            where_statement: None,
            order_by_statement: None,
        }
    }

    pub fn set_where(&mut self, values: Vec<WhereClause>) -> &mut Self {
        if !values.is_empty() {
            let mut result = WhereBuilder::build(values);
            self.where_statement = Some(result.statement);
            if !result.values.is_empty() {
                self.values.append(&mut result.values);
            }
        }
        self
    }

    pub fn set_fields(&mut self, values: Vec<&str>) -> anyhow::Result<&mut Self> {
        if values.is_empty() {
            return Err(anyhow!("select field value is empty"));
        }
        let mut fields: Vec<String> = values.into_iter().map(|value| value.to_string()).collect();
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
        let mut statement: String = format!("SELECT {} FROM {}", fields, self.table);

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
    use crate::postgres::{Condition, Operator, Sequence, WhereClauseItem};

    #[tokio::test]
    async fn test_select_builder() {
        let builder = SelectBuilder::new("mytable");
        let result = builder.build();
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(result.unwrap(), "SELECT * FROM mytable");

        let mut builder = SelectBuilder::new("mytable");
        let result = builder.set_fields(vec![]);
        assert!(result.is_err(), "expecting error for fields");
        let result = builder.build();
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(result.unwrap(), "SELECT * FROM mytable");

        let mut builder = SelectBuilder::new("mytable");
        let result = builder.set_fields(vec!["myfield1", "myfield2"]);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = builder.build();
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(result.unwrap(), "SELECT myfield1, myfield2 FROM mytable");

        let mut builder = SelectBuilder::new("mytable");
        let result = builder.set_fields(vec!["myfield1", "myfield2"]);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = builder.set_order_by(vec![OrderByItem {
            field: "myfield1".to_string(),
            sequence: Sequence::Asc,
        }]);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = builder.build();
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(
            result.unwrap(),
            "SELECT myfield1, myfield2 FROM mytable ORDER BY myfield1 ASC"
        );

        let mut builder = SelectBuilder::new("mytable");
        let result = builder.set_fields(vec!["myfield1", "myfield2"]);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = builder.set_order_by(vec![
            OrderByItem {
                field: "myfield1".to_string(),
                sequence: Sequence::Asc,
            },
            OrderByItem {
                field: "myfield2".to_string(),
                sequence: Sequence::Desc,
            },
        ]);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = builder.build();
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(
            result.unwrap(),
            "SELECT myfield1, myfield2 FROM mytable ORDER BY myfield1 ASC, myfield2 DESC"
        );

        let mut builder = SelectBuilder::new("mytable");
        let result = builder.set_fields(vec!["myfield1", "myfield2"]);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = builder.set_order_by(vec![
            OrderByItem {
                field: "myfield1".to_string(),
                sequence: Sequence::Asc,
            },
            OrderByItem {
                field: "myfield2".to_string(),
                sequence: Sequence::Desc,
            },
        ]);
        assert!(result.is_ok(), "{:?}", result.err());
        let where_clauses = vec![
            WhereClauseItem {
                field: "myfield3".to_string(),
                operator: Operator::Eq,
                value: Some(Value::String("MYVALUE".to_string())),
                condition: None,
            },
            WhereClauseItem {
                field: "myfield5".to_string(),
                operator: Operator::IsNull,
                value: None,
                condition: Some(Condition::Or),
            },
        ];
        let clause1 = WhereClause::build(where_clauses.clone(), None);
        assert!(clause1.is_ok(), "{:?}", clause1.err());
        let clause2 = WhereClause::build(where_clauses, Some(Condition::And));
        assert!(clause2.is_ok(), "{:?}", clause2.err());
        let result = builder
            .set_where(vec![clause1.unwrap(), clause2.unwrap()])
            .set_limit(10)
            .set_offset(0)
            .build();
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(
            result.unwrap(),
            "SELECT myfield1, myfield2 FROM mytable WHERE (myfield3 = ? OR myfield5 IS NULL) AND (myfield3 = ? OR myfield5 IS NULL) ORDER BY myfield1 ASC, myfield2 DESC LIMIT 10 OFFSET 0"
        );
        assert!(builder.get_values().len() == 2);

        let mut builder = SelectBuilder::new("configuration_items");

        let where_clause = WhereClause::build(
            vec![
                WhereClauseItem {
                    field: "project_id".to_string(),
                    operator: Operator::Eq,
                    value: Some(Value::Number(Number::from_u128(1).unwrap())),
                    condition: None,
                },
                WhereClauseItem {
                    field: "location_id".to_string(),
                    operator: Operator::In,
                    value: Some(Value::Array(vec![Value::Number(
                        Number::from_u128(1).unwrap(),
                    )])),
                    condition: Some(Condition::And),
                },
            ],
            None,
        )
        .unwrap();
        let result = builder
            .set_where(vec![where_clause])
            .set_fields(vec!["ci_number", "name", "project_id"])
            .unwrap()
            .set_limit(10)
            .set_offset(0)
            .build();
        //println!("{:?}", result);
        //println!("{:?}", builder.get_values());
    }
}
