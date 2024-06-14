use crate::cache::IntegrationCache;
use crate::cache::CACHE;
use crate::fetchers::gcp::GcpIpRanges;
use rocket::get;
use rocket::serde::json::serde_json;
use tracing::{error, info};
use uuid::Uuid;

#[get("/v1/gcp?<scope>&<service>&<ipv4>&<ipv6>")]
pub fn query_gcp_data(
    scope: Option<String>,
    service: Option<String>,
    ipv4: Option<bool>,
    ipv6: Option<bool>,
) -> Option<String> {
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

    // Read the global cache
    let cache = CACHE.clone();

    // Access the GCP cache from the global cache
    if let Some(gcp_data_ref) = cache.get("gcp") {
        // Extract the GCP data
        if let Some(gcp_cache) = gcp_data_ref.downcast_ref::<IntegrationCache<GcpIpRanges>>() {
            // Filter the GCP data based on the provided parameters
            let filtered_data: Vec<&str> = gcp_cache.data.as_ref().map_or_else(Vec::new, |data| {
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
                            // Return the appropriate IP prefix as &str
                            if ipv4.unwrap_or(false) && prefix.ipv4_prefix.is_some() {
                                Some(prefix.ipv4_prefix.as_deref().unwrap())
                            } else if ipv6.unwrap_or(false) && prefix.ipv6_prefix.is_some() {
                                Some(prefix.ipv6_prefix.as_deref().unwrap())
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
                return serde_json::to_string(&filtered_data).ok();
            }
        }
    }
    // Log failure to retrieve GCP data
    error!(
        request_id = %request_id,
        "Failed to retrieve GCP data"
    );
    None
}
