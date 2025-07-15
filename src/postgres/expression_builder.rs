use crate::postgres::{ConditionBuilder, Logic};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExpressionBuilder {
    pub condition: String,
    pub logic: Option<Logic>,
    pub values: Vec<Value>,
}

impl ExpressionBuilder {
    pub fn build(
        values: Vec<ConditionBuilder>,
        logic: Option<Logic>,
    ) -> anyhow::Result<ExpressionBuilder> {
        let mut data: ExpressionBuilder = ExpressionBuilder::default();
        for item in values {
            let mut condition = ConditionBuilder::format(&item)?;
            if let Some(logic) = item.logic {
                condition = format!("{logic} {condition}");
            }
            if data.condition.is_empty() {
                data.condition = condition;
            } else {
                data.condition = format!("{} {}", data.condition, condition)
            }
            if let Some(value) = item.value {
                data.values.push(value);
            }
        }
        data.logic = logic;
        Ok(data)
    }
}
