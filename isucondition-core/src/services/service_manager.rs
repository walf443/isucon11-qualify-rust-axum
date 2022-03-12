use crate::repos::repository_manager::RepositoryManagerImpl;
use crate::services::isu_list_service::{IsuListService, IsuListServiceImpl};

pub trait ServiceManager {
    type IsuListSrv: IsuListService;

    fn isu_list_service() -> Self::IsuListSrv;
}

pub struct ServiceManagerImpl<'r> {
    repo: &'r RepositoryManagerImpl,
}

impl<'r> ServiceManager for ServiceManagerImpl<'r> {
    type IsuListSrv = IsuListServiceImpl<'r, RepositoryManagerImpl>;

    fn isu_list_service() -> Self::IsuListSrv {
        todo!()
    }
}
