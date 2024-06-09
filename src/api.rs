use rocket::{get, routes, Route};
use crate::integrations::aws::AwsIpRanges;
use rocket::serde::json::serde_json;

use crate::cache::CACHE;


#[get("/aws?<region>&<service>&<network_border_group>")]
fn query_aws_data(
    region: Option<String>, 
    service: Option<String>, 
    network_border_group: Option<String>
) -> Option<String> {
    // Log the incoming query parameters
    println!("Received query with region: {:?}, service: {:?}, network_border_group: {:?}", region, service, network_border_group);
    
    // Read the global cache
    let cache = CACHE.clone();

    // Access the AWS cache from the global cache
    if let Some(aws_data_box) = cache.get("aws") {
        // Log the data type and debug information of aws_data_any
        println!("Data found in cache for 'aws': {:?}", aws_data_box.type_id());
        println!("Data debug: {:?}", aws_data_box);

        // Get the value stored inside the Ref
        let aws_data = aws_data_box.value();

        // Log the type of the actual data
        println!("Type of actual data: {:?}", std::any::type_name_of_val(&aws_data));
        
        // Print out the actual data to inspect its structure and contents
        println!("Actual data: {:?}", aws_data);
        
        if let Some(aws_data) = aws_data.downcast_ref::<Box<AwsIpRanges>>() {
            // Dereference the Box to access the inner AwsIpRanges
            let aws_data_inner = aws_data.as_ref();

            // Log the aws_data content
            println!("AWS data: {:?}", aws_data_inner);
            
            // Filter the cached data based on the provided parameters
            let filtered_data: Vec<&str> = aws_data_inner.prefixes.iter().filter_map(|prefix| {
                // Check if the data matches the provided parameters
                let matches = region.as_ref().map_or(true, |param| aws_data.prefixes.iter().any(|prefix| &prefix.region == param))
                    && service.as_ref().map_or(true, |param| aws_data.prefixes.iter().any(|prefix| &prefix.service == param))
                    && network_border_group.as_ref().map_or(true, |param| aws_data.prefixes.iter().any(|prefix| &prefix.network_border_group == param));

                println!("Entry matches: {}", matches);

                if matches {
                    Some(prefix.ip_prefix.as_str()) // Convert String to &str
                } else {
                    None
                }
            }).collect();

            // Log the filtered data
            println!("Filtered data: {:?}", filtered_data);

            // Return filtered data as JSON string
            return serde_json::to_string(&filtered_data).ok();
        } else {
            println!("Data found in cache for 'aws' could not be downcast to Vec<String>");
        }
    } else {
        println!("No data found in cache for 'aws'");
    }

    None
}


pub fn routes() -> Vec<Route> {
    routes![
        query_aws_data,
    ]
}
