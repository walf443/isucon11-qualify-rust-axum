use crate::responses::isu_condition_response::IsuConditionResponse;
use isucondition_core::models::isu::Isu;
use isucondition_core::services::isu_list_service::IsuWithCondition;
use serde::Serialize;

#[derive(Serialize)]
pub struct IsuResponse {
    id: String,
    jia_isu_uuid: String,
    name: String,
    character: Option<String>,
}

impl From<Isu> for IsuResponse {
    fn from(isu: Isu) -> Self {
        Self {
            id: isu.id.to_string(),
            jia_isu_uuid: isu.jia_isu_uuid.to_string(),
            name: isu.name,
            character: isu.character,
        }
    }
}

#[derive(Serialize)]
pub struct IsuWithConditionResponse {
    id: String,
    jia_isu_uuid: String,
    name: String,
    character: Option<String>,
    latest_isu_condition: Option<IsuConditionResponse>,
}

impl From<IsuWithCondition> for IsuWithConditionResponse {
    fn from((isu, latest_isu_condition): IsuWithCondition) -> Self {
        let condition: Option<IsuConditionResponse> = match latest_isu_condition {
            Some(condition) => Some((isu.clone(), condition).into()),
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
