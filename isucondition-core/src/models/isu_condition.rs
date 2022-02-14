use crate::models::isu::IsuUUID;
use sqlx::types::time::PrimitiveDateTime;
use std::fmt::{Display, Formatter};

#[derive(Debug, sqlx::Type, PartialEq, Clone)]
pub struct IsuConditionID(String);

impl IsuConditionID {
    pub fn new(str: String) -> Self {
        Self { 0: str }
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
    pub timestamp: PrimitiveDateTime,
    pub created_at: Option<PrimitiveDateTime>,
}
