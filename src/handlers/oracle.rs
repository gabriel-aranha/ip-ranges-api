use crate::cache::{IntegrationCache, CACHE};
use crate::fetchers::oracle::OracleIpRanges;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Serialize)]
pub struct OracleApiResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[get("/v1/oracle?<region>&<tag>")]
pub async fn query_oracle_data(
    region: Option<String>,
    tag: Option<String>,
) -> (Status, Json<OracleApiResponse>) {
    // Generate a unique request ID
    let request_id = Uuid::new_v4();

    // Log the start of the request with structured fields for received parameters
    info!(
        request_id = %request_id,
        region = region.clone().map(|s| s.to_lowercase()),
        tag = tag.clone().map(|s| s.to_lowercase()),
        "Received Oracle data request"
    );

    // Read the global cache
    let cache = CACHE.clone();

    // Access the Oracle cache from the global cache
    if let Some(oracle_data_ref) = cache.get("oracle") {
        // Extract the Oracle data
        if let Some(oracle_cache) =
            oracle_data_ref.downcast_ref::<IntegrationCache<OracleIpRanges>>()
        {
            // Prepare response based on requested region and tag
            let filtered_data: Vec<String> =
                oracle_cache
                    .data
                    .as_ref()
                    .map_or_else(Vec::new, |oracle_ranges| {
                        let param_region = region.clone().map(|s| s.to_lowercase());
                        let param_tag = tag.clone().map(|s| s.to_lowercase());

                        let mut addresses: Vec<String> = Vec::new();

                        for oracle_region in &oracle_ranges.regions {
                            let region_matches = param_region.as_ref().map_or(true, |req_region| {
                                oracle_region.region.to_lowercase() == *req_region
                            });

                            if region_matches {
                                // Iterate through the CIDRs in the Oracle region
                                for cidr in &oracle_region.cidrs {
                                    let tag_matches = param_tag.as_ref().map_or(true, |req_tag| {
                                        cidr.tags.iter().any(|t| t.to_lowercase() == *req_tag)
                                    });

                                    if tag_matches {
                                        addresses.push(cidr.cidr.clone());
                                    }
                                }
                            }
                        }

                        addresses
                    });

            // Log the number of filtered IP prefixes
            info!(
                request_id = %request_id,
                num_filtered_ips = filtered_data.len(),
                "Filtered Oracle data"
            );

            // If filtered data is found, return it as JSON
            if !filtered_data.is_empty() {
                info!(
                    request_id = %request_id,
                    "Oracle data found for request"
                );
                return (
                    Status::Ok,
                    Json(OracleApiResponse {
                        status: "success".to_string(),
                        data: Some(filtered_data),
                        message: None,
                    }),
                );
            }
        }
    }

    // Log failure to retrieve Oracle data
    error!(
        request_id = %request_id,
        "Failed to retrieve Oracle data"
    );

    (
        Status::NotFound,
        Json(OracleApiResponse {
            status: "error".to_string(),
            data: None,
            message: Some("Oracle data not found".to_string()),
        }),
    )
}
