use chrono::prelude::*;
use uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub uuid: Option<uuid::Uuid>,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone: Option<String>,
    pub created_on: Option<DateTime<Utc>>,
    pub updated_on: Option<DateTime<Utc>>,
}

impl Person {
    pub fn new(
        id: i32,
        uuid: Option<uuid::Uuid>,
        firstname: String,
        lastname: String,
        email: String,
        phone: Option<String>,
        created_on: Option<DateTime<Utc>>,
        updated_on: Option<DateTime<Utc>>,
    ) -> Person {
        Person {
            id: id,
            uuid: uuid,
            firstname: firstname,
            lastname: lastname,
            email: email,
            phone: phone,
            created_on: created_on,
            updated_on: updated_on,
        }
    }
}
