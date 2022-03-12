use crate::repos::repository_manager::tests::MockRepositoryManager;
use crate::repos::repository_manager::{RepositoryManager, RepositoryManagerImpl};
use crate::services::isu_list_service::{IsuListService};

pub trait ServiceManager {
    type Repo: RepositoryManager;

    fn isu_list_service(&self) -> IsuListService<Self::Repo>;
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
    type Repo = RepositoryManagerImpl;

    fn isu_list_service(&self) -> IsuListService<Self::Repo> {
        todo!()
    }
}
