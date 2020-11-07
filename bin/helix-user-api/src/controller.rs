use crate::state::AppState;
use crate::APP_NAME;
use actix_web::web::Data;
use actix_web::{web, HttpRequest, HttpResponse};
use helix_user_domain::core::app_user::AppUser;
use helix_user_domain::core::person::Person;
use std::sync::{Arc, Mutex};

pub fn healthcheck(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body(format!("Everything's fine on {}.", APP_NAME))
}

pub fn unimplemented(_req: HttpRequest) -> HttpResponse {
    HttpResponse::NotFound().body("unimplemented !")
}

pub fn version(_req: HttpRequest) -> HttpResponse {
    let version = helix_config_lib::version::Version::new(
        env!("CARGO_PKG_VERSION").to_owned(),
        env!("GIT_SHORT_HASH").to_owned(),
        env!("GIT_MESSAGE").to_owned(),
        env!("GIT_COMMIT_DATE").to_owned(),
    );

    HttpResponse::Ok().json(version)
}

//-----------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginData {
    login: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken {
    access_token: String,
    refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RefreshToken {
    refresh_token: String,
}

pub fn login(state: web::Data<Mutex<AppState>>, login_data: web::Json<LoginData>) -> HttpResponse {
    let locked_state = &mut *state.lock().unwrap();
    let domain = locked_state.get_domain();

    match domain.login(&login_data.login, &login_data.password) {
        Some(app_user) => match generate_keys(
            &login_data.login,
            &app_user.uuid.unwrap(),
            &app_user.person.uuid.unwrap(),
        ) {
            Ok(gen_auth_key) => {
                let atoken: AccessToken = AccessToken {
                    access_token: gen_auth_key.0,
                    refresh_token: gen_auth_key.1,
                };

                HttpResponse::Ok()
                    .content_type("application/json")
                    .body(helix_struct::to_vec(&atoken))
            }
            Err(_) => HttpResponse::Unauthorized()
                .content_type("text/html; charset=utf-8")
                .body("{'message':'invalid credentials'}"),
        },
        None => HttpResponse::Unauthorized()
            .content_type("text/html; charset=utf-8")
            .body("{'message':'invalid credentials'}"),
    }
}

pub fn get_all_persons(state: web::Data<Mutex<AppState>>, _req: HttpRequest) -> HttpResponse {
    let locked_state = &mut *state.lock().unwrap();
    let domain = locked_state.get_domain();
    match domain.get_all_persons() {
        Err(_) => return HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(persons) => return HttpResponse::Ok().json(persons),
    }
}

pub fn get_person(state: web::Data<Mutex<AppState>>, req: HttpRequest) -> HttpResponse {
    let locked_state = &mut *state.lock().unwrap();
    let domain = locked_state.get_domain();

    let uuid: uuid::Uuid = uuid::Uuid::parse_str(req.match_info().get("uuid").unwrap()).unwrap();

    match domain.get_person(&uuid) {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(wrap_person) => match wrap_person {
            None => return HttpResponse::NotFound().body("Person not found."),
            Some(person) => return HttpResponse::Ok().json(person),
        },
    }
}

pub fn create_person(state: web::Data<Mutex<AppState>>, json: web::Json<Person>) -> HttpResponse {
    let locked_state = &mut *state.lock().unwrap();
    let domain = locked_state.get_domain();

    let person: Person = json.into_inner();
    match domain.create_person(person) {
        Err(_) => return HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(created_person) => return HttpResponse::Created().json(created_person),
    };
}

pub fn update_person(state: web::Data<Mutex<AppState>>, json: web::Json<Person>) -> HttpResponse {
    let locked_state = &mut *state.lock().unwrap();
    let domain = locked_state.get_domain();

    let person: Person = json.into_inner();
    match domain.update_person(person) {
        Err(_) => return HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(updated_person) => return HttpResponse::Created().json(updated_person),
    };
}

pub fn delete_person(state: web::Data<Mutex<AppState>>, json: web::Json<Person>) -> HttpResponse {
    let locked_state = &mut *state.lock().unwrap();
    let domain = locked_state.get_domain();

    let person: Person = json.into_inner();
    domain.delete_person(&person);

    match domain.delete_person(&person) {
        Err(_) => return HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(_) => return HttpResponse::NoContent().body("Person deleted."),
    };
}

pub fn get_all_users(state: web::Data<Mutex<AppState>>, _req: HttpRequest) -> HttpResponse {
    let locked_state = &mut *state.lock().unwrap();
    let domain = locked_state.get_domain();

    match domain.get_all_users() {
        Err(_) => return HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(users) => return HttpResponse::Ok().json(users),
    };
}

pub fn get_user(state: web::Data<Mutex<AppState>>, req: HttpRequest) -> HttpResponse {
    let locked_state = &mut *state.lock().unwrap();
    let domain = locked_state.get_domain();

    let uuid: uuid::Uuid = uuid::Uuid::parse_str(req.match_info().get("uuid").unwrap()).unwrap();
    match domain.get_user(Some(uuid)) {
        None => return HttpResponse::NotFound().body("User not found."),
        Some(user) => return HttpResponse::Ok().json(user),
    }
}

pub fn create_user(state: web::Data<Mutex<AppState>>, json: web::Json<AppUser>) -> HttpResponse {
    let locked_state = &mut *state.lock().unwrap();
    let domain = locked_state.get_domain();

    let user: AppUser = json.into_inner();
    match domain.create_user(user) {
        None => return HttpResponse::NotFound().body("User not created."),
        Some(created_user) => return HttpResponse::Created().json(created_user),
    }
}

pub fn update_user(state: web::Data<Mutex<AppState>>, json: web::Json<AppUser>) -> HttpResponse {
    let locked_state = &mut *state.lock().unwrap();
    let domain = locked_state.get_domain();

    let user: AppUser = json.into_inner();
    match domain.update_user(user) {
        None => return HttpResponse::NotFound().body("User not created."),
        Some(updated_user) => return HttpResponse::Ok().json(updated_user),
    }
}

pub fn delete_user(state: web::Data<Mutex<AppState>>, json: web::Json<AppUser>) -> HttpResponse {
    let locked_state = &mut *state.lock().unwrap();
    let domain = locked_state.get_domain();

    let user: AppUser = json.into_inner();
    domain.delete_user(&user);
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("User deleted.")
}
