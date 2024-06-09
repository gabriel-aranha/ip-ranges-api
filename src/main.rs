mod api;
mod cache;
mod integrations;
use cache::initialize_cache;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Initialize the cache and start periodic updates
    initialize_cache().await;
    println!("cache initialized");

    // Launch the Rocket server
    rocket::build()
        .mount("/", api::routes())
        .launch()
        .await?;

    Ok(())
}
