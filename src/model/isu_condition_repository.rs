use sqlx::MySqlPool;

pub trait IsuConditionRepository {
}

#[derive(Clone)]
pub struct IsuConditionRepositoryImpl {
    pub pool: MySqlPool,
}

impl IsuConditionRepository for IsuConditionRepositoryImpl {
}
