use crate::configuration::Configuration;
use helix_user_domain::business::domain::UserDomain;
use helix_user_domain::business::traits::UserDomainTrait;
use pg_db_storage::PgDbUserStorage;
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
        Box::new(
            PgDbUserStorage::new(
                Configuration::get_database_name(),
                Configuration::get_database_host(),
                Configuration::get_database_port(),
                Configuration::get_database_user(),
                Configuration::get_database_password(),
            )
            .unwrap(),
        )
    }
}
