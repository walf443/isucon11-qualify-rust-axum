pub mod isu_list_service;
pub mod service_manager;
pub mod trend_service;
use crate::repos::Result;

pub trait Service<T> {
    fn run(&self) -> Result<T>;
}
