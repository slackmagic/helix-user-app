use crate::business::error::*;
use crate::core::app_user::AppUser;
use crate::core::person::Person;
use async_trait::async_trait;

#[async_trait]
pub trait UserDomainTrait: Send + Sync {
    fn generate_user_auth_key(&self, login: &String, password: &String) -> String;

    async fn login(&self, login: &String, password: &String) -> UserDomainResult<AppUser>;

    async fn get_all_users<'a>(&self) -> UserDomainResult<Vec<AppUser>>;
    async fn get_user<'a>(&self, uuid: &uuid::Uuid) -> UserDomainResult<Option<AppUser>>;
    async fn create_user(&self, user: AppUser) -> UserDomainResult<AppUser>;
    async fn update_user(&self, user: AppUser) -> UserDomainResult<AppUser>;
    async fn delete_user(&self, user: AppUser) -> UserDomainResult<()>;

    async fn get_all_persons(&self) -> UserDomainResult<Vec<Person>>;
    async fn get_person(&self, uuid: &uuid::Uuid) -> UserDomainResult<Option<Person>>;
    async fn create_person(&self, person: Person) -> UserDomainResult<Person>;
    async fn update_person<'a>(&self, person: Person) -> UserDomainResult<Person>;
    async fn delete_person(&self, person: Person) -> UserDomainResult<()>;
}
