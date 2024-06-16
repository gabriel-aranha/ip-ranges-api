use super::Integration;
use crate::cache::IntegrationCache;
use async_trait::async_trait;
use reqwest;
use rocket::serde::json::serde_json;
use serde::Deserialize;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct DigitalOceanIpRanges {
    pub ranges: Vec<DigitalOceanRange>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DigitalOceanRange {
    pub ip_prefix: String,
    pub alpha2code: String,
    pub region: String,
}

pub struct DigitalOceanIntegration {
    execution_id: Uuid,
}

impl DigitalOceanIntegration {
    pub fn new(execution_id: Uuid) -> Self {
        DigitalOceanIntegration { execution_id }
    }

    async fn fetch_ip_ranges() -> Option<Vec<DigitalOceanRange>> {
        let url = "https://digitalocean.com/geo/google.csv";
        let response = reqwest::get(url).await.ok()?.text().await.ok()?;
        let mut ip_ranges = Vec::new();

        for line in response.lines() {
            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() >= 3 {
                let ip_prefix = fields[0].trim().to_owned();
                let alpha2code = fields[1].trim().to_owned();
                let region = fields[2].trim().to_owned();

                ip_ranges.push(DigitalOceanRange {
                    ip_prefix,
                    alpha2code,
                    region,
                });
            }
        }

        Some(ip_ranges)
    }
}

#[async_trait]
impl Integration for DigitalOceanIntegration {
    type DataModel = DigitalOceanIpRanges;

    async fn update_cache(&mut self) -> IntegrationCache<Self::DataModel> {
        let ip_ranges = match Self::fetch_ip_ranges().await {
            Some(ranges) => ranges,
            None => {
                error!(
                    execution_id = %self.execution_id,
                    "Failed to fetch DigitalOcean IP ranges"
                );
                return IntegrationCache::new(None);
            }
        };

        let data_model = DigitalOceanIpRanges { ranges: ip_ranges };
        info!(
            execution_id = %self.execution_id,
            "DigitalOcean cache updated"
        );

        IntegrationCache::new(Some(data_model))
    }

    fn parse(&self, data: &str) -> Option<Self::DataModel> {
        match serde_json::from_str(data) {
            Ok(parsed_data) => Some(parsed_data),
            Err(err) => {
                error!(
                    execution_id = %self.execution_id,
                    "Failed to parse DigitalOcean JSON: {}", err
                );
                None
            }
        }
    }
}
