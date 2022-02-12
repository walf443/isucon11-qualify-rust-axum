use crate::model::isu_association_config_repository::{
    IsuAssociationConfigRepository, IsuAssociationConfigRepositoryImpl,
};
use crate::model::isu_condition_repository::{IsuConditionRepository, IsuConditionRepositoryImpl};
use crate::model::isu_repository::{IsuRepository, IsuRepositoryImpl};
use sqlx::MySqlPool;
pub use isucondition_core::repos::user_repository;
use isucondition_core::repos::user_repository::{UserRepository, UserRepositoryImpl};

pub mod cleaner;
pub mod isu_association_config_repository;
pub mod isu_condition;
pub mod isu_condition_repository;
pub mod isu_repository;

pub trait RepositoryManager: Clone + std::marker::Send + std::marker::Sync {
    type IsuRepo: IsuRepository;
    type IsuAssociationConfigRepo: IsuAssociationConfigRepository;
    type IsuConditionRepo: IsuConditionRepository;
    type UserRepo: UserRepository;

    fn isu_repository(&self) -> &Self::IsuRepo;
    fn isu_accosiation_config_repository(&self) -> &Self::IsuAssociationConfigRepo;
    fn isu_condition_repository(&self) -> &Self::IsuConditionRepo;
    fn user_repository(&self) -> &Self::UserRepo;
}

#[derive(Clone)]
pub struct RepositoryManagerImpl {
    isu_repository: IsuRepositoryImpl,
    isu_accosication_config_repository: IsuAssociationConfigRepositoryImpl,
    isu_condition_repository: IsuConditionRepositoryImpl,
    user_repository: UserRepositoryImpl,
}

impl RepositoryManagerImpl {
    pub fn new(pool: MySqlPool) -> Self {
        let isu_repoository = IsuRepositoryImpl { pool: pool.clone() };
        let isu_association_config_repository =
            IsuAssociationConfigRepositoryImpl { pool: pool.clone() };
        let isu_condition_repository = IsuConditionRepositoryImpl { pool: pool.clone() };
        let user_repository = UserRepositoryImpl { pool: pool.clone() };

        Self {
            isu_repository: isu_repoository,
            isu_accosication_config_repository: isu_association_config_repository,
            isu_condition_repository: isu_condition_repository,
            user_repository: user_repository,
        }
    }
}

impl RepositoryManager for RepositoryManagerImpl {
    type IsuRepo = IsuRepositoryImpl;
    type IsuAssociationConfigRepo = IsuAssociationConfigRepositoryImpl;
    type IsuConditionRepo = IsuConditionRepositoryImpl;
    type UserRepo = UserRepositoryImpl;

    fn isu_repository(&self) -> &Self::IsuRepo {
        &self.isu_repository
    }

    fn isu_accosiation_config_repository(&self) -> &Self::IsuAssociationConfigRepo {
        &self.isu_accosication_config_repository
    }

    fn isu_condition_repository(&self) -> &Self::IsuConditionRepo {
        &self.isu_condition_repository
    }

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repository
    }
}
