use crate::cache::{IntegrationCache, CACHE};
use crate::fetchers::gcp::GcpIpRanges;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Serialize)]
pub struct GcpApiResponse<T> {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[get("/v1/gcp?<scope>&<service>&<ipv4>&<ipv6>")]
pub fn query_gcp_data(
    scope: Option<String>,
    service: Option<String>,
    ipv4: Option<bool>,
    ipv6: Option<bool>,
) -> (Status, Json<GcpApiResponse<Vec<String>>>) {
    // Generate a unique request ID
    let request_id = Uuid::new_v4();

    // Log the start of the request with structured fields for received parameters
    info!(
        request_id = %request_id,
        scope = scope.clone().map(|s| s.to_lowercase()),
        service = service.clone().map(|s| s.to_lowercase()),
        ipv4 = ipv4.unwrap_or(false),
        ipv6 = ipv6.unwrap_or(false),
        "Received request"
    );

    // Check if both ipv4 and ipv6 flags are false or not set
    if !ipv4.unwrap_or(false) && !ipv6.unwrap_or(false) {
        return (
            Status::BadRequest,
            Json(GcpApiResponse {
                status: "error".to_string(),
                data: None,
                message: Some("Either ipv4 or ipv6 must be specified".to_string()),
            }),
        );
    }

    // Read the global cache
    let cache = CACHE.clone();

    // Access the GCP cache from the global cache
    if let Some(gcp_data_ref) = cache.get("gcp") {
        // Extract the GCP data
        if let Some(gcp_cache) = gcp_data_ref.downcast_ref::<IntegrationCache<GcpIpRanges>>() {
            // Filter the GCP data based on the provided parameters
            let filtered_data: Vec<String> =
                gcp_cache.data.as_ref().map_or_else(Vec::new, |data| {
                    data.prefixes
                        .iter()
                        .filter_map(|prefix| {
                            // Convert both parameter and data to lowercase for case-insensitive comparison
                            let param_scope = scope.clone().map(|s| s.to_lowercase());
                            let param_service = service.clone().map(|s| s.to_lowercase());

                            let matches = param_scope
                                .as_deref()
                                .map_or(true, |param| prefix.scope.to_lowercase() == param)
                                && param_service
                                    .as_deref()
                                    .map_or(true, |param| prefix.service.to_lowercase() == param)
                                && ((ipv4.unwrap_or(false) && prefix.ipv4_prefix.is_some())
                                    || (ipv6.unwrap_or(false) && prefix.ipv6_prefix.is_some()));

                            if matches {
                                // Return the appropriate IP prefix as String
                                if ipv4.unwrap_or(false) && prefix.ipv4_prefix.is_some() {
                                    Some(prefix.ipv4_prefix.as_ref().unwrap().clone())
                                } else if ipv6.unwrap_or(false) && prefix.ipv6_prefix.is_some() {
                                    Some(prefix.ipv6_prefix.as_ref().unwrap().clone())
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .collect()
                });

            // Serialize the filtered data to JSON string
            if !filtered_data.is_empty() {
                info!(
                    request_id = %request_id,
                    "GCP data found for request"
                );
                return (
                    Status::Ok,
                    Json(GcpApiResponse {
                        status: "success".to_string(),
                        data: Some(filtered_data),
                        message: None,
                    }),
                );
            }
        }
    }

    // Log failure to retrieve GCP data
    error!(
        request_id = %request_id,
        "Failed to retrieve GCP data"
    );
    (
        Status::NotFound,
        Json(GcpApiResponse {
            status: "error".to_string(),
            data: None,
            message: Some("GCP data not found".to_string()),
        }),
    )
}
