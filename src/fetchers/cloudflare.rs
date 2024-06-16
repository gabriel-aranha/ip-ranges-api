use super::Integration;
use crate::cache::IntegrationCache;
use async_trait::async_trait;
use reqwest;
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use uuid::Uuid;

const CLOUDFLARE_IPV4_URL: &str = "https://www.cloudflare.com/ips-v4/";
const CLOUDFLARE_IPV6_URL: &str = "https://www.cloudflare.com/ips-v6/";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CloudflareIpRanges {
    pub ipv4_cidrs: Vec<String>,
    pub ipv6_cidrs: Vec<String>,
}

pub struct CloudflareIntegration {
    execution_id: Uuid,
}

impl CloudflareIntegration {
    pub fn new(execution_id: Uuid) -> Self {
        CloudflareIntegration { execution_id }
    }

    async fn fetch_ip_ranges(url: &str) -> Option<Vec<String>> {
        let response = reqwest::get(url).await.ok()?.text().await.ok()?;
        let cidrs: Vec<String> = response
            .lines()
            .map(|line| line.trim().to_string())
            .collect();
        Some(cidrs)
    }
}

#[async_trait]
impl Integration for CloudflareIntegration {
    type DataModel = CloudflareIpRanges;

    async fn update_cache(&mut self) -> IntegrationCache<Self::DataModel> {
        let ipv4_cidrs = match Self::fetch_ip_ranges(CLOUDFLARE_IPV4_URL).await {
            Some(cidrs) => cidrs,
            None => {
                error!(
                    execution_id = %self.execution_id,
                    "Failed to fetch Cloudflare IPv4 data"
                );
                return IntegrationCache::new(None);
            }
        };

        let ipv6_cidrs = match Self::fetch_ip_ranges(CLOUDFLARE_IPV6_URL).await {
            Some(cidrs) => cidrs,
            None => {
                error!(
                    execution_id = %self.execution_id,
                    "Failed to fetch Cloudflare IPv6 data"
                );
                return IntegrationCache::new(None);
            }
        };

        let data = CloudflareIpRanges {
            ipv4_cidrs,
            ipv6_cidrs,
        };

        info!(
            execution_id = %self.execution_id,
            "Cloudflare cache updated"
        );

        IntegrationCache::new(Some(data))
    }

    fn parse(&self, _data: &str) -> Option<Self::DataModel> {
        // Cloudflare data is directly fetched and parsed, so this is not used
        None
    }
}
