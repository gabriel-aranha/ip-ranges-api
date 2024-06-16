use crate::cache::{IntegrationCache, CACHE};
use crate::fetchers::azure::AzureIpRanges;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Serialize)]
pub struct AzureApiResponse<T> {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[get("/v1/azure?<region>&<system_service>&<ipv4>&<ipv6>")]
pub fn query_azure_data(
    region: Option<String>,
    system_service: Option<String>,
    ipv4: Option<bool>,
    ipv6: Option<bool>,
) -> (Status, Json<AzureApiResponse<Vec<String>>>) {
    // Generate a unique request ID
    let request_id = Uuid::new_v4();

    // Log the start of the request with structured fields for received parameters
    info!(
        request_id = %request_id,
        region = region.clone().map(|s| s.to_lowercase()),
        system_service = system_service.clone().map(|s| s.to_lowercase()),
        ipv4 = ipv4.unwrap_or(false),
        ipv6 = ipv6.unwrap_or(false),
        "Received request"
    );

    // Check if both ipv4 and ipv6 flags are false or not set
    if !ipv4.unwrap_or(false) && !ipv6.unwrap_or(false) {
        return (
            Status::BadRequest,
            Json(AzureApiResponse {
                status: "error".to_string(),
                data: None,
                message: Some("Either ipv4 or ipv6 must be specified".to_string()),
            }),
        );
    }

    // Read the global cache
    let cache = CACHE.clone();

    // Access the Azure cache from the global cache
    if let Some(azure_data_ref) = cache.get("azure") {
        // Extract the Azure data
        if let Some(azure_cache) = azure_data_ref.downcast_ref::<IntegrationCache<AzureIpRanges>>()
        {
            // Filter the Azure data based on the provided parameters
            let mut filtered_data: Vec<String> =
                azure_cache.data.as_ref().map_or_else(Vec::new, |data| {
                    let param_region = region.clone().map(|s| s.to_lowercase());
                    let param_system_service = system_service.clone().map(|s| s.to_lowercase());

                    data.values
                        .iter()
                        .filter_map(|value| {
                            let matches =
                                param_region.as_deref().map_or(true, |param| {
                                    value.properties.region.to_lowercase() == param
                                }) && param_system_service.as_deref().map_or(true, |param| {
                                    value.properties.system_service.to_lowercase() == param
                                });

                            if matches {
                                Some(value.properties.address_prefixes.clone())
                            } else {
                                None
                            }
                        })
                        .flatten()
                        .collect()
                });

            let ipv4_flag = ipv4.unwrap_or(false);
            let ipv6_flag = ipv6.unwrap_or(false);

            filtered_data.retain(|prefix| {
                let is_ipv4 = is_ipv4(prefix);
                if ipv4_flag && !ipv6_flag {
                    is_ipv4
                } else if !ipv4_flag && ipv6_flag {
                    !is_ipv4
                } else {
                    true
                }
            });

            // Return the filtered data as a JSON response
            if !filtered_data.is_empty() {
                info!(
                    request_id = %request_id,
                    "Azure data found for request"
                );
                return (
                    Status::Ok,
                    Json(AzureApiResponse {
                        status: "success".to_string(),
                        data: Some(filtered_data),
                        message: None,
                    }),
                );
            }
        }
    }

    // Log failure to retrieve Azure data
    error!(
        request_id = %request_id,
        "Failed to retrieve Azure data"
    );
    (
        Status::NotFound,
        Json(AzureApiResponse {
            status: "error".to_string(),
            data: None,
            message: Some("Azure data not found".to_string()),
        }),
    )
}

fn is_ipv4(prefix: &str) -> bool {
    prefix.contains('.')
}
