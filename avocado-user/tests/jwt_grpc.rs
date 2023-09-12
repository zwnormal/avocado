use crate::app::start_server;
use avocado_proto::grpc::jwt::jwt_client::JwtClient;
use avocado_proto::grpc::jwt::{RefreshRequest, VerifyRequest};
use avocado_proto::grpc::user::user_client::UserClient;
use avocado_proto::grpc::user::{LoginRequest, WhoAmIRequest};
use std::time::Duration;
use tokio::time::sleep;
use tonic::metadata::MetadataValue;

mod app;

#[tokio::test]
async fn jwt_grpc_works() {
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
        .expect("user login grpc call failed")
        .into_inner();
    let access_token = response.access_token.clone();
    let refresh_token = response.refresh_token.clone();

    // Wait for one second so the new access token won't be exactly same as the old one
    sleep(Duration::from_secs(1)).await;

    // Refresh to get a new token
    let mut jwt_client = JwtClient::connect("http://[::1]:50051")
        .await
        .expect("failed to connect to jwt grpc server");
    let access_token_header: MetadataValue<_> = access_token
        .parse()
        .expect("cannot insert grpc auth header");
    let mut request = tonic::Request::new(RefreshRequest {
        refresh_token: refresh_token.clone(),
    });
    request
        .metadata_mut()
        .insert("auth", access_token_header.clone());
    let response = jwt_client
        .refresh(request)
        .await
        .expect("jwt token refresh grpc call failed");
    let token_reply = response.into_inner();
    let access_token_1 = token_reply.access_token;
    let refresh_token_1 = token_reply.refresh_token;
    assert_ne!(access_token, access_token_1);
    assert_ne!(refresh_token, refresh_token_1.clone());

    // Ensure we can call who I am with the new token
    let access_token_header: MetadataValue<_> = access_token_1
        .parse()
        .expect("cannot insert grpc auth header");
    let mut request = tonic::Request::new(WhoAmIRequest {});
    request
        .metadata_mut()
        .insert("auth", access_token_header.clone());
    let response = user_client
        .who_am_i(request)
        .await
        .expect("cannot get who I am");
    let email = response.into_inner().email;
    assert_eq!(email, "admin@avocado.com");

    // Make sure the new refresh token can get a new access token and refresh token
    sleep(Duration::from_secs(1)).await;
    let access_token_header: MetadataValue<_> = access_token_1
        .parse()
        .expect("cannot insert grpc auth header");
    let request = tonic::Request::new(VerifyRequest {
        token: access_token_1.clone(),
    });
    let response = jwt_client
        .verify(request)
        .await
        .expect("jwt token refresh grpc call failed");
    assert!(!response.into_inner().sub.is_empty());
    let mut request = tonic::Request::new(RefreshRequest {
        refresh_token: refresh_token_1.clone(),
    });
    request
        .metadata_mut()
        .insert("auth", access_token_header.clone());
    let response = jwt_client
        .refresh(request)
        .await
        .expect("jwt token refresh grpc call failed");
    let token_reply = response.into_inner();
    let access_token_2 = token_reply.access_token;
    let refresh_token_2 = token_reply.refresh_token;
    assert_ne!(access_token_1, access_token_2);
    assert_ne!(refresh_token_1, refresh_token_2);
}
