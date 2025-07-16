use crate::postgres::{
    ExpressionBuilder, GroupByBuilder, GroupByItem, JoinBuilder, JoinKind, OrderByBuilder,
    OrderByItem, WhereBuilder,
};
use anyhow::anyhow;
use serde_json::Value;

#[derive(Clone, Debug, Default)]
pub struct SelectBuilder {
    pub distinct: bool,
    pub table: String,
    pub fields: Vec<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub values: Vec<Value>,
    pub filter_statement: Option<String>,
    pub join_statement: Option<String>,
    pub group_by_statement: Option<String>,
    pub order_by_statement: Option<String>,
}

impl SelectBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn distinct(&mut self) -> &mut Self {
        self.distinct = true;
        self
    }

    pub fn table(&mut self, table: &str, table_alias: &str) -> &mut Self {
        self.table = format!("{table} as {table_alias}");
        self
    }


    pub fn join(
        &mut self,
        kind: JoinKind,
        table: &str,
        table_alias: &str,
        values: Vec<ExpressionBuilder>,
    ) -> &mut Self {
        if !values.is_empty() {
            let mut item = JoinBuilder::build(kind, table, table_alias, values);
            if !item.values.is_empty() {
                self.values.append(&mut item.values);
            }
            self.join_statement = if let Some(statement) = &self.join_statement {
                Some(format!("{} {}", statement, item.statement))
            } else {
                Some(item.statement)
            };
        }
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

    pub fn columns(&mut self, table_alias: &str, values: Vec<&str>) -> anyhow::Result<&mut Self> {
        if values.is_empty() {
            return Err(anyhow!("select field value is empty"));
        }
        let mut fields: Vec<String> = values
            .into_iter()
            .map(|value| format!("{table_alias}.{value}"))
            .collect();
        self.fields.append(&mut fields);
        Ok(self)
    }

    pub fn order_by(&mut self, values: Vec<OrderByItem>) -> anyhow::Result<&mut Self> {
        if !values.is_empty() {
            self.order_by_statement = Some(OrderByBuilder::build(values)?);
        }
        Ok(self)
    }

    pub fn group_by(&mut self, values: Vec<GroupByItem>) -> anyhow::Result<&mut Self> {
        if !values.is_empty() {
            self.group_by_statement = Some(GroupByBuilder::build(values)?);
        }
        Ok(self)
    }

    pub fn limit(&mut self, value: usize) -> &mut Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(&mut self, value: usize) -> &mut Self {
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
        let mut statement: String = if self.distinct {
            format!(
                "SELECT DISTINCT {} FROM {}",
                fields, self.table
            )
        } else {
            format!(
                "SELECT {} FROM {}",
                fields, self.table
            )
        };
        if let Some(value) = &self.join_statement {
            statement = format!("{statement} {value}");
        }
        if let Some(value) = &self.filter_statement {
            statement = format!("{statement} {value}");
        }
        if let Some(value) = &self.group_by_statement {
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
    use crate::postgres::{ConditionBuilder, ConditionValue, Logic, Operator, Sequence};

    #[tokio::test]
    async fn test_select_builder() {
        let mut builder = SelectBuilder::new();
        let result = builder.table("mytable","t").build();
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(result.unwrap(), "SELECT * FROM mytable as t");

        let mut builder = SelectBuilder::new();
        let result = builder.table("mytable","t").columns("t",vec![]);
        assert!(result.is_err(), "expecting error for fields");
        let result = builder.build();
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(result.unwrap(), "SELECT * FROM mytable as t");

        let mut builder = SelectBuilder::new();
        let result = builder.table("mytable","t").columns("t",vec!["myfield1", "myfield2"]);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = builder.build();
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(
            result.unwrap(),
            "SELECT t.myfield1, t.myfield2 FROM mytable as t"
        );

        let mut builder = SelectBuilder::new();
        let result = builder.table("mytable","t").columns("t",vec!["myfield1", "myfield2"]);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = builder.order_by(vec![OrderByItem {
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

        let mut builder = SelectBuilder::new();
        let result = builder.table("mytable","t").columns("t",vec!["myfield1", "myfield2"]);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = builder.order_by(vec![
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

        let mut builder = SelectBuilder::new();
        let result = builder.table("mytable","t").columns("t",vec!["myfield1", "myfield2"]);
        assert!(result.is_ok(), "{:?}", result.err());
        let result = builder.order_by(vec![
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
        let filter_clauses = vec![
            ConditionBuilder {
                table_alias: Some("t".to_string()),
                field: "myfield3".to_string(),
                operator: Operator::Eq,
                value: Some(ConditionValue::Single(Value::String("MYVALUE".to_string()))),
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
        let clause1 = ExpressionBuilder::build(filter_clauses.clone(), None);
        assert!(clause1.is_ok(), "{:?}", clause1.err());
        let clause2 = ExpressionBuilder::build(filter_clauses, Some(Logic::And));
        assert!(clause2.is_ok(), "{:?}", clause2.err());
        let result = builder
            .filter(vec![clause1.unwrap(), clause2.unwrap()])
            .limit(10)
            .offset(0)
            .build();
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(
            result.unwrap(),
            "SELECT t.myfield1, t.myfield2 FROM mytable as t WHERE (t.myfield3 = ? OR t.myfield5 IS NULL) AND (t.myfield3 = ? OR t.myfield5 IS NULL) ORDER BY t.myfield1 ASC, t.myfield2 DESC LIMIT 10 OFFSET 0"
        );
        assert!(builder.get_values().len() == 2);

        let mut builder = SelectBuilder::new();

        let join_clause = ExpressionBuilder::build(
            vec![ConditionBuilder {
                table_alias: Some("p".to_string()),
                field: "id".to_string(),
                operator: Operator::Eq,
                value: Some(ConditionValue::Field(
                    "o".to_string(),
                    "product_id".to_string(),
                )),
                logic: None,
            }],
            None,
        )
        .unwrap();
        let result = builder
            .table("orders","o")
            .join(JoinKind::Inner, "products", "p", vec![join_clause])
            .columns("o",vec!["id", "user_id", "product_id"])
            .unwrap()
            .limit(10)
            .offset(0)
            .build();
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(
            result.unwrap(),
            "SELECT o.id, o.user_id, o.product_id FROM orders as o INNER JOIN products as p ON p.id = o.product_id LIMIT 10 OFFSET 0"
        );

        let mut builder = SelectBuilder::new();

        let join_clause = ExpressionBuilder::build(
            vec![ConditionBuilder {
                table_alias: Some("p".to_string()),
                field: "id".to_string(),
                operator: Operator::Eq,
                value: Some(ConditionValue::Field(
                    "o".to_string(),
                    "product_id".to_string(),
                )),
                logic: None,
            }],
            None,
        )
        .unwrap();

        let filter_clause = ExpressionBuilder::build(
            vec![ConditionBuilder {
                table_alias: Some("o".to_string()),
                field: "id".to_string(),
                operator: Operator::Eq,
                value: Some(ConditionValue::Single(Value::Number(
                    Number::from_u128(1).unwrap(),
                ))),
                logic: None,
            }],
            None,
        )
        .unwrap();
        let result = builder
            .table("orders","o")
            .join(JoinKind::Left, "products", "p", vec![join_clause.clone()])
            .filter(vec![filter_clause.clone()])
            .columns("o",vec!["id", "user_id", "product_id"])
            .unwrap()
            .limit(10)
            .offset(0)
            .build();
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(
            result.unwrap(),
            "SELECT o.id, o.user_id, o.product_id FROM orders as o LEFT JOIN products as p ON p.id = o.product_id WHERE o.id = ? LIMIT 10 OFFSET 0"
        );

        let mut builder = SelectBuilder::new();
        let result = builder
            .table("orders","o")
            .join(JoinKind::Left, "products", "p", vec![join_clause])
            .filter(vec![filter_clause])
            .columns("o",vec!["id", "user_id", "product_id"])
            .unwrap()
            .order_by(vec![OrderByItem {
                table_alias: Some("o".to_string()),
                field: "user_id".to_string(),
                sequence: Sequence::Asc,
            }])
            .unwrap()
            .group_by(vec![GroupByItem {
                table_alias: Some("o".to_string()),
                field: "user_id".to_string(),
            }])
            .unwrap()
            .limit(10)
            .offset(0)
            .build();
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(
            result.unwrap(),
            "SELECT o.id, o.user_id, o.product_id FROM orders as o LEFT JOIN products as p ON p.id = o.product_id WHERE o.id = ? GROUP BY o.user_id ORDER BY o.user_id ASC LIMIT 10 OFFSET 0"
        );
    }
}
