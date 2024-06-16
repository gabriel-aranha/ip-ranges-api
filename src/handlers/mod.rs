pub mod aws;
pub mod azure;
pub mod cloudflare;
pub mod fastly;
pub mod gcp;
pub mod health;
pub mod linode;
pub mod oracle;

use crate::handlers::{
    aws::query_aws_data, azure::query_azure_data, cloudflare::query_cloudflare_data,
    fastly::query_fastly_data, gcp::query_gcp_data, health::health_check,
    linode::query_linode_data, oracle::query_oracle_data,
};

use rocket::{routes, Route};

pub fn routes() -> Vec<Route> {
    routes![
        query_aws_data,
        query_azure_data,
        query_cloudflare_data,
        query_fastly_data,
        query_gcp_data,
        query_linode_data,
        health_check,
        query_oracle_data
    ]
}
