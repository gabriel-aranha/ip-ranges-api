use crate::cache::IntegrationCache;
use crate::cache::CACHE;
use crate::integrations::aws::AwsIpRanges;
use rocket::serde::json::serde_json;
use rocket::{get, routes, Route};
use uuid::Uuid;

#[get("/aws?<region>&<service>&<network_border_group>")]
fn query_aws_data(
    region: Option<String>,
    service: Option<String>,
    network_border_group: Option<String>,
) -> Option<String> {
    // Generate a unique request ID
    let request_id = Uuid::new_v4();

    // Log the start of the request with structured fields for received parameters
    tracing::info!(
        request_id = %request_id,
        region = region.clone().unwrap_or_else(|| "None".to_string()),
        service = service.clone().unwrap_or_else(|| "None".to_string()),
        network_border_group = network_border_group.clone().unwrap_or_else(|| "None".to_string()),
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
                        // Check if the data matches the provided parameters
                        let matches = region
                            .as_deref()
                            .map_or(true, |param| prefix.region == param)
                            && service
                                .as_deref()
                                .map_or(true, |param| prefix.service == param)
                            && network_border_group
                                .as_deref()
                                .map_or(true, |param| prefix.network_border_group == param);

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
                tracing::info!(
                    request_id = %request_id,
                    "AWS data found for request"
                );
                return serde_json::to_string(&filtered_data).ok();
            }
        }
    }
    // Log failure to retrieve AWS data
    tracing::error!(
        request_id = %request_id,
        "Failed to retrieve AWS data"
    );
    None
}

pub fn routes() -> Vec<Route> {
    routes![query_aws_data]
}
