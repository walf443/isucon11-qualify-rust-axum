use crate::repos::repository_manager::RepositoryManager;
use crate::services::isu_list_service::{IsuListService, IsuListServiceImpl};
use std::sync::Arc;

pub trait ServiceManager: Clone + std::marker::Send + std::marker::Sync {
    type Repo: 'static + RepositoryManager;
    type IsuListSrv: IsuListService<Self::Repo>;

    fn isu_list_service(&self) -> &Self::IsuListSrv;
}

#[derive(Clone)]
pub struct ServiceManagerImpl<R: RepositoryManager> {
    repo: Arc<R>,
    isu_list_service: IsuListServiceImpl<R>,
}

impl<R: 'static + RepositoryManager> ServiceManagerImpl<R> {
    pub fn new(repo: Arc<R>) -> Self {
        let isu_list_service = IsuListServiceImpl::from_repo(repo.clone());
        Self {
            repo,
            isu_list_service,
        }
    }
}

impl<R: 'static + RepositoryManager> ServiceManager for ServiceManagerImpl<R> {
    type Repo = R;
    type IsuListSrv = IsuListServiceImpl<Self::Repo>;

    fn isu_list_service(&self) -> &Self::IsuListSrv {
        &self.isu_list_service
    }
}

#[cfg(any(test, feature = "test"))]
pub mod tests {
    use crate::repos::repository_manager::tests::MockRepositoryManager;
    use crate::repos::repository_manager::RepositoryManager;
    use crate::services::isu_list_service::MockIsuListService;
    use crate::services::service_manager::ServiceManager;
    use std::sync::Arc;

    pub struct MockServiceManager<R: 'static + RepositoryManager> {
        pub repo: Arc<R>,
        pub isu_list_service: MockIsuListService<R>,
    }

    impl MockServiceManager<MockRepositoryManager> {
        pub fn new(repo: Arc<MockRepositoryManager>) -> Self {
            let isu_list_service = MockIsuListService::new();
            Self {
                repo,
                isu_list_service,
            }
        }
    }

    impl ServiceManager for MockServiceManager<MockRepositoryManager> {
        type Repo = MockRepositoryManager;
        type IsuListSrv = MockIsuListService<Self::Repo>;

        fn isu_list_service(&self) -> &Self::IsuListSrv {
            &self.isu_list_service
        }
    }

    impl<R: RepositoryManager> Clone for MockServiceManager<R> {
        // automock does not implement Clone trait, so give up copying mock expectation
        fn clone(&self) -> Self {
            let isu_list_service = MockIsuListService::new();
            Self {
                repo: self.repo.clone(),
                isu_list_service,
            }
        }
    }
}
