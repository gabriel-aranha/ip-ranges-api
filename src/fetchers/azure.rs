use super::Integration;
use crate::cache::IntegrationCache;
use async_trait::async_trait;
use reqwest;
use rocket::serde::json::serde_json;
use serde::Deserialize;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AzureIpRanges {
    pub change_number: u32,
    pub cloud: String,
    pub values: Vec<AzureValue>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AzureValue {
    pub name: String,
    pub id: String,
    pub properties: AzureProperties,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AzureProperties {
    pub change_number: u32,
    pub region: String,
    pub region_id: u32,
    pub platform: String,
    pub system_service: String,
    pub address_prefixes: Vec<String>,
}

pub struct AzureIntegration {
    execution_id: Uuid,
}

impl AzureIntegration {
    pub fn new(execution_id: Uuid) -> Self {
        AzureIntegration { execution_id }
    }
}

#[async_trait]
impl Integration for AzureIntegration {
    type DataModel = AzureIpRanges;

    async fn update_cache(&mut self) -> IntegrationCache<Self::DataModel> {
        let url = "https://raw.githubusercontent.com/femueller/cloud-ip-ranges/master/microsoft-azure-ip-ranges.json";
        let response = match reqwest::get(url).await {
            Ok(response) => response.text().await.ok(),
            Err(err) => {
                error!(
                    execution_id = %self.execution_id,
                    "Failed to fetch Azure data: {}", err);
                return IntegrationCache::new(None);
            }
        };

        let data = self.parse(response.as_ref().unwrap());
        info!(
            execution_id = %self.execution_id,
            "Azure cache updated"
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
