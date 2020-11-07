use crate::core::app_user::*;
use crate::core::person::*;
use crate::storage::error::*;

pub trait StorageTrait: Sync + Send {
    fn login(&self, key: String) -> StorageResult<Option<AppUser>>;
    fn get_user(&self, uuid: Option<uuid::Uuid>) -> StorageResult<Option<AppUser>>;
    fn get_all_users(&self) -> StorageResult<Vec<AppUser>>;
    fn create_user(&self) -> StorageResult<Option<AppUser>>;
    fn update_user(&self) -> StorageResult<Option<AppUser>>;
    fn delete_user(&self) -> StorageResult<()>;

    fn create_person(person: Person) -> StorageResult<Option<Person>>;
    fn update_person(person: Person) -> StorageResult<Option<Person>>;
    fn delete_person(person: &Person) -> StorageResult<()>;
    fn get_person_by_uuid(uuid: &uuid::Uuid) -> StorageResult<Option<Person>>;
    fn get_person_by_id(id: i32) -> StorageResult<Option<Person>>;
    fn get_all_person() -> StorageResult<Vec<Person>>;
}
