use crate::cache::IntegrationCache;
use crate::cache::CACHE;
use crate::fetchers::aws::AwsIpRanges;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;
use tracing::{error, info};
use uuid::Uuid;

// Make the AwsApiResponse struct public
#[derive(Serialize)]
pub struct AwsApiResponse<T> {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[get("/v1/aws?<region>&<service>&<network_border_group>")]
pub fn query_aws_data(
    region: Option<String>,
    service: Option<String>,
    network_border_group: Option<String>,
) -> (Status, Json<AwsApiResponse<Vec<String>>>) {
    // Generate a unique request ID
    let request_id = Uuid::new_v4();

    // Log the start of the request with structured fields for received parameters
    info!(
        request_id = %request_id,
        region = region.clone().map(|s| s.to_lowercase()),
        service = service.clone().map(|s| s.to_lowercase()),
        network_border_group = network_border_group.clone().map(|s| s.to_lowercase()),
        "Received request"
    );

    // Read the global cache
    let cache = CACHE.clone();

    // Access the AWS cache from the global cache
    if let Some(aws_data_ref) = cache.get("aws") {
        // Extract the AWS data
        if let Some(aws_cache) = aws_data_ref.downcast_ref::<IntegrationCache<AwsIpRanges>>() {
            // Filter the AWS data based on the provided parameters
            let filtered_data: Vec<String> =
                aws_cache.data.as_ref().map_or_else(Vec::new, |data| {
                    data.prefixes
                        .iter()
                        .filter_map(|prefix| {
                            // Convert both parameter and data to lowercase for case-insensitive comparison
                            let param_region = region.clone().map(|s| s.to_lowercase());
                            let param_service = service.clone().map(|s| s.to_lowercase());
                            let param_network_border_group =
                                network_border_group.clone().map(|s| s.to_lowercase());

                            let matches = param_region
                                .as_deref()
                                .map_or(true, |param| prefix.region.to_lowercase() == param)
                                && param_service
                                    .as_deref()
                                    .map_or(true, |param| prefix.service.to_lowercase() == param)
                                && param_network_border_group.as_deref().map_or(true, |param| {
                                    prefix.network_border_group.to_lowercase() == param
                                });

                            if matches {
                                // Return the IP prefix as String
                                Some(prefix.ip_prefix.clone())
                            } else {
                                None
                            }
                        })
                        .collect()
                });

            // If filtered data is found, return it as JSON
            if !filtered_data.is_empty() {
                info!(
                    request_id = %request_id,
                    "AWS data found for request"
                );
                return (
                    Status::Ok,
                    Json(AwsApiResponse {
                        status: "success".to_string(),
                        data: Some(filtered_data),
                        message: None,
                    }),
                );
            }
        }
    }

    // Log failure to retrieve AWS data
    error!(
        request_id = %request_id,
        "Failed to retrieve AWS data"
    );

    (
        Status::NotFound,
        Json(AwsApiResponse {
            status: "error".to_string(),
            data: None,
            message: Some("AWS data not found".to_string()),
        }),
    )
}
