use isucondition_core::services::isu_list_service::IsuWithCondition;
use serde::Serialize;

#[derive(Serialize)]
pub struct IsuResponse {
    id: String,
    jia_isu_uuid: String,
    name: String,
    character: Option<String>,
    latest_isu_condition: Option<IsuConditionResponse>,
}

#[derive(Serialize)]
struct IsuConditionResponse {
    jia_isu_uuid: String,
    isu_name: String,
    timestamp: i64,
    is_sitting: bool,
    condition: String,
    condition_level: &'static str,
    message: String,
}

impl From<IsuWithCondition> for IsuResponse {
    fn from((isu, latest_isu_condition): IsuWithCondition) -> Self {
        let condition = match latest_isu_condition {
            Some(condition) => Some(IsuConditionResponse {
                jia_isu_uuid: condition.jia_isu_uuid.to_string(),
                isu_name: isu.name.clone(),
                timestamp: condition.timestamp.timestamp(),
                is_sitting: condition.is_sitting,
                condition: condition.condition,
                condition_level: "",
                message: condition.message,
            }),
            None => None,
        };
        Self {
            id: isu.id.to_string(),
            jia_isu_uuid: isu.jia_isu_uuid.to_string(),
            name: isu.name,
            character: isu.character,
            latest_isu_condition: condition,
        }
    }
}
