use crate::model::isu_association_config_repository::{
    IsuAssociationConfigRepository, IsuAssociationConfigRepositoryImpl,
};
use crate::model::user_repository::{UserRepository, UserRepositoryImpl};
use sqlx::MySqlPool;
use std::sync::Arc;

pub mod cleaner;
pub mod isu_association_config_repository;
pub mod user_repository;

pub trait RepositoryManager: Clone + std::marker::Send + std::marker::Sync {
    type IsuAssociationConfigRepo: IsuAssociationConfigRepository;
    type UserRepo: UserRepository;

    fn isu_accosiation_config_repository(&self) -> &Self::IsuAssociationConfigRepo;
    fn user_repository(&self) -> &Self::UserRepo;
}

#[derive(Clone)]
pub struct RepositoryManagerImpl {
    isu_accosication_config_repository: IsuAssociationConfigRepositoryImpl,
    user_repository: UserRepositoryImpl,
}

impl RepositoryManagerImpl {
    pub fn new(pool: MySqlPool) -> Self {
        let isu_association_config_repository = IsuAssociationConfigRepositoryImpl { pool: pool.clone() };
        let user_repository = UserRepositoryImpl { pool: pool.clone() };

        Self {
            isu_accosication_config_repository: isu_association_config_repository,
            user_repository: user_repository,
        }
    }
}

impl RepositoryManager for RepositoryManagerImpl {
    type IsuAssociationConfigRepo = IsuAssociationConfigRepositoryImpl;
    type UserRepo = UserRepositoryImpl;

    fn isu_accosiation_config_repository(&self) -> &Self::IsuAssociationConfigRepo {
        &self.isu_accosication_config_repository
    }

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repository
    }
}
