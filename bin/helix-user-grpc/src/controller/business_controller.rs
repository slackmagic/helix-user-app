use crate::controller::user_service_server::UserService;
use crate::controller::{AuthRequest, AuthResponse};
use tonic::Response;

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
