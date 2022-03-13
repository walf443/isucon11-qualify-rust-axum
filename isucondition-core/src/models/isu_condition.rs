use crate::models::isu::IsuUUID;
use chrono::NaiveDateTime;
use std::fmt::{Display, Formatter};

#[derive(Debug, sqlx::Type, PartialEq, Clone)]
pub struct IsuConditionID(u64);

impl IsuConditionID {
    pub fn new(num: u64) -> Self {
        Self { 0: num }
    }
}

impl Display for IsuConditionID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq)]
pub struct IsuCondition {
    pub id: IsuConditionID,
    pub jia_isu_uuid: IsuUUID,
    pub is_sitting: bool,
    pub condition: String,
    pub message: String,
    pub timestamp: NaiveDateTime,
    pub created_at: Option<NaiveDateTime>,
}
pub enum ConditionLevel {
    Info,
    Warning,
    Critical,
    Unknown,
}

impl ConditionLevel {
    pub fn to_string(&self) -> String {
        match self {
            ConditionLevel::Info => "info".to_string(),
            ConditionLevel::Warning => "warning".to_string(),
            ConditionLevel::Critical => "critical".to_string(),
            ConditionLevel::Unknown => panic!("invalid level unknown"),
        }
    }
}

impl IsuCondition {
    pub fn condition_level(&self) -> ConditionLevel {
        let warn_count = self.condition.matches("=true").count();
        match warn_count {
            0 => ConditionLevel::Info,
            1 | 2 => ConditionLevel::Warning,
            3 => ConditionLevel::Critical,
            _ => ConditionLevel::Unknown,
        }
    }
}
