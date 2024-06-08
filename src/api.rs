use rocket::{get, routes, State, Route};
use std::sync::Mutex;
use crate::integrations::aws::AwsIpRanges;
use rocket::serde::json::serde_json;

#[get("/aws?<region>&<service>&<network_border_group>")]
fn query_aws_data(region: Option<String>, service: Option<String>, network_border_group: Option<String>, aws_cache: &State<Mutex<Vec<String>>>) -> Option<String> {
    // Lock the cache for reading
    let cache = aws_cache.lock().unwrap();

    // Filter the cached data based on the provided parameters
    let filtered_data: Vec<&str> = cache.iter().filter_map(|data| {
        let aws_data: AwsIpRanges = serde_json::from_str(data).unwrap(); // Deserialize cached data
        
        // Check if the data matches the provided parameters
        let matches = region.as_ref().map_or(true, |param| aws_data.prefixes.iter().any(|prefix| &prefix.region == param))
            && service.as_ref().map_or(true, |param| aws_data.prefixes.iter().any(|prefix| &prefix.service == param))
            && network_border_group.as_ref().map_or(true, |param| aws_data.prefixes.iter().any(|prefix| &prefix.network_border_group == param));
        
        if matches {
            Some(data.as_str()) // Convert String to &str
        } else {
            None
        }
    }).collect();

    // Return filtered data as JSON string
    serde_json::to_string(&filtered_data).ok()
}

pub fn routes() -> Vec<Route> {
    routes![
        query_aws_data,
    ]
}
