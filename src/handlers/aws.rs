use crate::cache::IntegrationCache;
use crate::cache::CACHE;
use crate::fetchers::aws::AwsIpRanges;
use rocket::get;
use rocket::serde::json::serde_json;
use tracing::{error, info};
use uuid::Uuid;

#[get("/v1/aws?<region>&<service>&<network_border_group>")]
pub fn query_aws_data(
    region: Option<String>,
    service: Option<String>,
    network_border_group: Option<String>,
) -> Option<String> {
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
            let filtered_data: Vec<&str> = aws_cache.data.as_ref().map_or_else(Vec::new, |data| {
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
                            // Return the IP prefix as &str
                            Some(prefix.ip_prefix.as_str())
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
                    "AWS data found for request"
                );
                return serde_json::to_string(&filtered_data).ok();
            }
        }
    }
    // Log failure to retrieve AWS data
    error!(
        request_id = %request_id,
        "Failed to retrieve AWS data"
    );
    None
}
