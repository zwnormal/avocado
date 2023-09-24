use avocado_base::log::init_subscriber;
use avocado_proto::grpc::user::user_client::UserClient;
use avocado_user::run;
use once_cell::sync::Lazy;
use sqlx::__rt::timeout;
use std::time::Duration;
use tokio::time::sleep;

static TRACING: Lazy<()> = Lazy::new(|| {
    if std::env::var("TEST_LOG").is_ok() {
        init_subscriber();
    }
});

pub async fn start_server() {
    Lazy::force(&TRACING);

    let address = "[::1]:50051".parse().expect("invalid socket address");
    let server = run(address)
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
        .expect("timeout of 10s while waiting for server starts");
}
