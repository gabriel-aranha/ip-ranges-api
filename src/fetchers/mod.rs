// fetchers.rs

pub mod aws;
pub mod azure;
pub mod cloudflare;
pub mod fastly;
pub mod gcp;
pub mod linode;
pub mod oracle;

use crate::cache::IntegrationCache;
use async_trait::async_trait;
use std::collections::HashMap;
use tracing::{error, info};
use uuid::Uuid;

use aws::AwsIpRanges;
use azure::AzureIpRanges;
use cloudflare::CloudflareIpRanges;
use fastly::FastlyIpRanges;
use gcp::GcpIpRanges;
use linode::LinodeIpRanges;
use oracle::OracleIpRanges;

pub enum IntegrationResult {
    Aws(IntegrationCache<AwsIpRanges>),
    Azure(IntegrationCache<AzureIpRanges>),
    Cloudflare(IntegrationCache<CloudflareIpRanges>),
    Fastly(IntegrationCache<FastlyIpRanges>),
    Gcp(IntegrationCache<GcpIpRanges>),
    Linode(IntegrationCache<LinodeIpRanges>),
    Oracle(IntegrationCache<OracleIpRanges>),
}

#[async_trait]
pub trait Integration {
    type DataModel;

    async fn update_cache(&mut self) -> IntegrationCache<Self::DataModel>;
    fn parse(&self, data: &str) -> Option<Self::DataModel>;
}

// Update the update_all function to include Oracle and Cloudflare tasks
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

    // Cloudflare integration update task
    let cloudflare_task = async {
        info!(execution_id = %execution_id, "Starting Cloudflare integration update");
        let mut cloudflare_integration = cloudflare::CloudflareIntegration::new(execution_id);
        let cloudflare_cache = cloudflare_integration.update_cache().await;
        if let Some(_cloudflare_data) = &cloudflare_cache.data {
            info!(execution_id = %execution_id, "Cloudflare integration update succeeded");
            Some((
                "cloudflare".to_string(),
                IntegrationResult::Cloudflare(cloudflare_cache),
            ))
        } else {
            error!(execution_id = %execution_id, "Cloudflare integration update failed");
            None
        }
    };

    // Fastly integration update task
    let fastly_task = async {
        info!(execution_id = %execution_id, "Starting Fastly integration update");
        let mut fastly_integration = fastly::FastlyIntegration::new(execution_id);
        let fastly_cache = fastly_integration.update_cache().await;
        if let Some(_fastly_data) = &fastly_cache.data {
            info!(execution_id = %execution_id, "Fastly integration update succeeded");
            Some((
                "fastly".to_string(),
                IntegrationResult::Fastly(fastly_cache),
            ))
        } else {
            error!(execution_id = %execution_id, "Fastly integration update failed");
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

    // Linode integration update task
    let linode_task = async {
        info!(execution_id = %execution_id, "Starting Linode integration update");
        let mut linode_integration = linode::LinodeIntegration::new(execution_id);
        let linode_cache = linode_integration.update_cache().await;
        if let Some(_linode_data) = &linode_cache.data {
            info!(execution_id = %execution_id, "Linode integration update succeeded");
            Some((
                "linode".to_string(),
                IntegrationResult::Linode(linode_cache),
            ))
        } else {
            error!(execution_id = %execution_id, "Linode integration update failed");
            None
        }
    };

    // Oracle integration update task
    let oracle_task = async {
        info!(execution_id = %execution_id, "Starting Oracle integration update");
        let mut oracle_integration = oracle::OracleIntegration::new(execution_id);
        let oracle_cache = oracle_integration.update_cache().await;
        if let Some(_oracle_data) = &oracle_cache.data {
            info!(execution_id = %execution_id, "Oracle integration update succeeded");
            Some((
                "oracle".to_string(),
                IntegrationResult::Oracle(oracle_cache),
            ))
        } else {
            error!(execution_id = %execution_id, "Oracle integration update failed");
            None
        }
    };

    // Wait for all integration tasks to complete
    let (
        aws_result,
        azure_result,
        cloudflare_result,
        fastly_result,
        gcp_result,
        linode_result,
        oracle_result,
    ) = tokio::join!(
        aws_task,
        azure_task,
        cloudflare_task,
        fastly_task,
        gcp_task,
        linode_task,
        oracle_task
    );

    if let Some((integration_name, integration_result)) = aws_result {
        all_data.insert(integration_name, integration_result);
    }

    if let Some((integration_name, integration_result)) = azure_result {
        all_data.insert(integration_name, integration_result);
    }

    if let Some((integration_name, integration_result)) = cloudflare_result {
        all_data.insert(integration_name, integration_result);
    }

    if let Some((integration_name, integration_result)) = fastly_result {
        all_data.insert(integration_name, integration_result);
    }

    if let Some((integration_name, integration_result)) = gcp_result {
        all_data.insert(integration_name, integration_result);
    }

    if let Some((integration_name, integration_result)) = linode_result {
        all_data.insert(integration_name, integration_result);
    }

    if let Some((integration_name, integration_result)) = oracle_result {
        all_data.insert(integration_name, integration_result);
    }

    info!(execution_id = %execution_id, "Completed update for all integrations");

    all_data
}
