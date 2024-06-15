mod cache;
mod fetchers;
mod handlers;

use cache::initialize_cache;
use rocket::Config;
use std::env;
use tracing::{info, Level};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Initialize tracing subscriber with appropriate settings
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set subscriber");

    // Initialize the cache and start periodic updates
    initialize_cache().await;
    info!("Cache initialized");

    // Get the port from the environment variable, default to 8000 if not set
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());

    // Configure Rocket to bind to 0.0.0.0:port
    let config = Config {
        address: "0.0.0.0".parse().unwrap(),
        port: port.parse().unwrap(),
        ..Config::default()
    };

    // Launch the Rocket server with the configured settings
    rocket::custom(config)
        .mount("/", handlers::routes())
        .launch()
        .await?;

    Ok(())
}
