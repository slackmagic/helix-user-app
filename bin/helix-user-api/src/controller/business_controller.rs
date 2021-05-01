use crate::state::AppState;
use actix_web::web::Data;
use actix_web::{web, HttpRequest, HttpResponse};
use helix_auth_lib::HelixAuth;
use helix_user_domain::core::app_user::AppUser;
use helix_user_domain::core::person::Person;
use std::sync::{Arc, Mutex};

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
pub struct RefreshToken {
    refresh_token: String,
}

pub async fn login(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    login_data: web::Json<LoginData>,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    match domain.login(&login_data.login, &login_data.password).await {
        Ok(app_user) => {
            let generated_keys = HelixAuth::generate_tokens(
                &login_data.login,
                &app_user.uuid.unwrap(),
                &app_user.person.uuid.unwrap(),
            );

            match generated_keys {
                Ok(generated_keys) => {
                    let atoken: AccessToken = AccessToken {
                        access_token: generated_keys.0,
                        refresh_token: generated_keys.1,
                    };

                    HttpResponse::Ok().json(atoken)
                }
                Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
            }
        }

        Err(_) => HttpResponse::Unauthorized().body("{'message':'invalid credentials'}"),
    }
}

pub async fn refresh(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    refresh_token: web::Json<RefreshToken>,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let generated_keys = HelixAuth::refresh_tokens(&refresh_token.refresh_token);

    match generated_keys {
        Ok(generated_keys) => {
            let atoken: AccessToken = AccessToken {
                access_token: generated_keys.0,
                refresh_token: generated_keys.1,
            };

            HttpResponse::Ok().json(atoken)
        }
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
    }
}

pub async fn get_all_persons(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    _req: HttpRequest,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();
    match domain.get_all_persons().await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(persons) => HttpResponse::Ok().json(persons),
    }
}

pub async fn get_person(wrap_state: Data<Arc<Mutex<AppState>>>, req: HttpRequest) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    let uuid: uuid::Uuid = uuid::Uuid::parse_str(req.match_info().get("uuid").unwrap()).unwrap();

    match domain.get_person(&uuid).await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(wrap_person) => match wrap_person {
            None => HttpResponse::NotFound().body("Person not found."),
            Some(person) => HttpResponse::Ok().json(person),
        },
    }
}

pub async fn create_person(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    json: web::Json<Person>,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    let person: Person = json.into_inner();
    match domain.create_person(person).await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(created_person) => HttpResponse::Created().json(created_person),
    }
}

pub async fn update_person(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    json: web::Json<Person>,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    let person: Person = json.into_inner();
    match domain.update_person(person).await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(updated_person) => HttpResponse::Created().json(updated_person),
    }
}

pub async fn delete_person(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    json: web::Json<Person>,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    let person: Person = json.into_inner();

    match domain.delete_person(person).await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(_) => HttpResponse::NoContent().body("Person deleted."),
    }
}

pub async fn get_all_users(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    _req: HttpRequest,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    match domain.get_all_users().await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(users) => HttpResponse::Ok().json(users),
    }
}

pub async fn get_user(wrap_state: Data<Arc<Mutex<AppState>>>, req: HttpRequest) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    //TODO: Control parse
    let uuid: uuid::Uuid = uuid::Uuid::parse_str(req.match_info().get("uuid").unwrap()).unwrap();

    match domain.get_user(&uuid).await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(wrap_user) => match wrap_user {
            None => HttpResponse::NotFound().body("User not found."),
            Some(user) => HttpResponse::Ok().json(user),
        },
    }
}

pub async fn create_user(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    json: web::Json<AppUser>,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    let user: AppUser = json.into_inner();
    match domain.create_user(user).await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(created_user) => HttpResponse::Created().json(created_user),
    }
}

pub async fn update_user(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    json: web::Json<AppUser>,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    let user: AppUser = json.into_inner();
    match domain.update_user(user).await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(updated_user) => HttpResponse::Created().json(updated_user),
    }
}

pub async fn delete_user(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    json: web::Json<AppUser>,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    let user: AppUser = json.into_inner();

    match domain.delete_user(user).await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(_) => HttpResponse::NoContent().body("User deleted."),
    }
}
