use crate::repos::repository_manager::RepositoryManager;
use crate::services::isu_list_service::IsuListService;
use std::sync::Arc;

pub trait ServiceManager: Clone + std::marker::Send + std::marker::Sync {
    type Repo: RepositoryManager;

    fn isu_list_service(&self) -> IsuListService<Self::Repo>;
}

#[derive(Clone)]
pub struct ServiceManagerImpl<R: RepositoryManager> {
    repo: Arc<R>,
}

impl<R: RepositoryManager> ServiceManagerImpl<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }
}

impl<R: RepositoryManager> ServiceManager for ServiceManagerImpl<R> {
    type Repo = R;

    fn isu_list_service(&self) -> IsuListService<Self::Repo> {
        IsuListService::new(self.repo.clone())
    }
}
