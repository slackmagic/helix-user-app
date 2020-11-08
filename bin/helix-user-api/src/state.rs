use crate::configuration::Configuration;
use helix_pg_db_user_storage::PgDbUserStorage;
use helix_user_domain::business::domain::UserDomain;
use helix_user_domain::business::traits::UserDomainTrait;
use std::boxed::Box;

pub struct AppState {
    user_domain: Box<dyn UserDomainTrait + Send>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            user_domain: Box::new(UserDomain::new(AppState::get_pg_storage())),
        }
    }

    pub fn get_domain(&self) -> &Box<dyn UserDomainTrait + Send> {
        &self.user_domain
    }

    fn get_pg_storage() -> Box<PgDbUserStorage> {
        Box::new(PgDbUserStorage::new(
            Configuration::get_database_connection_string(),
        ))
    }
}
