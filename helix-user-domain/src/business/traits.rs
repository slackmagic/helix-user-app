use crate::business::error::*;
use crate::core::app_user::AppUser;
use crate::core::person::Person;

pub trait UserDomainTrait: Send {
    fn generate_user_auth_key(&self, login: &String, password: &String) -> String;

    fn login(&self, login: &String, password: &String) -> UserDomainResult<AppUser>;

    fn get_all_users<'a>(&self) -> UserDomainResult<Vec<AppUser>>;
    fn get_user<'a>(&self, uuid: Option<uuid::Uuid>) -> UserDomainResult<Option<AppUser>>;
    fn create_user(&self, user: AppUser) -> UserDomainResult<AppUser>;
    fn update_user(&self, user: AppUser) -> UserDomainResult<AppUser>;
    fn delete_user(&self, user: &AppUser) -> UserDomainResult<()>;

    fn get_all_persons(&self) -> UserDomainResult<Vec<Person>>;
    fn get_person(&self, uuid: &uuid::Uuid) -> UserDomainResult<Option<Person>>;
    fn create_person(&self, person: Person) -> UserDomainResult<Person>;
    fn update_person<'a>(&self, person: Person) -> UserDomainResult<Person>;
    fn delete_person(&self, person: &Person) -> UserDomainResult<()>;
}
