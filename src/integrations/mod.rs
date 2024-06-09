pub mod aws;
use async_trait::async_trait;
use crate::integrations::aws::AwsIpRanges;
use std::collections::HashMap;
use crate::cache::IntegrationCache;

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

pub async fn update_all() -> HashMap<String, IntegrationResult> {
    let mut all_data = HashMap::new();

    // Update data for the AWS integration
    let mut aws_integration = aws::AwsIntegration::new();
    let aws_cache = aws_integration.update_cache().await;
    if let Some(_aws_data) = &aws_cache.data {
        all_data.insert("aws".to_string(), IntegrationResult::Aws(aws_cache));
    }

    // Update data for other integrations here

    all_data
}
