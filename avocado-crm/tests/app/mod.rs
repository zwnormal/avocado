use avocado_base::log::{get_subscriber, init_subscriber};
use avocado_crm::run;
use avocado_proto::grpc::user::user_client::UserClient;
use avocado_user::run as run_user_server;
use once_cell::sync::Lazy;
use reqwest::Client;
use std::time::Duration;
use tokio::time::{sleep, timeout};

pub static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "avocado-crm-tests".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub async fn start_user_server() {
    let address = "[::1]:50051"
        .parse()
        .expect("invalid user server socket address");
    let server = run_user_server(address)
        .await
        .expect("failed to create avocado-user grpc router");
    tokio::spawn(server);

    async fn wait_server() {
        loop {
            if UserClient::connect("http://[::1]:50051").await.is_ok() {
                break;
            }
            sleep(Duration::from_millis(300)).await;
        }
    }
    timeout(Duration::from_secs(10), wait_server())
        .await
        .expect("timeout of 10s while waiting for user server starts");
}

pub async fn start_crm_server() {
    let address = "[::1]:3000"
        .parse()
        .expect("invalid crm server socket address");
    let server = run(address).await;
    tokio::spawn(server);

    async fn wait_server() {
        let client = Client::new();
        loop {
            if client
                .get("http://[::1]:3000/health-check")
                .send()
                .await
                .is_ok()
            {
                break;
            }
            sleep(Duration::from_millis(300)).await;
        }
    }
    timeout(Duration::from_secs(10), wait_server())
        .await
        .expect("timeout of 10s while waiting for crm server starts");
}
