use crate::cache::{IntegrationCache, CACHE};
use crate::fetchers::cloudflare::CloudflareIpRanges;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;
use tracing::{error, info};
use uuid::Uuid;

// Define the CloudflareApiResponse struct
#[derive(Serialize)]
pub struct CloudflareApiResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[get("/v1/cloudflare?<ipv4>&<ipv6>")]
pub fn query_cloudflare_data(
    ipv4: Option<bool>,
    ipv6: Option<bool>,
) -> (Status, Json<CloudflareApiResponse>) {
    // Generate a unique request ID
    let request_id = Uuid::new_v4();

    // Log the start of the request with structured fields for received parameters
    info!(
        request_id = %request_id,
        ipv4 = ipv4.unwrap_or(false),
        ipv6 = ipv6.unwrap_or(false),
        "Received Cloudflare data request"
    );

    // Check if both ipv4 and ipv6 flags are false or not set
    if !ipv4.unwrap_or(false) && !ipv6.unwrap_or(false) {
        return (
            Status::BadRequest,
            Json(CloudflareApiResponse {
                status: "error".to_string(),
                data: None,
                message: Some("Either ipv4 or ipv6 must be specified".to_string()),
            }),
        );
    }

    // Read the global cache
    let cache = CACHE.clone();

    // Access the Cloudflare cache from the global cache
    if let Some(cloudflare_data_ref) = cache.get("cloudflare") {
        // Extract the Cloudflare data
        if let Some(cloudflare_cache) =
            cloudflare_data_ref.downcast_ref::<IntegrationCache<CloudflareIpRanges>>()
        {
            // Prepare response based on requested IP versions
            let filtered_data: Vec<String> =
                cloudflare_cache
                    .data
                    .as_ref()
                    .map_or_else(Vec::new, |data| {
                        let mut addresses: Vec<String> = Vec::new();

                        // Include IPv4 addresses if requested
                        if ipv4.unwrap_or(false) {
                            addresses.extend(data.ipv4_cidrs.iter().cloned());
                        }

                        // Include IPv6 addresses if requested
                        if ipv6.unwrap_or(false) {
                            addresses.extend(data.ipv6_cidrs.iter().cloned());
                        }

                        addresses
                    });

            // If filtered data is found, return it as JSON
            if !filtered_data.is_empty() {
                info!(
                    request_id = %request_id,
                    "Cloudflare data found for request"
                );
                return (
                    Status::Ok,
                    Json(CloudflareApiResponse {
                        status: "success".to_string(),
                        data: Some(filtered_data),
                        message: None,
                    }),
                );
            }
        }
    }

    // Log failure to retrieve Cloudflare data
    error!(
        request_id = %request_id,
        "Failed to retrieve Cloudflare data"
    );

    (
        Status::NotFound,
        Json(CloudflareApiResponse {
            status: "error".to_string(),
            data: None,
            message: Some("Cloudflare data not found".to_string()),
        }),
    )
}
