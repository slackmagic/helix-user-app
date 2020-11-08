use std::env;

pub struct Configuration {}

impl Configuration {
    pub fn get_database_connection_string() -> String {
        env::var("HELIX_DB_CONNECTION_STRING").expect("DB_CONNECTION_STRING not found.")
    }
}
