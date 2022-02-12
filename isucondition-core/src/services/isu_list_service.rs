use crate::repos::isu_condition_repository::IsuConditionRepository;
use crate::repos::isu_repository::IsuRepository;
use crate::repos::repository_manager::RepositoryManager;
use crate::repos::Result;
use serde::Serialize;

pub struct IsuListService<R: RepositoryManager> {
    repo: R,
}

#[derive(Serialize)]
pub struct IsuEntity {
    id: String,
    jia_isu_uuid: String,
    name: String,
    character: Option<String>,
    latest_isu_condition: Option<IsuConditionEntity>,
}

#[derive(Serialize)]
pub struct IsuConditionEntity {
    jia_isu_uuid: String,
    isu_name: String,
    timestamp: i64,
    is_sitting: bool,
    condition: String,
    condition_level: &'static str,
    message: String,
}

impl<R: RepositoryManager> IsuListService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn run(&self, jia_user_id: String) -> Result<Vec<IsuEntity>> {
        let chairs = self
            .repo
            .isu_repository()
            .find_all_by_user_id(jia_user_id)
            .await?;

        let list: Vec<IsuEntity> = Vec::new();

        for chair in chairs {
            let last_isu_condition = self
                .repo
                .isu_condition_repository()
                .find_last_by_isu_id(&chair.jia_isu_uuid)
                .await?;

            let last_condition_entity = match last_isu_condition {
                Some(condition) => Some(IsuConditionEntity {
                    jia_isu_uuid: condition.jia_isu_uuid.to_string(),
                    isu_name: chair.name.clone(),
                    timestamp: condition.timestamp.timestamp(),
                    is_sitting: condition.is_sitting,
                    condition: condition.condition,
                    condition_level: "",
                    message: condition.message,
                }),
                None => None,
            };
        }

        Ok(list)
    }
}
