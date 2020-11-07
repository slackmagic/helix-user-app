use helix_user_domain::business::domain::UserDomain;
use helix_user_domain::business::traits::UserDomainTrait;
use std::boxed::Box;

pub struct AppState {
    user_domain: Box<dyn UserDomainTrait + Send>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            user_domain: Box::new(UserDomain::new()),
        }
    }

    pub fn get_domain(&self) -> &Box<dyn UserDomainTrait + Send> {
        &self.user_domain
    }
}
