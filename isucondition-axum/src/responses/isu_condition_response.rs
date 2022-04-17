use isucondition_core::models::isu::Isu;
use isucondition_core::models::isu_condition::IsuCondition;
use serde::Serialize;

#[derive(Serialize)]
pub struct IsuConditionResponse {
    jia_isu_uuid: String,
    isu_name: String,
    timestamp: i64,
    is_sitting: bool,
    condition: String,
    condition_level: String,
    message: String,
}

impl From<(Isu, IsuCondition)> for IsuConditionResponse {
    fn from((isu, cond): (Isu, IsuCondition)) -> Self {
        Self {
            jia_isu_uuid: isu.jia_isu_uuid.to_string(),
            isu_name: isu.name,
            timestamp: cond.timestamp.timestamp(),
            is_sitting: cond.is_sitting,
            condition: cond.condition.clone(),
            condition_level: cond.condition_level().to_string(),
            message: cond.message,
        }
    }
}
