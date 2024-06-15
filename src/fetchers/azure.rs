use super::Integration;
use crate::cache::IntegrationCache;
use async_trait::async_trait;
use reqwest;
use rocket::serde::json::serde_json;
use scraper::{Html, Selector};
use serde::Deserialize;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AzureIpRanges {
    pub values: Vec<AzureValue>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AzureValue {
    pub properties: AzureProperties,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AzureProperties {
    pub region: String,
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

    async fn fetch_latest_url() -> Option<String> {
        let url = "https://www.microsoft.com/en-us/download/confirmation.aspx?id=56519";
        let response = reqwest::get(url).await.ok()?.text().await.ok()?;
        
        let document = Html::parse_document(&response);
        let selector = Selector::parse("a").unwrap();

        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                if href.contains("ServiceTags_") {
                    return Some(href.to_string());
                }
            }
        }
        None
    }
}

#[async_trait]
impl Integration for AzureIntegration {
    type DataModel = AzureIpRanges;

    async fn update_cache(&mut self) -> IntegrationCache<Self::DataModel> {
        let url = match Self::fetch_latest_url().await {
            Some(url) => url,
            None => {
                error!(
                    execution_id = %self.execution_id,
                    "Failed to find the latest Azure IP ranges URL"
                );
                return IntegrationCache::new(None);
            }
        };

        let response = match reqwest::get(&url).await {
            Ok(response) => response.text().await.ok(),
            Err(err) => {
                error!(
                    execution_id = %self.execution_id,
                    "Failed to fetch Azure data: {}", err
                );
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
                    "Failed to parse JSON: {}", err
                );
                None
            }
        }
    }
}
