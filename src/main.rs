mod cache;
mod fetchers;
mod handlers;

use cache::initialize_cache;
use tracing::{info, Level};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Initialize tracing subscriber with appropriate settings
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    // Initialize the cache and start periodic updates
    initialize_cache().await;
    info!("Cache initialized");

    // Launch the Rocket server
    rocket::build()
        .mount("/", handlers::routes())
        .launch()
        .await?;

    Ok(())
}
