use crate::cache::{IntegrationCache, CACHE};
use crate::fetchers::digitalocean::DigitalOceanIpRanges;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Serialize)]
pub struct DigitalOceanApiResponse<T> {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[get("/v1/digitalocean?<alpha2code>&<region>&<ipv4>&<ipv6>")]
pub fn query_digitalocean_data(
    alpha2code: Option<String>,
    region: Option<String>,
    ipv4: Option<bool>,
    ipv6: Option<bool>,
) -> (Status, Json<DigitalOceanApiResponse<Vec<String>>>) {
    // Generate a unique request ID
    let request_id = Uuid::new_v4();

    // Log the start of the request with structured fields for received parameters
    info!(
        request_id = %request_id,
        alpha2code = alpha2code.clone().map(|s| s.to_lowercase()),
        region = region.clone().map(|s| s.to_lowercase()),
        ipv4 = ipv4.unwrap_or(false),
        ipv6 = ipv6.unwrap_or(false),
        "Received request"
    );

    // Check if both ipv4 and ipv6 flags are false or not set
    if !ipv4.unwrap_or(false) && !ipv6.unwrap_or(false) {
        return (
            Status::BadRequest,
            Json(DigitalOceanApiResponse {
                status: "error".to_string(),
                data: None,
                message: Some("Either ipv4 or ipv6 must be specified".to_string()),
            }),
        );
    }

    // Read the global cache
    let cache = CACHE.clone();

    // Access the DigitalOcean cache from the global cache
    if let Some(digitalocean_data_ref) = cache.get("digitalocean") {
        // Extract the DigitalOcean data
        if let Some(digitalocean_cache) =
            digitalocean_data_ref.downcast_ref::<IntegrationCache<DigitalOceanIpRanges>>()
        {
            // Filter the DigitalOcean data based on the provided parameters
            let mut filtered_data: Vec<String> =
                digitalocean_cache.data.as_ref().map_or_else(Vec::new, |data| {
                    let param_alpha2code = alpha2code.clone().map(|s| s.to_uppercase());
                    let param_region = region.clone().map(|s| s.to_lowercase());

                    data.ranges
                        .iter()
                        .filter_map(|range| {
                            let matches = param_alpha2code
                                .as_deref()
                                .map_or(true, |param| param == range.alpha2code.to_uppercase())
                                && param_region.as_deref().map_or(true, |param| {
                                    range.region.to_lowercase().contains(param)
                                });

                            if matches {
                                Some(range.ip_prefix.clone())
                            } else {
                                None
                            }
                        })
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
                    "DigitalOcean data found for request"
                );
                return (
                    Status::Ok,
                    Json(DigitalOceanApiResponse {
                        status: "success".to_string(),
                        data: Some(filtered_data),
                        message: None,
                    }),
                );
            }
        }
    }

    // Log failure to retrieve DigitalOcean data
    error!(
        request_id = %request_id,
        "Failed to retrieve DigitalOcean data"
    );
    (
        Status::NotFound,
        Json(DigitalOceanApiResponse {
            status: "error".to_string(),
            data: None,
            message: Some("DigitalOcean data not found".to_string()),
        }),
    )
}

fn is_ipv4(prefix: &str) -> bool {
    prefix.contains('.')
}
