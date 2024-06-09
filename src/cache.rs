use lazy_static::lazy_static;
use rocket::tokio::time::{self, Duration};
use crate::integrations::{update_all, IntegrationResult};
use dashmap::DashMap;
use std::sync::Arc;
use tokio::task;

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
    // Initialize the cache synchronously
    update_cache().await;
    
    // Start periodic updates asynchronously
    task::spawn(async {
        periodic_update_cache().await;
    });
}

async fn update_cache() {
    // Update data for all integrations
    let data = update_all().await;

    for (integration_name, integration_result) in data {
        match integration_result {
            IntegrationResult::Aws(aws_cache) => {
                CACHE.insert(integration_name, Box::new(aws_cache));
            }
            // Add other integration types here
        }
    }
}

async fn periodic_update_cache() {
    // Start periodic updates
    let mut interval = time::interval(Duration::from_secs(15));
    loop {
        interval.tick().await;
        update_cache().await;
        println!("Cache updated");
    }
}
