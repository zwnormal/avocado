use crate::app::{start_crm_server, start_user_server, TRACING};
use once_cell::sync::Lazy;
use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use serde::{Deserialize, Serialize};

mod app;

#[tokio::test]
async fn user_api_works() {
    Lazy::force(&TRACING);

    start_user_server().await;
    start_crm_server().await;

    // Login
    #[derive(Serialize)]
    struct LoginRequest {
        email: String,
        password: String,
    }

    #[derive(Deserialize, Debug)]
    struct LoginReply {
        session_id: String,
    }

    let client = Client::builder().build().unwrap();
    let reply = client
        .post("http://[::1]:3000/api/user/login")
        .json(&LoginRequest {
            email: "admin@avocado.com".to_string(),
            password: "kIxv4NomLT0WwGKF".to_string(),
        })
        .send()
        .await;
    let login_reply = reply.unwrap().json::<LoginReply>().await.unwrap();
    tracing::info!("user login reply: {:?}", login_reply);
    assert_eq!(login_reply.session_id.len(), 36);

    // List Users
    #[derive(Deserialize, Debug)]
    pub struct UserReply {
        pub id: String,
        pub email: String,
        pub first_name: String,
        pub last_name: String,
        pub role: String,
    }

    let reply = client
        .get("http://[::1]:3000/api/user/list")
        .header(AUTHORIZATION, format!("Bearer {}", login_reply.session_id))
        .send()
        .await;
    let list_reply = reply.unwrap().json::<Vec<UserReply>>().await.unwrap();
    tracing::info!("user list reply: {:?}", list_reply);
    assert_eq!(list_reply.get(0).unwrap().email, "admin@avocado.com");

    // Logout
    let reply = client
        .post("http://[::1]:3000/api/user/logout")
        .send()
        .await;
    assert!(reply.is_ok());
    let logout_reply = reply.unwrap();
    tracing::info!("user logout reply: {:?}", logout_reply);
}
