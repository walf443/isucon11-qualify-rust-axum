use crate::models::isu_condition::ConditionLevel;
use crate::models::trend::{IsuWithCondition, Trend};
use crate::repos;
use crate::repos::isu_condition_repository::IsuConditionRepository;
use crate::repos::isu_repository::IsuRepository;
use crate::repos::repository_manager::RepositoryManager;

pub struct TrendService<'r, R: RepositoryManager> {
    repo: &'r R,
}

impl<'r, R: RepositoryManager> TrendService<'r, R> {
    pub fn new(repo: &'r R) -> Self {
        Self { repo }
    }

    pub async fn run(&self, character: String) -> repos::Result<Trend> {
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

            let result = match condition {
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

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_run() {}
}
