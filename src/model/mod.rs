use crate::model::isu_association_config_repository::{
    IsuAssociationConfigRepository, IsuAssociationConfigRepositoryImpl,
};
use crate::model::user_repository::{UserRepository, UserRepositoryImpl};
use sqlx::MySqlPool;

pub mod cleaner;
pub mod isu_association_config_repository;
pub mod user_repository;

pub trait RepositoryManager {
    type IsuAssociationConfigRepo: IsuAssociationConfigRepository;
    type UserRepo: UserRepository;

    fn isu_accosiation_config_repository(&self) -> &Self::IsuAssociationConfigRepo;
    fn user_repository(&self) -> &Self::UserRepo;
}

pub struct RepositoryManagerImpl<'a> {
    isu_accosication_config_repository: IsuAssociationConfigRepositoryImpl,
    user_repository: UserRepositoryImpl<'a>,
}

impl<'a> RepositoryManagerImpl<'a> {
    pub fn new(pool: &'a MySqlPool) -> Self {
        let isu_association_config_repository =
            IsuAssociationConfigRepositoryImpl { pool: pool.clone() };
        let user_repository = UserRepositoryImpl { pool };

        Self {
            isu_accosication_config_repository: isu_association_config_repository,
            user_repository: user_repository,
        }
    }
}

impl<'a> RepositoryManager for RepositoryManagerImpl<'a> {
    type IsuAssociationConfigRepo = IsuAssociationConfigRepositoryImpl;
    type UserRepo = UserRepositoryImpl<'a>;

    fn isu_accosiation_config_repository(&self) -> &Self::IsuAssociationConfigRepo {
        &self.isu_accosication_config_repository
    }

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repository
    }
}
