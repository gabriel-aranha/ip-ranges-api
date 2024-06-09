use async_trait::async_trait;
use reqwest;
use serde::Deserialize;
use sha2::{Sha256, Digest};
use hex::encode;
use super::Integration;
use crate::cache::IntegrationCache;
use rocket::serde::json::serde_json;
use tracing::{error, info};
use uuid::Uuid;


#[derive(Debug, Deserialize, Clone)]
pub struct AwsIpRanges {
    pub prefixes: Vec<AwsPrefix>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AwsPrefix {
    pub ip_prefix: String,
    pub region: String,
    pub service: String,
    pub network_border_group: String,
}

pub struct AwsIntegration {
    cached_sha: Option<String>,
}

impl AwsIntegration {
    pub fn new() -> Self {
        AwsIntegration { cached_sha: None }
    }
}

#[async_trait]
impl Integration for AwsIntegration {
    type DataModel = AwsIpRanges;

    async fn update_cache(&mut self, execution_id: Uuid) -> IntegrationCache<Self::DataModel> {
        let url = "https://ip-ranges.amazonaws.com/ip-ranges.json";
        let response = match reqwest::get(url).await {
            Ok(response) => response.text().await.ok(),
            Err(err) => {
                error!("Failed to fetch AWS data: {}", err);
                return IntegrationCache::new("".to_string(), None);
            }
        };

        let new_sha = self.calculate_sha(response.as_ref().unwrap());
        
        if self.cached_sha.as_ref().map_or(true, |sha| sha != &new_sha) {
            let data = self.parse(response.as_ref().unwrap());
            info!(
                execution_id = %execution_id,
                "AWS cache updated"
            );
            self.cached_sha = Some(new_sha.clone());
            IntegrationCache::new(new_sha, data)
        } else {
            IntegrationCache::new(new_sha, None)
        }
    }

    fn parse(&self, data: &str) -> Option<Self::DataModel> {
        match serde_json::from_str(data) {
            Ok(parsed_data) => Some(parsed_data),
            Err(err) => {
                error!("Failed to parse JSON: {}", err);
                None
            }
        }
    }

    fn calculate_sha(&self, data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash_result = hasher.finalize();
        encode(hash_result)
    }
}
