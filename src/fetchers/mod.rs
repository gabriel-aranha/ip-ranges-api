pub mod aws;
pub mod azure;
pub mod gcp;

use crate::cache::IntegrationCache;
use crate::fetchers::aws::AwsIpRanges;
use crate::fetchers::azure::AzureIpRanges;
use crate::fetchers::gcp::GcpIpRanges;
use async_trait::async_trait;
use std::collections::HashMap;
use tracing::{error, info};
use uuid::Uuid;

pub enum IntegrationResult {
	Aws(IntegrationCache<AwsIpRanges>),
	Azure(IntegrationCache<AzureIpRanges>),
	Gcp(IntegrationCache<GcpIpRanges>),
}

#[async_trait]
pub trait Integration {
	type DataModel;

	async fn update_cache(&mut self) -> IntegrationCache<Self::DataModel>;
	fn parse(&self, data: &str) -> Option<Self::DataModel>;
}

pub async fn update_all(execution_id: Uuid) -> HashMap<String, IntegrationResult> {
	let mut all_data = HashMap::new();

	info!(execution_id = %execution_id, "Starting update for all integrations");

	// AWS integration update task
	let aws_task = async {
		info!(execution_id = %execution_id, "Starting AWS integration update");
		let mut aws_integration = aws::AwsIntegration::new(execution_id);
		let aws_cache = aws_integration.update_cache().await;
		if let Some(_aws_data) = &aws_cache.data {
			info!(execution_id = %execution_id, "AWS integration update succeeded");
			Some(("aws".to_string(), IntegrationResult::Aws(aws_cache)))
		} else {
			error!(execution_id = %execution_id, "AWS integration update failed");
			None
		}
	};

	// Azure integration update task
	let azure_task = async {
		info!(execution_id = %execution_id, "Starting Azure integration update");
		let mut azure_integration = azure::AzureIntegration::new(execution_id);
		let azure_cache = azure_integration.update_cache().await;
		if let Some(_azure_data) = &azure_cache.data {
			info!(execution_id = %execution_id, "Azure integration update succeeded");
			Some(("azure".to_string(), IntegrationResult::Azure(azure_cache)))
		} else {
			error!(execution_id = %execution_id, "Azure integration update failed");
			None
		}
	};

	// GCP integration update task
	let gcp_task = async {
		info!(execution_id = %execution_id, "Starting GCP integration update");
		let mut gcp_integration = gcp::GcpIntegration::new(execution_id);
		let gcp_cache = gcp_integration.update_cache().await;
		if let Some(_gcp_data) = &gcp_cache.data {
			info!(execution_id = %execution_id, "GCP integration update succeeded");
			Some(("gcp".to_string(), IntegrationResult::Gcp(gcp_cache)))
		} else {
			error!(execution_id = %execution_id, "GCP integration update failed");
			None
		}
	};

	// Wait for all integration tasks to complete
	let (aws_result, azure_result, gcp_result) = tokio::join!(aws_task, azure_task, gcp_task);

	if let Some((integration_name, integration_result)) = aws_result {
		all_data.insert(integration_name, integration_result);
	}

	if let Some((integration_name, integration_result)) = azure_result {
		all_data.insert(integration_name, integration_result);
	}

	if let Some((integration_name, integration_result)) = gcp_result {
		all_data.insert(integration_name, integration_result);
	}

	info!(execution_id = %execution_id, "Completed update for all integrations");

	all_data
}
