mod api;
mod cache;
mod integrations;

use cache::initialize_cache;
use tracing::info;
use tracing_subscriber::{FmtSubscriber, EnvFilter};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Initialize tracing subscriber with appropriate settings
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info")) // Set default log level to info
        .unwrap();

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(filter)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set subscriber");

    // Initialize the cache and start periodic updates
    initialize_cache().await;
    info!("Cache initialized");

    // Launch the Rocket server
    rocket::build()
        .mount("/", api::routes())
        .launch()
        .await?;

    Ok(())
}
