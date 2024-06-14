use super::Integration;
use crate::cache::IntegrationCache;
use async_trait::async_trait;
use reqwest;
use rocket::serde::json::serde_json;
use serde::Deserialize;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct GcpIpRanges {
    pub prefixes: Vec<GcpPrefix>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GcpPrefix {
    #[serde(rename = "ipv4Prefix", skip_serializing_if = "Option::is_none")]
    pub ipv4_prefix: Option<String>,
    #[serde(rename = "ipv6Prefix", skip_serializing_if = "Option::is_none")]
    pub ipv6_prefix: Option<String>,
    pub service: String,
    pub scope: String,
}

pub struct GcpIntegration {
    execution_id: Uuid,
}

impl GcpIntegration {
    pub fn new(execution_id: Uuid) -> Self {
        GcpIntegration { execution_id }
    }
}

#[async_trait]
impl Integration for GcpIntegration {
    type DataModel = GcpIpRanges;

    async fn update_cache(&mut self) -> IntegrationCache<Self::DataModel> {
        let url = "https://www.gstatic.com/ipranges/cloud.json";
        let response = match reqwest::get(url).await {
            Ok(response) => response.text().await.ok(),
            Err(err) => {
                error!(
                    execution_id = %self.execution_id,
                    "Failed to fetch GCP data: {}", err
                );
                return IntegrationCache::new(None);
            }
        };

        let data = self.parse(response.as_ref().unwrap());
        info!(
            execution_id = %self.execution_id,
            "GCP cache updated"
        );

        IntegrationCache::new(data)
    }

    fn parse(&self, data: &str) -> Option<Self::DataModel> {
        match serde_json::from_str(data) {
            Ok(parsed_data) => Some(parsed_data),
            Err(err) => {
                error!(
                    execution_id = %self.execution_id,
                    "Failed to parse JSON: {}", err
                );
                None
            }
        }
    }
}
