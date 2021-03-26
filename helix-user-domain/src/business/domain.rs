use crate::business::error::*;
use crate::business::traits::UserDomainTrait;
use crate::core::app_user::AppUser;
use crate::core::person::Person;
use crate::storage::traits::StorageTrait;
use async_trait::async_trait;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::boxed::Box;

pub struct UserDomain {
    storage: Box<dyn StorageTrait>,
}

impl UserDomain {
    pub fn new(storage: Box<dyn StorageTrait>) -> Self {
        UserDomain { storage: storage }
    }
}

#[async_trait]
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
    async fn login(&self, login: &String, password: &String) -> UserDomainResult<AppUser> {
        match self
            .storage
            .login(self.generate_user_auth_key(login, password))
            .await
        {
            Ok(wrap_user) => match wrap_user {
                Some(user) => Ok(user),
                //User not found
                None => {
                    println!("User not found");
                    Err(UserDomainError::NotImplemented)
                }
            },
            //Error with backend
            Err(_) => {
                println!("Err with backend");
                Err(UserDomainError::NotImplemented)
            }
        }
    }

    async fn get_all_users<'a>(&self) -> UserDomainResult<Vec<AppUser>> {
        Ok(self.storage.get_all_users().await?)
    }

    async fn get_user<'a>(&self, uuid: &uuid::Uuid) -> UserDomainResult<Option<AppUser>> {
        Ok(self.storage.get_user(uuid).await?)
    }
    async fn create_user(&self, user: AppUser) -> UserDomainResult<AppUser> {
        Ok(self.storage.create_user(user).await?)
    }
    async fn update_user(&self, user: AppUser) -> UserDomainResult<AppUser> {
        self.storage.update_person(user.person.clone()).await?;
        Ok(self.storage.update_user(user).await?)
    }
    async fn delete_user(&self, user: AppUser) -> UserDomainResult<()> {
        Ok(self.storage.delete_user(user).await?)
    }
    async fn get_all_persons(&self) -> UserDomainResult<Vec<Person>> {
        Ok(self.storage.get_all_person().await?)
    }
    async fn get_person(&self, uuid: &uuid::Uuid) -> UserDomainResult<Option<Person>> {
        Ok(self.storage.get_person_by_uuid(uuid).await?)
    }
    async fn create_person(&self, person: Person) -> UserDomainResult<Person> {
        Ok(self.storage.create_person(person).await?)
    }
    async fn update_person<'a>(&self, person: Person) -> UserDomainResult<Person> {
        Ok(self.storage.update_person(person).await?)
    }
    async fn delete_person(&self, person: Person) -> UserDomainResult<()> {
        Ok(self.storage.delete_person(person).await?)
    }
}
