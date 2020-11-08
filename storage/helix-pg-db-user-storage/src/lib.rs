use helix_db_lib::postgres_connector::PostgresConnector;
use helix_user_domain::core::app_user::AppUser;
use helix_user_domain::core::person::Person;
use helix_user_domain::storage::error::*;
use helix_user_domain::storage::traits::StorageTrait;
use postgres::transaction::Transaction;
use postgres::{Connection, TlsMode};

pub struct PgDbUserStorage {
    pub db_conn: Connection,
}

impl PgDbUserStorage {
    pub fn new(conn_string: String) -> Self {
        let t_connection: Connection = Connection::connect(conn_string, TlsMode::None).unwrap();
        PgDbUserStorage {
            db_conn: t_connection,
        }
    }
}

impl StorageTrait for PgDbUserStorage {
    fn login(&self, key: String) -> StorageResult<Option<AppUser>> {
        let mut result: Option<AppUser> = None;

        let query: String = "select * from userstore.applicationuser where password=$1".to_string();

        for row in &self.db_conn.query(&query, &[&key]).unwrap() {
            match self.get_person_by_id(row.get("person_"))? {
                None => {
                    result = None;
                }
                Some(person) => {
                    result = Some(AppUser::new(
                        row.get("id"),
                        row.get("uuid"),
                        row.get("login"),
                        //Do not restitute password
                        "".to_string(),
                        row.get("photo"),
                        row.get("created_on"),
                        row.get("updated_on"),
                        row.get("lastlogin_on"),
                        person,
                    ));
                }
            }
        }

        //return
        Ok(result)
    }
    fn get_user(&self, uuid: Option<uuid::Uuid>) -> StorageResult<Option<AppUser>> {
        Err(StorageError::NotImplemented)
    }
    fn get_all_users(&self) -> StorageResult<Vec<AppUser>> {
        Err(StorageError::NotImplemented)
    }
    fn create_user(&self) -> StorageResult<Option<AppUser>> {
        Err(StorageError::NotImplemented)
    }
    fn update_user(&self) -> StorageResult<Option<AppUser>> {
        Err(StorageError::NotImplemented)
    }
    fn delete_user(&self) -> StorageResult<()> {
        Err(StorageError::NotImplemented)
    }

    fn create_person(&self, person: Person) -> StorageResult<Option<Person>> {
        Err(StorageError::NotImplemented)
    }
    fn update_person(&self, person: Person) -> StorageResult<Option<Person>> {
        Err(StorageError::NotImplemented)
    }
    fn delete_person(&self, person: &Person) -> StorageResult<()> {
        Err(StorageError::NotImplemented)
    }
    fn get_person_by_uuid(&self, uuid: &uuid::Uuid) -> StorageResult<Option<Person>> {
        Err(StorageError::NotImplemented)
    }
    fn get_person_by_id(&self, id: i32) -> StorageResult<Option<Person>> {
        Err(StorageError::NotImplemented)
    }
    fn get_all_person(&self) -> StorageResult<Vec<Person>> {
        Err(StorageError::NotImplemented)
    }
}
