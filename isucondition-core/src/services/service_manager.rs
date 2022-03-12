use crate::repos::repository_manager::tests::MockRepositoryManager;
use crate::repos::repository_manager::{RepositoryManager, RepositoryManagerImpl};
use crate::services::isu_list_service::{IsuListService, IsuListServiceImpl};

pub trait ServiceManager {
    type IsuListSrv: IsuListService;

    fn isu_list_service(&self) -> Self::IsuListSrv;
}

pub struct ServiceManagerImpl<R: RepositoryManager> {
    repo: R,
}

impl<R: RepositoryManager> ServiceManagerImpl<R> {
    pub fn new(repo: R) -> Self {
        Self { repo: repo }
    }
}

impl<R: RepositoryManager> ServiceManager for ServiceManagerImpl<R> {
    type IsuListSrv = IsuListServiceImpl<R>;

    fn isu_list_service(&self) -> Self::IsuListSrv {
        Self::IsuListSrv::new(self.repo.clone())
    }
}
