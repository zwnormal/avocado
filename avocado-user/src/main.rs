use avocado_base::log::init_subscriber;
use avocado_user::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_subscriber();

    let address = "[::1]:50051".parse()?;
    run(address).await?.await?;
    Ok(())
}
