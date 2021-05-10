use crate::configuration::Configuration;
use crate::APP_NAME;
use actix_files::NamedFile;
use actix_web::{HttpRequest, HttpResponse, Result};
use helix_config_lib::version::Version;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct HealthCheckResponse {
    app_name: String,
    message: String,
    version: Version,
}

pub async fn serve_static_file(req: HttpRequest) -> Result<NamedFile> {
    let filename: &str = req.match_info().query("filename");
    let base_path = Configuration::get_static_folder();

    let filename = match filename.contains(".") {
        true => filename,
        false => "index.html",
    };

    let serve_file_path = format!("{}{}", base_path, filename);
    let path: PathBuf = PathBuf::from(serve_file_path);
    Ok(NamedFile::open(path)?)
}

pub fn healthcheck(_req: HttpRequest) -> HttpResponse {
    let message = HealthCheckResponse {
        app_name: APP_NAME.to_string(),
        message: "Everything's fine !".to_string(),
        version: helix_config_lib::version::Version::new(
            env!("CARGO_PKG_VERSION").to_owned(),
            env!("VERSION_NAME").to_owned(),
            env!("GIT_SHORT_HASH").to_owned(),
            env!("GIT_MESSAGE").to_owned(),
            env!("GIT_COMMIT_DATE").to_owned(),
        ),
    };

    HttpResponse::Ok().json(message)
}

pub fn unimplemented(_req: HttpRequest) -> HttpResponse {
    HttpResponse::NotFound().body("unimplemented !")
}
