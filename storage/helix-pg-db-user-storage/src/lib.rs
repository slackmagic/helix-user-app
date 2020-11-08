use chrono::prelude::*;
use helix_user_domain::core::app_user::AppUser;
use helix_user_domain::core::person::Person;
use helix_user_domain::storage::error::*;
use helix_user_domain::storage::traits::StorageTrait;
use postgres::{Connection, TlsMode};
use uuid;

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

        for row in &self.db_conn.query(&query, &[&key])? {
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

    fn get_user(&self, uuid: &uuid::Uuid) -> StorageResult<Option<AppUser>> {
        let mut result: Option<AppUser> = None;

        let query: String = "
        select *
        from userstore.applicationuser
        where 1=1
        and uuid=$1
        "
        .to_string();

        for row in &self.db_conn.query(&query, &[&uuid])? {
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

        Ok(result)
    }
    fn get_all_users(&self) -> StorageResult<Vec<AppUser>> {
        let mut result: Vec<AppUser> = Vec::new();

        let query: String = "
        select *
        from userstore.applicationuser
        where 1=1 and person_ is not null
        order by login;
        "
        .to_string();

        for row in &self.db_conn.query(&query, &[])? {
            match self.get_person_by_id(row.get("person_"))? {
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

    fn create_user(&self, mut user: AppUser) -> StorageResult<AppUser> {
        user.created_on = Some(Utc::now());

        let query: String = "
        INSERT INTO userstore.APPLICATIONUSER
        VALUES (DEFAULT,DEFAULT,$1,$2,$3,$4,NULL,NULL,$5) 
        RETURNING id, uuid;"
            .to_string();

        let stmt = &self.db_conn.prepare(&query)?;
        let row_inserted = stmt.query(&[
            &user.login,
            &user.password,
            &user.photo,
            &user.created_on,
            &user.person.id,
        ])?;

        let row_data = row_inserted.iter().next().unwrap();
        user.id = row_data.get("id");
        user.uuid = row_data.get("uuid");

        Ok(user)
    }
    fn update_user(&self, mut user: AppUser) -> StorageResult<AppUser> {
        user.updated_on = Some(Utc::now());

        let query: String = "
        UPDATE userstore.APPLICATIONUSER SET (uuid, login, password, photo, created_on, updated_on, lastlogin_on, person_) 
        = ($2,$3,$4,$5,$6,$7,$8,$9)
        WHERE ID = $1;"
        .to_string();

        let stmt = &self.db_conn.prepare(&query)?;
        stmt.query(&[
            &user.id,
            &user.uuid,
            &user.login,
            &user.password,
            &user.photo,
            &user.created_on,
            &user.updated_on,
            &user.last_login_on,
            &user.person.id,
        ])?;

        Ok(user)
    }
    fn delete_user(&self, user: AppUser) -> StorageResult<()> {
        let query: String = "
        DELETE FROM userstore.APPLICATIONUSER WHERE ID = $1;"
            .to_string();

        &self.db_conn.execute(&query, &[&user.id])?;
        Ok(())
    }

    fn create_person(&self, mut person: Person) -> StorageResult<Person> {
        person.created_on = Some(Utc::now());
        let query: String = "
        INSERT INTO userstore.PERSON
        VALUES (DEFAULT,DEFAULT,$1,$2,$3,$4,$5,NULL) 
        RETURNING id, uuid;"
            .to_string();

        let stmt = &self.db_conn.prepare(&query)?;
        let row_inserted = stmt.query(&[
            &person.firstname,
            &person.lastname,
            &person.email,
            &person.phone,
            &person.created_on,
        ])?;

        let row_data = row_inserted.iter().next().unwrap();
        person.id = row_data.get("id");
        person.uuid = row_data.get("uuid");

        Ok(person)
    }

    fn update_person(&self, mut person: Person) -> StorageResult<Person> {
        person.updated_on = Some(Utc::now());
        let query: String = "
        UPDATE userstore.PERSON SET (firstname, lastname, email, phone, updated_on) 
        = ($2,$3,$4,$5,$6)
        WHERE ID = $1;"
            .to_string();

        let stmt = &self.db_conn.prepare(&query)?;
        stmt.query(&[
            &person.id,
            &person.firstname,
            &person.lastname,
            &person.email,
            &person.phone,
            &person.updated_on,
        ])?;

        Ok(person)
    }

    fn delete_person(&self, person: Person) -> StorageResult<()> {
        let query: String = "
        DELETE FROM userstore.PERSON WHERE ID = $1;"
            .to_string();

        &self.db_conn.execute(&query, &[&person.id])?;
        Ok(())
    }

    fn get_person_by_uuid(&self, uuid: &uuid::Uuid) -> StorageResult<Option<Person>> {
        let mut result: Option<Person> = None;
        let query: String = "
        select *
        from userstore.person as pe
        where 1=1
        and uuid=$1
        "
        .to_string();

        for row in &self.db_conn.query(&query, &[&uuid])? {
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

    fn get_person_by_id(&self, id: i32) -> StorageResult<Option<Person>> {
        let mut result: Option<Person> = None;
        let query: String = "
        select *
        from userstore.person as pe
        where 1=1
        and id=$1
        "
        .to_string();

        for row in &self.db_conn.query(&query, &[&id])? {
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

    fn get_all_person(&self) -> StorageResult<Vec<Person>> {
        let mut result: Vec<Person> = Vec::new();

        let query: String = "
        select *
        from userstore.person as pe
        where 1=1
        order by firstname;
        "
        .to_string();

        for row in &self.db_conn.query(&query, &[])? {
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
