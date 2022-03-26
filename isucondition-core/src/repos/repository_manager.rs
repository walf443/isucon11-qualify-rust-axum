use crate::database::DBConnectionPool;
use crate::repos;
use crate::repos::isu_association_config_repository::{
    IsuAssociationConfigRepository, IsuAssociationConfigRepositoryImpl,
};
use crate::repos::isu_condition_repository::{IsuConditionRepository, IsuConditionRepositoryImpl};
use crate::repos::isu_repository::{IsuRepository, IsuRepositoryImpl};
use crate::repos::transaction::{Transaction, TransactionImpl};
use crate::repos::user_repository::{UserRepository, UserRepositoryImpl};
use async_trait::async_trait;
use sqlx::MySql;

#[async_trait]
pub trait RepositoryManager: Clone + std::marker::Send + std::marker::Sync {
    type IsuRepo: IsuRepository + std::marker::Send + std::marker::Sync;
    type IsuAssociationConfigRepo: IsuAssociationConfigRepository;
    type IsuConditionRepo: IsuConditionRepository + std::marker::Send + std::marker::Sync;
    type UserRepo: UserRepository;
    type Tx<'t>: Transaction<'t>;

    // todo:  don't depend on db
    async fn get_transaction<'t>(&self) -> Result<Self::Tx<'t>, repos::Error>;
    fn isu_repository(&self) -> &Self::IsuRepo;
    fn isu_association_config_repository(&self) -> &Self::IsuAssociationConfigRepo;
    fn isu_condition_repository(&self) -> &Self::IsuConditionRepo;
    fn user_repository(&self) -> &Self::UserRepo;
}

#[derive(Clone)]
pub struct RepositoryManagerImpl {
    isu_repository: IsuRepositoryImpl,
    isu_association_config_repository: IsuAssociationConfigRepositoryImpl,
    isu_condition_repository: IsuConditionRepositoryImpl,
    user_repository: UserRepositoryImpl,
}

impl RepositoryManagerImpl {
    pub fn new(pool: DBConnectionPool) -> Self {
        let isu_repository = IsuRepositoryImpl { pool: pool.clone() };
        let isu_association_config_repository =
            IsuAssociationConfigRepositoryImpl { pool: pool.clone() };
        let isu_condition_repository = IsuConditionRepositoryImpl { pool: pool.clone() };
        let user_repository = UserRepositoryImpl { pool: pool.clone() };

        Self {
            isu_repository,
            isu_association_config_repository,
            isu_condition_repository,
            user_repository,
        }
    }
}

#[async_trait]
impl RepositoryManager for RepositoryManagerImpl {
    type IsuRepo = IsuRepositoryImpl;
    type IsuAssociationConfigRepo = IsuAssociationConfigRepositoryImpl;
    type IsuConditionRepo = IsuConditionRepositoryImpl;
    type UserRepo = UserRepositoryImpl;
    type Tx<'t> = TransactionImpl<'t>;

    async fn get_transaction<'t>(&self) -> Result<Self::Tx<'t>, repos::Error> {
        let tx = self.isu_repository.pool.begin().await?;
        Ok(TransactionImpl::new(tx))
    }

    fn isu_repository(&self) -> &Self::IsuRepo {
        &self.isu_repository
    }

    fn isu_association_config_repository(&self) -> &Self::IsuAssociationConfigRepo {
        &self.isu_association_config_repository
    }

    fn isu_condition_repository(&self) -> &Self::IsuConditionRepo {
        &self.isu_condition_repository
    }

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repository
    }
}

#[cfg(any(test, feature = "test"))]
pub mod tests {
    use crate::repos;
    use crate::repos::isu_association_config_repository::MockIsuAssociationConfigRepository;
    use crate::repos::isu_condition_repository::MockIsuConditionRepository;
    use crate::repos::isu_repository::MockIsuRepository;
    use crate::repos::repository_manager::RepositoryManager;
    use crate::repos::transaction::{Transaction, TransactionImpl};
    use crate::repos::user_repository::MockUserRepository;
    use crate::repos::Error;
    use async_trait::async_trait;
    use sqlx::MySql;

    impl Clone for MockIsuAssociationConfigRepository {
        fn clone(&self) -> Self {
            Self::new()
        }
    }

    impl Clone for MockIsuConditionRepository {
        fn clone(&self) -> Self {
            Self::new()
        }
    }

    impl Clone for MockIsuRepository {
        fn clone(&self) -> Self {
            Self::new()
        }
    }

    impl Clone for MockUserRepository {
        fn clone(&self) -> Self {
            Self::new()
        }
    }

    #[derive(Clone)]
    pub struct MockRepositoryManager {
        pub isu_repository: MockIsuRepository,
        pub isu_association_config_repository: MockIsuAssociationConfigRepository,
        pub isu_condition_repository: MockIsuConditionRepository,
        pub user_repository: MockUserRepository,
    }

    impl MockRepositoryManager {
        pub fn new() -> Self {
            Self {
                isu_repository: MockIsuRepository::new(),
                isu_association_config_repository: MockIsuAssociationConfigRepository::new(),
                isu_condition_repository: MockIsuConditionRepository::new(),
                user_repository: MockUserRepository::new(),
            }
        }
    }

    #[async_trait]
    impl RepositoryManager for MockRepositoryManager {
        type IsuRepo = MockIsuRepository;
        type IsuAssociationConfigRepo = MockIsuAssociationConfigRepository;
        type IsuConditionRepo = MockIsuConditionRepository;
        type UserRepo = MockUserRepository;
        type Tx<'t> = TransactionImpl<'t>;

        // you should use RepositoryImpl for transaction test
        async fn get_transaction<'t>(&self) -> Result<Self::Tx<'t>, Error> {
            unimplemented!();

            Err(repos::Error::TestError())
        }

        fn isu_repository(&self) -> &Self::IsuRepo {
            &self.isu_repository
        }

        fn isu_association_config_repository(&self) -> &Self::IsuAssociationConfigRepo {
            &self.isu_association_config_repository
        }

        fn isu_condition_repository(&self) -> &Self::IsuConditionRepo {
            &self.isu_condition_repository
        }

        fn user_repository(&self) -> &Self::UserRepo {
            &self.user_repository
        }
    }
}
