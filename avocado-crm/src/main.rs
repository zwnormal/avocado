use avocado_base::log::init_subscriber;
use avocado_crm::run;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    init_subscriber();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    run(addr).await.await
}
