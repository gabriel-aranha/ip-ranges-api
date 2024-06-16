use super::Integration;
use crate::cache::IntegrationCache;
use async_trait::async_trait;
use reqwest;
use rocket::serde::json::serde_json;
use serde::Deserialize;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct OracleIpRanges {
    pub regions: Vec<OracleRegion>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OracleRegion {
    pub region: String,
    pub cidrs: Vec<OracleCidr>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OracleCidr {
    pub cidr: String,
    #[serde(default = "default_tags")]
    pub tags: Vec<String>,
}

fn default_tags() -> Vec<String> {
    Vec::new()
}

pub struct OracleIntegration {
    execution_id: Uuid,
}

impl OracleIntegration {
    pub fn new(execution_id: Uuid) -> Self {
        OracleIntegration { execution_id }
    }

    async fn fetch_ip_ranges() -> Option<Vec<OracleRegion>> {
        let url = "https://docs.oracle.com/en-us/iaas/tools/public_ip_ranges.json";
        let response = reqwest::get(url).await.ok()?.text().await.ok()?;
        let ip_ranges: OracleIpRanges = match serde_json::from_str(&response) {
            Ok(data) => data,
            Err(err) => {
                error!(
                    "Failed to parse Oracle IP ranges JSON: {}. Response: {}",
                    err, response
                );
                return None;
            }
        };

        Some(ip_ranges.regions)
    }
}

#[async_trait]
impl Integration for OracleIntegration {
    type DataModel = OracleIpRanges;

    async fn update_cache(&mut self) -> IntegrationCache<Self::DataModel> {
        let ip_ranges = match Self::fetch_ip_ranges().await {
            Some(ranges) => ranges,
            None => {
                error!(
                    execution_id = %self.execution_id,
                    "Failed to fetch Oracle IP ranges"
                );
                return IntegrationCache::new(None);
            }
        };

        let data_model = OracleIpRanges { regions: ip_ranges };
        info!(
            execution_id = %self.execution_id,
            "Oracle cache updated"
        );

        IntegrationCache::new(Some(data_model))
    }

    fn parse(&self, data: &str) -> Option<Self::DataModel> {
        match serde_json::from_str(data) {
            Ok(parsed_data) => Some(parsed_data),
            Err(err) => {
                error!(
                    execution_id = %self.execution_id,
                    "Failed to parse Oracle JSON: {}", err
                );
                None
            }
        }
    }
}
