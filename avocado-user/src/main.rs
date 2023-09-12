use avocado_base::log::{get_subscriber, init_subscriber};
use avocado_user::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = get_subscriber(
        "avocado-user".to_string(),
        "info".to_string(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let address = "[::1]:50051".parse()?;
    run(address).await?.await?;
    Ok(())
}
