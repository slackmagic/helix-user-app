use crate::controller::business_controller::ImplUserService;
use crate::controller::user_service_server::UserServiceServer;
use tonic::transport::Server;

pub mod controller;

const APP_NAME: &str = "USER_APP";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[HELIX gRPC {} {}]", APP_NAME, env!("CARGO_PKG_VERSION"));
    let addr = "127.0.0.1:42420".parse()?;
    let impl_user_service = ImplUserService::default();

    print!("--> Started on ");
    println!("http://{}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(impl_user_service))
        .serve(addr)
        .await?;

    Ok(())
}
