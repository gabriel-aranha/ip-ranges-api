pub mod aws;
use crate::cache::IntegrationCache;
use crate::integrations::aws::AwsIpRanges;
use async_trait::async_trait;
use std::collections::HashMap;
use tracing::{error, info};
use uuid::Uuid;

pub enum IntegrationResult {
    Aws(IntegrationCache<AwsIpRanges>),
    // Define other integration types here
}

#[async_trait]
pub trait Integration {
    type DataModel;

    async fn update_cache(&mut self) -> IntegrationCache<Self::DataModel>;
    fn parse(&self, data: &str) -> Option<Self::DataModel>;
    fn calculate_sha(&self, data: &str) -> String;
}

pub async fn update_all(execution_id: Uuid) -> HashMap<String, IntegrationResult> {
    let mut all_data = HashMap::new();

    info!(execution_id = %execution_id, "Starting update for all integrations");

    // Update data for the AWS integration
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

    // Add other integration tasks here

    // Wait for all integration tasks to complete
    let (aws_result /* other_result */,) = tokio::join!(aws_task /* , other_integration_task */);

    if let Some((integration_name, integration_result)) = aws_result {
        all_data.insert(integration_name, integration_result);
    }

    // Add other integration results here

    info!(execution_id = %execution_id, "Completed update for all integrations");

    all_data
}
