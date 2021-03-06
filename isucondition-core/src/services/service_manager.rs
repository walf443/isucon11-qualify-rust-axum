use crate::repos::repository_manager::RepositoryManager;
use crate::services::isu_list_service::{IsuListService, IsuListServiceImpl};
use crate::services::reset_database_service::{ResetDatabaseService, ResetDatabaseServiceImpl};
use crate::services::trend_list_service::{TrendListService, TrendListServiceImpl};
use std::sync::Arc;

pub trait ServiceManager: Clone + std::marker::Send + std::marker::Sync {
    type Repo: 'static + RepositoryManager;
    type IsuListSrv: IsuListService<Self::Repo>;
    type TrendListSrv: TrendListService<Self::Repo>;
    type ResetDatabaseSrv: ResetDatabaseService;

    fn isu_list_service(&self) -> &Self::IsuListSrv;
    fn trend_list_service(&self) -> &Self::TrendListSrv;
    fn reset_database_service(&self) -> &Self::ResetDatabaseSrv;
}

#[derive(Clone)]
pub struct ServiceManagerImpl<R: RepositoryManager> {
    isu_list_service: IsuListServiceImpl<R>,
    trend_list_service: TrendListServiceImpl<R>,
    reset_database_service: ResetDatabaseServiceImpl,
}

impl<R: 'static + RepositoryManager> ServiceManagerImpl<R> {
    pub fn new(repo: Arc<R>) -> Self {
        let isu_list_service = IsuListServiceImpl::from_repo(repo.clone());
        let trend_list_service = TrendListServiceImpl::from_repo(repo.clone());
        let reset_database_service = ResetDatabaseServiceImpl::new();

        Self {
            isu_list_service,
            trend_list_service,
            reset_database_service,
        }
    }
}

impl<R: 'static + RepositoryManager> ServiceManager for ServiceManagerImpl<R> {
    type Repo = R;
    type IsuListSrv = IsuListServiceImpl<Self::Repo>;
    type TrendListSrv = TrendListServiceImpl<Self::Repo>;
    type ResetDatabaseSrv = ResetDatabaseServiceImpl;

    fn isu_list_service(&self) -> &Self::IsuListSrv {
        &self.isu_list_service
    }
    fn trend_list_service(&self) -> &Self::TrendListSrv {
        &self.trend_list_service
    }
    fn reset_database_service(&self) -> &Self::ResetDatabaseSrv {
        &self.reset_database_service
    }
}

#[cfg(any(test, feature = "test"))]
pub mod tests {
    use crate::repos::repository_manager::tests::MockRepositoryManager;
    use crate::repos::repository_manager::RepositoryManager;
    use crate::services::isu_list_service::MockIsuListService;
    use crate::services::reset_database_service::MockResetDatabaseService;
    use crate::services::service_manager::ServiceManager;
    use crate::services::trend_list_service::MockTrendListService;
    use std::sync::Arc;

    pub struct MockServiceManager<R: 'static + RepositoryManager> {
        pub isu_list_service: MockIsuListService<R>,
        pub trend_list_service: MockTrendListService<R>,
        pub reset_database_service: MockResetDatabaseService,
    }

    impl MockServiceManager<MockRepositoryManager> {
        // _repo is ignored
        pub fn new(_repo: Arc<MockRepositoryManager>) -> Self {
            let isu_list_service = MockIsuListService::new();
            let trend_list_service = MockTrendListService::new();
            let reset_database_service = MockResetDatabaseService::new();

            Self {
                isu_list_service,
                trend_list_service,
                reset_database_service,
            }
        }
    }

    impl ServiceManager for MockServiceManager<MockRepositoryManager> {
        type Repo = MockRepositoryManager;
        type IsuListSrv = MockIsuListService<Self::Repo>;
        type TrendListSrv = MockTrendListService<Self::Repo>;
        type ResetDatabaseSrv = MockResetDatabaseService;

        fn isu_list_service(&self) -> &Self::IsuListSrv {
            &self.isu_list_service
        }

        fn trend_list_service(&self) -> &Self::TrendListSrv {
            &self.trend_list_service
        }

        fn reset_database_service(&self) -> &Self::ResetDatabaseSrv {
            &self.reset_database_service
        }
    }

    impl<R: RepositoryManager> Clone for MockServiceManager<R> {
        // automock does not implement Clone trait, so give up copying mock expectation
        fn clone(&self) -> Self {
            let isu_list_service = MockIsuListService::new();
            let trend_list_service = MockTrendListService::new();
            let reset_database_service = MockResetDatabaseService::new();

            Self {
                isu_list_service,
                trend_list_service,
                reset_database_service,
            }
        }
    }
}
