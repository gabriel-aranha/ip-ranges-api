use crate::integrations::{update_all, IntegrationResult};
use dashmap::DashMap;
use lazy_static::lazy_static;
use rocket::tokio::time::{self, Duration};
use std::sync::Arc;
use tokio::task;
use tracing::info;
use uuid::Uuid;

#[allow(dead_code)]
pub struct IntegrationCache<T> {
    latest_sha: String,
    pub data: Option<T>,
}

impl<T> IntegrationCache<T> {
    pub fn new(latest_sha: String, data: Option<T>) -> Self {
        IntegrationCache { latest_sha, data }
    }
}

lazy_static! {
    // Define the global cache as a map of integration names to their data
    pub static ref CACHE: Arc<DashMap<String, Box<dyn std::any::Any + Send + Sync>>> = Arc::new(DashMap::new());
}

pub async fn initialize_cache() {
    info!("Initializing cache");

    // Initialize the cache synchronously
    update_cache().await;

    // Start periodic updates asynchronously
    task::spawn(async {
        periodic_update_cache().await;
    });
}

async fn update_cache() {
    // Generate a unique execution ID for this cache update
    let execution_id = Uuid::new_v4();
    info!(execution_id = %execution_id, "Starting cache update");

    // Update data for all integrations
    let data = update_all(execution_id).await;

    for (integration_name, integration_result) in data {
        match integration_result {
            IntegrationResult::Aws(aws_cache) => {
                let integration_name_str = integration_name.as_str();
                CACHE.insert(integration_name.clone(), Box::new(aws_cache));
                info!(
                    integration_name = integration_name_str,
                    execution_id = %execution_id,
                    "Cache updated for integration"
                );
            } // Add other integration types here
              // Additional integration types should be matched here
        }
    }

    info!(execution_id = %execution_id, "Cache update completed");
}

async fn periodic_update_cache() {
    info!("Starting periodic cache updates");

    // Start periodic updates
    let mut interval = time::interval(Duration::from_secs(15));
    loop {
        interval.tick().await;
        info!("Performing periodic cache update");
        update_cache().await;
    }
}
