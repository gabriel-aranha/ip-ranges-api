use super::Integration;
use crate::cache::IntegrationCache;
use async_trait::async_trait;
use reqwest;
use rocket::serde::json::serde_json;
use serde::Deserialize;
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
	execution_id: Uuid,
}

impl AwsIntegration {
	pub fn new(execution_id: Uuid) -> Self {
		AwsIntegration { execution_id }
	}
}

#[async_trait]
impl Integration for AwsIntegration {
	type DataModel = AwsIpRanges;

	async fn update_cache(&mut self) -> IntegrationCache<Self::DataModel> {
		let url = "https://ip-ranges.amazonaws.com/ip-ranges.json";
		let response = match reqwest::get(url).await {
			Ok(response) => response.text().await.ok(),
			Err(err) => {
				error!(
					execution_id = %self.execution_id,
					"Failed to fetch AWS data: {}", err);
				return IntegrationCache::new(None);
			}
		};

		let data = self.parse(response.as_ref().unwrap());
		info!(
			execution_id = %self.execution_id,
			"AWS cache updated"
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
