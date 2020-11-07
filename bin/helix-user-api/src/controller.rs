use crate::state::AppState;
use crate::APP_NAME;
use actix_web::web::Data;
use actix_web::{web, HttpRequest, HttpResponse};
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
