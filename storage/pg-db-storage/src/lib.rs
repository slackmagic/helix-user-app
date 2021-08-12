use async_trait::async_trait;
use chrono::prelude::*;
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use helix_user_domain::core::app_user::AppUser;
use helix_user_domain::core::person::Person;
use helix_user_domain::storage::error::*;
use helix_user_domain::storage::traits::StorageTrait;
use tokio_postgres::tls::NoTls;

use uuid;

pub struct PgDbUserStorage {
    pub pool: Pool,
}

impl PgDbUserStorage {
    pub fn new(
        database: String,
        host: String,
        port: u16,
        user: String,
        password: String,
    ) -> StorageResult<PgDbUserStorage> {
        let mut cfg = Config::new();
        cfg.dbname = Some(database);
        cfg.host = Some(host);
        cfg.port = Some(port);
        cfg.user = Some(user);
        cfg.password = Some(password);
        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });

        Ok(PgDbUserStorage {
            pool: cfg.create_pool(NoTls).unwrap(),
        })
    }
}

#[async_trait]
impl StorageTrait for PgDbUserStorage {
    async fn login(&self, key: String) -> StorageResult<Option<AppUser>> {
        let mut result: Option<AppUser> = None;
        let query = "select * from userstore.applicationuser where password=$1;";

        let client = &self.pool.get().await.unwrap();
        client.query(query, &[&key]).await?;

        for row in client.query(query, &[&key]).await? {
            match self.get_person_by_id(row.get("person_")).await? {
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

    async fn get_user(&self, uuid: &uuid::Uuid) -> StorageResult<Option<AppUser>> {
        let mut result: Option<AppUser> = None;

        let query = "
        select *
        from userstore.applicationuser
        where 1=1
        and uuid=$1;";

        let client = &self.pool.get().await.unwrap();
        for row in client.query(query, &[&uuid]).await? {
            match self.get_person_by_id(row.get("person_")).await? {
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

        Ok(result)
    }
    async fn get_all_users(&self) -> StorageResult<Vec<AppUser>> {
        let mut result: Vec<AppUser> = Vec::new();

        let query = "
        select *
        from userstore.applicationuser
        where 1=1 and person_ is not null
        order by login;";

        let client = &self.pool.get().await.unwrap();
        for row in client.query(query, &[]).await? {
            match self.get_person_by_id(row.get("person_")).await? {
                None => {}
                Some(person) => {
                    result.push(AppUser::new(
                        row.get("id"),
                        row.get("uuid"),
                        row.get("login"),
                        row.get("password"),
                        row.get("photo"),
                        row.get("created_on"),
                        row.get("updated_on"),
                        row.get("lastlogin_on"),
                        person,
                    ));
                }
            }
        }

        Ok(result)
    }

    async fn create_user(&self, mut user: AppUser) -> StorageResult<AppUser> {
        user.created_on = Some(Utc::now());

        let query = "
        INSERT INTO userstore.APPLICATIONUSER
        VALUES (DEFAULT,DEFAULT,$1,$2,$3,$4,NULL,NULL,$5) 
        RETURNING id, uuid;";

        let client = &self.pool.get().await.unwrap();
        let row_inserted = client
            .query(
                query,
                &[
                    &user.login,
                    &user.password,
                    &user.photo,
                    &user.created_on,
                    &user.person.id,
                ],
            )
            .await?;

        let row_data = row_inserted.iter().next().unwrap();
        user.id = row_data.get("id");
        user.uuid = row_data.get("uuid");

        Ok(user)
    }
    async fn update_user(&self, mut user: AppUser) -> StorageResult<AppUser> {
        user.updated_on = Some(Utc::now());

        let query = "UPDATE userstore.APPLICATIONUSER SET (uuid, login, password, photo, created_on, updated_on, lastlogin_on, person_) 
        = ($2,$3,$4,$5,$6,$7,$8,$9)
        WHERE ID = $1;";

        let client = &self.pool.get().await.unwrap();

        client
            .execute(
                query,
                &[
                    &user.id,
                    &user.uuid,
                    &user.login,
                    &user.password,
                    &user.photo,
                    &user.created_on,
                    &user.updated_on,
                    &user.last_login_on,
                    &user.person.id,
                ],
            )
            .await?;

        Ok(user)
    }
    async fn delete_user(&self, user: AppUser) -> StorageResult<()> {
        let query = "
        DELETE FROM userstore.APPLICATIONUSER WHERE ID = $1;";

        let client = &self.pool.get().await.unwrap();
        client.execute(query, &[&user.id]).await?;
        Ok(())
    }

    async fn create_person(&self, mut person: Person) -> StorageResult<Person> {
        person.created_on = Some(Utc::now());
        let query = "
        INSERT INTO userstore.PERSON
        VALUES (DEFAULT,DEFAULT,$1,$2,$3,$4,$5,NULL) 
        RETURNING id, uuid;";

        let client = &self.pool.get().await.unwrap();
        let row_inserted = client
            .query(
                query,
                &[
                    &person.firstname,
                    &person.lastname,
                    &person.email,
                    &person.phone,
                    &person.created_on,
                ],
            )
            .await?;

        let row_data = row_inserted.iter().next().unwrap();
        person.id = row_data.get("id");
        person.uuid = row_data.get("uuid");

        Ok(person)
    }

    async fn update_person(&self, mut person: Person) -> StorageResult<Person> {
        person.updated_on = Some(Utc::now());
        let query = "
        UPDATE userstore.PERSON SET (firstname, lastname, email, phone, updated_on) 
        = ($2,$3,$4,$5,$6)
        WHERE ID = $1;";

        let client = &self.pool.get().await.unwrap();
        client
            .execute(
                query,
                &[
                    &person.id,
                    &person.firstname,
                    &person.lastname,
                    &person.email,
                    &person.phone,
                    &person.updated_on,
                ],
            )
            .await?;

        Ok(person)
    }

    async fn delete_person(&self, person: Person) -> StorageResult<()> {
        let query = "DELETE FROM userstore.PERSON WHERE ID = $1;";

        let client = &self.pool.get().await.unwrap();
        client.execute(query, &[&person.id]).await?;
        Ok(())
    }

    async fn get_person_by_uuid(&self, uuid: &uuid::Uuid) -> StorageResult<Option<Person>> {
        let mut result: Option<Person> = None;
        let query = "
        select *
        from userstore.person as pe
        where 1=1
        and uuid=$1;";

        let client = &self.pool.get().await.unwrap();
        for row in client.query(query, &[&uuid]).await? {
            result = Some(Person::new(
                row.get("id"),
                row.get("uuid"),
                row.get("firstname"),
                row.get("lastname"),
                row.get("email"),
                row.get("phone"),
                row.get("created_on"),
                row.get("updated_on"),
            ));
        }

        Ok(result)
    }

    async fn get_person_by_id(&self, id: i32) -> StorageResult<Option<Person>> {
        let mut result: Option<Person> = None;
        let query = "
        select *
        from userstore.person as pe
        where 1=1
        and id=$1;
        ";

        let client = &self.pool.get().await.unwrap();
        for row in client.query(query, &[&id]).await? {
            result = Some(Person::new(
                row.get("id"),
                row.get("uuid"),
                row.get("firstname"),
                row.get("lastname"),
                row.get("email"),
                row.get("phone"),
                row.get("created_on"),
                row.get("updated_on"),
            ));
        }

        Ok(result)
    }

    async fn get_all_person(&self) -> StorageResult<Vec<Person>> {
        let mut result: Vec<Person> = Vec::new();

        let query = "
        select *
        from userstore.person as pe
        where 1=1
        order by firstname;
        ";

        let client = &self.pool.get().await.unwrap();
        for row in client.query(query, &[]).await? {
            let person: Person = Person::new(
                row.get("id"),
                row.get("uuid"),
                row.get("firstname"),
                row.get("lastname"),
                row.get("email"),
                row.get("phone"),
                row.get("created_on"),
                row.get("updated_on"),
            );

            result.push(person);
        }

        Ok(result)
    }
}
