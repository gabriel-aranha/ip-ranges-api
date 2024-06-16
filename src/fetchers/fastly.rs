use super::Integration;
use crate::cache::IntegrationCache;
use async_trait::async_trait;
use reqwest;
use rocket::serde::json::serde_json;
use serde::Deserialize;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct FastlyIpRanges {
    #[serde(rename = "addresses")]
    pub ipv4_addresses: Vec<String>,
    pub ipv6_addresses: Vec<String>,
}

pub struct FastlyIntegration {
    execution_id: Uuid,
}

impl FastlyIntegration {
    pub fn new(execution_id: Uuid) -> Self {
        FastlyIntegration { execution_id }
    }
}

#[async_trait]
impl Integration for FastlyIntegration {
    type DataModel = FastlyIpRanges;

    async fn update_cache(&mut self) -> IntegrationCache<Self::DataModel> {
        let url = "https://api.fastly.com/public-ip-list";
        let response = match reqwest::get(url).await {
            Ok(response) => response.text().await.ok(),
            Err(err) => {
                error!(
                    execution_id = %self.execution_id,
                    "Failed to fetch Fastly data: {}", err);
                return IntegrationCache::new(None);
            }
        };

        let data = self.parse(response.as_ref().unwrap());
        info!(
            execution_id = %self.execution_id,
            "Fastly cache updated"
        );

        IntegrationCache::new(data)
    }

    fn parse(&self, data: &str) -> Option<Self::DataModel> {
        match serde_json::from_str(data) {
            Ok(parsed_data) => Some(parsed_data),
            Err(err) => {
                error!(
                    execution_id = %self.execution_id,
                    "Failed to parse JSON: {}", err);
                None
            }
        }
    }
}
