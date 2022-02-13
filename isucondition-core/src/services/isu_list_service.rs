use crate::models::isu::Isu;
use crate::models::isu_condition::IsuCondition;
use crate::models::user::UserID;
use crate::repos::isu_condition_repository::IsuConditionRepository;
use crate::repos::isu_repository::IsuRepository;
use crate::repos::repository_manager::RepositoryManager;
use crate::repos::Result;

pub type IsuWithCondition = (Isu, Option<IsuCondition>);

pub struct IsuListService<R: RepositoryManager> {
    repo: R,
}

impl<R: RepositoryManager> IsuListService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn run(&self, jia_user_id: UserID) -> Result<Vec<IsuWithCondition>> {
        let chairs = self
            .repo
            .isu_repository()
            .find_all_by_user_id(jia_user_id)
            .await?;

        let mut list: Vec<IsuWithCondition> = Vec::new();

        for chair in chairs {
            let last_isu_condition = self
                .repo
                .isu_condition_repository()
                .find_last_by_isu_id(&chair.jia_isu_uuid)
                .await?;

            list.push((chair, last_isu_condition));
        }

        Ok(list)
    }
}
