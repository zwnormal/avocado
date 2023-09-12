use avocado_base::log::{get_subscriber, init_subscriber};
use avocado_crm::run;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let subscriber = get_subscriber(
        "avocado-crm".to_string(),
        "info".to_string(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    run(addr).await.await
}
