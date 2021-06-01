use helix_user::user_service_server::{UserService, UserServiceServer};
use helix_user::{AuthRequest, AuthResponse};
use tonic::{transport::Server, Request, Response, Status};

mod helix_user {
    //include the wrapped generated lib
    tonic::include_proto!("helix_user_v1");
}

#[derive(Default)]
pub struct ImplUserService {}

#[tonic::async_trait]
impl UserService for ImplUserService {
    async fn authenticate(
        &self,
        request: tonic::Request<AuthRequest>,
    ) -> Result<tonic::Response<AuthResponse>, tonic::Status> {
        println!("Authenticate request : {:?}", request);

        Ok(Response::new(AuthResponse {
            token: "A_VALID_TOKEN".to_string(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let impl_user_service = ImplUserService::default();

    Server::builder()
        .add_service(UserServiceServer::new(impl_user_service))
        .serve(addr)
        .await?;

    Ok(())
}
