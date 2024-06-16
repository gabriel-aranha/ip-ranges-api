pub mod aws;
pub mod azure;
pub mod health;
pub mod gcp;

use crate::handlers::aws::query_aws_data;
use crate::handlers::azure::query_azure_data;
use crate::habdlers::health::health_check;
use crate::handlers::gcp::query_gcp_data;

use rocket::{routes, Route};

pub fn routes() -> Vec<Route> {
    routes![query_aws_data, query_azure_data, health_check, query_gcp_data]
}
