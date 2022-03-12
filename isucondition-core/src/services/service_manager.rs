use crate::repos::repository_manager::tests::MockRepositoryManager;
use crate::repos::repository_manager::{RepositoryManager, RepositoryManagerImpl};
use crate::services::isu_list_service::{IsuListService, IsuListServiceImpl};

pub trait ServiceManager<'r> {
    type IsuListSrv: IsuListService<'r>;

    fn isu_list_service(&self) -> Self::IsuListSrv;
}

pub struct ServiceManagerImpl<'r, R: RepositoryManager> {
    repo: &'r R,
}

impl<'r, R: RepositoryManager> ServiceManagerImpl<'r, R> {
    pub fn new(repo: &'r R) -> Self {
        Self { repo: repo }
    }
}

impl<'r, R: RepositoryManager> ServiceManager<'r> for ServiceManagerImpl<'r, R> {
    type IsuListSrv = IsuListServiceImpl<'r, R>;

    fn isu_list_service(&self) -> Self::IsuListSrv {
        Self::IsuListSrv::new(self.repo)
    }
}
