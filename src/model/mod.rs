use crate::model::isu_association_config_repository::{
    IsuAssociationConfigRepository, IsuAssociationConfigRepositoryImpl,
};
use crate::model::isu_repository::{IsuRepository, IsuRepositoryImpl};
use crate::model::user_repository::{UserRepository, UserRepositoryImpl};
use sqlx::MySqlPool;

pub mod cleaner;
pub mod isu_association_config_repository;
pub mod isu_repository;
pub mod user_repository;

pub trait RepositoryManager: Clone + std::marker::Send + std::marker::Sync {
    type IsuRepo: IsuRepository;
    type IsuAssociationConfigRepo: IsuAssociationConfigRepository;
    type UserRepo: UserRepository;

    fn isu_repository(&self) -> &Self::IsuRepo;
    fn isu_accosiation_config_repository(&self) -> &Self::IsuAssociationConfigRepo;
    fn user_repository(&self) -> &Self::UserRepo;
}

#[derive(Clone)]
pub struct RepositoryManagerImpl {
    isu_repository: IsuRepositoryImpl,
    isu_accosication_config_repository: IsuAssociationConfigRepositoryImpl,
    user_repository: UserRepositoryImpl,
}

impl RepositoryManagerImpl {
    pub fn new(pool: MySqlPool) -> Self {
        let isu_repoository = IsuRepositoryImpl { pool: pool.clone() };
        let isu_association_config_repository =
            IsuAssociationConfigRepositoryImpl { pool: pool.clone() };
        let user_repository = UserRepositoryImpl { pool: pool.clone() };

        Self {
            isu_repository: isu_repoository,
            isu_accosication_config_repository: isu_association_config_repository,
            user_repository: user_repository,
        }
    }
}

impl RepositoryManager for RepositoryManagerImpl {
    type IsuRepo = IsuRepositoryImpl;
    type IsuAssociationConfigRepo = IsuAssociationConfigRepositoryImpl;
    type UserRepo = UserRepositoryImpl;

    fn isu_repository(&self) -> &Self::IsuRepo {
        &self.isu_repository
    }

    fn isu_accosiation_config_repository(&self) -> &Self::IsuAssociationConfigRepo {
        &self.isu_accosication_config_repository
    }

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repository
    }
}
