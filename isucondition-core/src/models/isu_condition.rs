use sqlx::types::time::PrimitiveDateTime;
use sqlx::Error;

#[derive(Debug, PartialEq)]
pub struct IsuCondition {
    pub id: i64,
    pub jia_isu_uuid: String,
    pub is_sitting: bool,
    pub condition: String,
    pub message: String,
    pub timestamp: PrimitiveDateTime,
    pub created_at: Option<PrimitiveDateTime>,
}
