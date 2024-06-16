pub mod aws;
pub mod azure;
pub mod gcp;
pub mod health;

use crate::handlers::aws::query_aws_data;
use crate::handlers::azure::query_azure_data;
use crate::handlers::gcp::query_gcp_data;
use crate::handlers::health::health_check;

use rocket::{routes, Route};

pub fn routes() -> Vec<Route> {
    routes![
        query_aws_data, 
        query_azure_data,
        query_gcp_data,
        health_check,
    ]
}
