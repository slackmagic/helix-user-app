use crate::business::error::*;
use crate::business::traits::UserDomainTrait;
use crate::core::app_user::AppUser;
use crate::core::person::Person;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub struct UserDomain {}

impl UserDomain {
    pub fn new() -> Self {
        UserDomain {}
    }
}

impl UserDomainTrait for UserDomain {
    fn generate_user_auth_key(&self, login: &String, password: &String) -> String {
        let salt: &str = "__H3l!X__";

        //Hash construct
        let mut to_hash: String = String::new();
        to_hash.push_str(&login);
        to_hash.push_str(&password);
        to_hash.push_str(&salt.to_owned());

        let mut hasher = Sha256::new();
        hasher.input_str(&to_hash);

        //Key construct.
        let mut key: String = String::new();
        key.push_str(&login);
        key.push_str(&":".to_owned());
        key.push_str(&hasher.result_str());

        //return
        key
    }
    fn login(&self, login: &String, password: &String) -> UserDomainResult<AppUser> {
        Err(UserDomainError::NotImplemented)
    }

    fn get_all_users<'a>(&self) -> UserDomainResult<Vec<AppUser>> {
        Err(UserDomainError::NotImplemented)
    }
    fn get_user<'a>(&self, uuid: Option<uuid::Uuid>) -> UserDomainResult<Option<AppUser>> {
        Err(UserDomainError::NotImplemented)
    }
    fn create_user(&self, user: AppUser) -> UserDomainResult<AppUser> {
        Err(UserDomainError::NotImplemented)
    }
    fn update_user(&self, user: AppUser) -> UserDomainResult<AppUser> {
        Err(UserDomainError::NotImplemented)
    }
    fn delete_user(&self, user: &AppUser) -> UserDomainResult<()> {
        Err(UserDomainError::NotImplemented)
    }

    fn get_all_persons(&self) -> UserDomainResult<Vec<Person>> {
        Err(UserDomainError::NotImplemented)
    }
    fn get_person(&self, uuid: &uuid::Uuid) -> UserDomainResult<Option<Person>> {
        Err(UserDomainError::NotImplemented)
    }
    fn create_person(&self, person: Person) -> UserDomainResult<Person> {
        Err(UserDomainError::NotImplemented)
    }
    fn update_person<'a>(&self, person: Person) -> UserDomainResult<Person> {
        Err(UserDomainError::NotImplemented)
    }
    fn delete_person(&self, person: &Person) -> UserDomainResult<()> {
        Err(UserDomainError::NotImplemented)
    }
}
