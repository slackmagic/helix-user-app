use crate::core::person::Person;
use chrono::prelude::*;
use uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppUser {
    pub id: i32,
    pub uuid: Option<uuid::Uuid>,
    pub login: String,
    pub password: String,
    pub photo: Option<Vec<u8>>,
    pub created_on: Option<DateTime<Utc>>,
    pub updated_on: Option<DateTime<Utc>>,
    pub last_login_on: Option<DateTime<Utc>>,
    pub person: Person,
}

impl AppUser {
    pub fn new(
        id: i32,
        uuid: Option<uuid::Uuid>,
        login: String,
        password: String,
        photo: Option<Vec<u8>>,
        created_on: Option<DateTime<Utc>>,
        updated_on: Option<DateTime<Utc>>,
        last_login_date: Option<DateTime<Utc>>,
        person: Person,
    ) -> AppUser {
        AppUser {
            id: id,
            uuid: uuid,
            login: login,
            password: password,
            photo: photo,
            created_on: created_on,
            updated_on: updated_on,
            last_login_on: last_login_date,
            person: person,
        }
    }
}
