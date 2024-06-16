use crate::cache::IntegrationCache;
use crate::cache::CACHE;
use crate::fetchers::azure::AzureIpRanges;
use rocket::get;
use rocket::serde::json::serde_json;
use tracing::{error, info};
use uuid::Uuid;

#[get("/v1/azure?<region>&<system_service>&<ipv4>&<ipv6>")]
pub fn query_azure_data(
    region: Option<String>,
    system_service: Option<String>,
    ipv4: Option<bool>,
    ipv6: Option<bool>,
) -> Option<serde_json::Value> {
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

    // Read the global cache
    let cache = CACHE.clone();

    // Access the Azure cache from the global cache
    if let Some(azure_data_ref) = cache.get("azure") {
        // Extract the Azure data
        if let Some(azure_cache) = azure_data_ref.downcast_ref::<IntegrationCache<AzureIpRanges>>()
        {
            // Filter the Azure data based on the provided parameters
            let mut filtered_data: Vec<String> = azure_cache.data.as_ref().map_or_else(Vec::new, |data| {
                let param_region = region.clone().map(|s| s.to_lowercase());
                let param_system_service = system_service.clone().map(|s| s.to_lowercase());

                data.values.iter().filter_map(|value| {
                    let matches = param_region.as_deref().map_or(true, |param| {
                        value.properties.region.to_lowercase() == param
                    }) && param_system_service.as_deref().map_or(true, |param| {
                        value.properties.system_service.to_lowercase() == param
                    });

                    if matches {
                        Some(value.properties.address_prefixes.clone())
                    } else {
                        None
                    }
                }).flatten().collect()
            });

            let ipv4_flag = ipv4.unwrap_or(false);
            let ipv6_flag = ipv6.unwrap_or(false);

            // Further filter based on IPv4 and IPv6 flags
            if !ipv4_flag && !ipv6_flag {
                return None;
            }

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
                return Some(serde_json::json!(filtered_data));
            }
        }
    }

    // Log failure to retrieve Azure data
    error!(
        request_id = %request_id,
        "Failed to retrieve Azure data"
    );
    None
}

fn is_ipv4(prefix: &str) -> bool {
    prefix.contains('.')
}
