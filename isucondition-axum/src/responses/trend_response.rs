use isucondition_core::models::trend::{IsuWithCondition, Trend};
use serde::Serialize;

#[derive(Serialize)]
pub struct TrendResponse {
    character: String,
    info: Vec<TrendConditionResponse>,
    warning: Vec<TrendConditionResponse>,
    critical: Vec<TrendConditionResponse>,
}

#[derive(Serialize)]
pub struct TrendConditionResponse {
    isu_id: i64,
    timestamp: i64,
}

impl From<Trend> for TrendResponse {
    fn from(trend: Trend) -> Self {
        Self {
            character: trend.character,
            info: trend.info.into_iter().map(|cond| cond.into()).collect(),
            warning: trend.warning.into_iter().map(|cond| cond.into()).collect(),
            critical: trend.critical.into_iter().map(|cond| cond.into()).collect(),
        }
    }
}

impl From<IsuWithCondition> for TrendConditionResponse {
    fn from((isu, cond): IsuWithCondition) -> Self {
        Self {
            isu_id: isu.id.to_i64(),
            timestamp: cond.timestamp.timestamp(),
        }
    }
}
