use crate::core::app_user::*;
use crate::core::person::*;
use crate::storage::error::*;
use async_trait::async_trait;

#[async_trait]
pub trait StorageTrait: Send + Sync {
    async fn login(&self, key: String) -> StorageResult<Option<AppUser>>;
    async fn get_user(&self, uuid: &uuid::Uuid) -> StorageResult<Option<AppUser>>;
    async fn get_all_users(&self) -> StorageResult<Vec<AppUser>>;
    async fn create_user(&self, user: AppUser) -> StorageResult<AppUser>;
    async fn update_user(&self, user: AppUser) -> StorageResult<AppUser>;
    async fn delete_user(&self, user: AppUser) -> StorageResult<()>;

    async fn create_person(&self, person: Person) -> StorageResult<Person>;
    async fn update_person(&self, person: Person) -> StorageResult<Person>;
    async fn delete_person(&self, person: Person) -> StorageResult<()>;
    async fn get_person_by_uuid(&self, uuid: &uuid::Uuid) -> StorageResult<Option<Person>>;
    async fn get_person_by_id(&self, id: i32) -> StorageResult<Option<Person>>;
    async fn get_all_person(&self) -> StorageResult<Vec<Person>>;
}
