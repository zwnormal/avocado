use crate::app::start_server;
use avocado_proto::grpc::user::user_client::UserClient;
use avocado_proto::grpc::user::{LoginRequest, WhoAmIRequest};
use tonic::metadata::MetadataValue;

mod app;

#[tokio::test]
async fn user_grpc_works() {
    start_server().await;

    let mut user_client = UserClient::connect("http://[::1]:50051")
        .await
        .expect("failed to connect to user grpc server");

    // Login as admin
    let request = tonic::Request::new(LoginRequest {
        email: "admin@avocado.com".to_string(),
        password: "kIxv4NomLT0WwGKF".to_string(),
    });
    let response = user_client
        .login(request)
        .await
        .expect("user login grpc call failed");
    let jwt_token = response.into_inner().access_token;

    // Get who I am
    let access_token: MetadataValue<_> = jwt_token.parse().expect("cannot insert grpc auth header");
    let mut request = tonic::Request::new(WhoAmIRequest {});
    request.metadata_mut().insert("auth", access_token.clone());
    let response = user_client
        .who_am_i(request)
        .await
        .expect("cannot get who I am");
    let email = response.into_inner().email;
    assert_eq!(email, "admin@avocado.com");
}
