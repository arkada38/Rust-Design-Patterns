use super::{BusinessLaunch, CheapLaunch};

pub trait LaunchFactory<T> {
    fn create(&self) -> T;
}

pub struct CheapLaunchFactory;
pub struct BusinessLaunchFactory;

impl LaunchFactory<CheapLaunch> for CheapLaunchFactory {
    fn create(&self) -> CheapLaunch {
        CheapLaunch::new()
    }
}

impl LaunchFactory<BusinessLaunch> for BusinessLaunchFactory {
    fn create(&self) -> BusinessLaunch {
        BusinessLaunch::new()
    }
}
