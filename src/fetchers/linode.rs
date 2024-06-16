use super::Integration;
use crate::cache::IntegrationCache;
use async_trait::async_trait;
use reqwest;
use rocket::serde::json::serde_json;
use serde::Deserialize;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct LinodeIpRanges {
    pub ranges: Vec<LinodeRange>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LinodeRange {
    pub ip_prefix: String,
    pub alpha2code: String,
    pub region: String,
}

pub struct LinodeIntegration {
    execution_id: Uuid,
}

impl LinodeIntegration {
    pub fn new(execution_id: Uuid) -> Self {
        LinodeIntegration { execution_id }
    }

    async fn fetch_ip_ranges() -> Option<Vec<LinodeRange>> {
        let url = "https://geoip.linode.com/";
        let response = reqwest::get(url).await.ok()?.text().await.ok()?;
        let mut ip_ranges = Vec::new();

        for line in response.lines().skip(3) {
            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() >= 3 {
                let ip_prefix = fields[0].trim();
                let alpha2code = fields[1].trim().to_owned();
                let region = fields[2].trim().to_owned();

                ip_ranges.push(LinodeRange {
                    ip_prefix: ip_prefix.to_owned(),
                    alpha2code,
                    region,
                });
            }
        }

        Some(ip_ranges)
    }
}

#[async_trait]
impl Integration for LinodeIntegration {
    type DataModel = LinodeIpRanges;

    async fn update_cache(&mut self) -> IntegrationCache<Self::DataModel> {
        let ip_ranges = match Self::fetch_ip_ranges().await {
            Some(ranges) => ranges,
            None => {
                error!(
                    execution_id = %self.execution_id,
                    "Failed to fetch Linode IP ranges"
                );
                return IntegrationCache::new(None);
            }
        };

        let data_model = LinodeIpRanges { ranges: ip_ranges };
        info!(
            execution_id = %self.execution_id,
            "Linode cache updated"
        );

        IntegrationCache::new(Some(data_model))
    }

    fn parse(&self, data: &str) -> Option<Self::DataModel> {
        match serde_json::from_str(data) {
            Ok(parsed_data) => Some(parsed_data),
            Err(err) => {
                error!(
                    execution_id = %self.execution_id,
                    "Failed to parse Linode JSON: {}", err
                );
                None
            }
        }
    }
}
