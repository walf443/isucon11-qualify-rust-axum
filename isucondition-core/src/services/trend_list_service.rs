use crate::models::isu_condition::ConditionLevel;
use crate::models::trend::{IsuWithCondition, Trend};
use crate::repos;
use crate::repos::isu_condition_repository::IsuConditionRepository;
use crate::repos::isu_repository::IsuRepository;
use crate::repos::repository_manager::RepositoryManager;
use async_trait::async_trait;
use std::sync::Arc;

#[cfg(test)]
mod tests;

#[cfg_attr(any(test, feature = "test"), mockall::automock)]
#[async_trait]
pub trait TrendListService<R: 'static + RepositoryManager> {
    fn from_repo(repo: Arc<R>) -> Self;
    async fn run(&self) -> repos::Result<Vec<Trend>>;
}

#[derive(Clone)]
pub struct TrendListServiceImpl<R: RepositoryManager> {
    repo: Arc<R>,
}

#[async_trait]
impl<R: 'static + RepositoryManager> TrendListService<R> for TrendListServiceImpl<R> {
    fn from_repo(repo: Arc<R>) -> Self {
        Self { repo }
    }

    async fn run(&self) -> repos::Result<Vec<Trend>> {
        let character_list = self.repo.isu_repository().find_character_group_by().await?;

        let mut trends: Vec<Trend> = Vec::new();

        for character in character_list {
            if !character.is_none() {
                let character = character.unwrap();
                let trend = self.get_trend(character).await?;

                trends.push(trend);
            }
        }

        Ok(trends)
    }
}

impl<R: RepositoryManager> TrendListServiceImpl<R> {
    async fn get_trend(&self, character: String) -> repos::Result<Trend> {
        let isu_list = self
            .repo
            .isu_repository()
            .find_all_by_character(&character)
            .await?;

        let mut info_conditions: Vec<IsuWithCondition> = Vec::new();
        let mut warning_conditions: Vec<IsuWithCondition> = Vec::new();
        let mut critical_conditions: Vec<IsuWithCondition> = Vec::new();

        for isu in isu_list {
            let condition = self
                .repo
                .isu_condition_repository()
                .find_last_by_isu_id(&isu.jia_isu_uuid)
                .await?;

            match condition {
                Some(last_isu_condition) => {
                    let level = last_isu_condition.condition_level();

                    match level {
                        ConditionLevel::Info => info_conditions.push((isu, last_isu_condition)),
                        ConditionLevel::Warning => {
                            warning_conditions.push((isu, last_isu_condition))
                        }
                        ConditionLevel::Critical => {
                            critical_conditions.push((isu, last_isu_condition))
                        }
                        ConditionLevel::Unknown => {}
                    };
                }
                None => (),
            };
        }

        info_conditions.sort_by_key(|(_, cond)| std::cmp::Reverse(cond.timestamp));
        warning_conditions.sort_by_key(|(_, cond)| std::cmp::Reverse(cond.timestamp));
        critical_conditions.sort_by_key(|(_, cond)| std::cmp::Reverse(cond.timestamp));

        Ok(Trend {
            character,
            info: info_conditions,
            warning: warning_conditions,
            critical: critical_conditions,
        })
    }
}
