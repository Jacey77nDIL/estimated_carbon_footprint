use axum::{
    extract::Json,
    http::StatusCode,
    routing::post,
    Router,
};
use csv::Error as CsvError;
use csv::ReaderBuilder;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::json;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use reqwest::Client;
use std::env;

#[derive(Deserialize, Debug)]
struct DeviceData {
    screen_width: u32,
    screen_height: u32,
    user_device: String,
    is_ios: bool,
    is_laptop: bool,
}

#[derive(Serialize, Debug)]
struct IphoneModelResponse {
    iphone_model: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct DeviceResponse {
    battery_capacity: String,
}

#[derive(Deserialize, Debug)]
struct BatteryQuery {
    user_device: String,
}

async fn get_ios_options(Json(data): Json<DeviceData>) -> (StatusCode, Json<IphoneModelResponse>) {
    let iphone_models = vec![
        ("iPhone 16 Pro Max".to_string(), (440, 956)),
        ("iPhone 16 Pro".to_string(), (402, 874)),
        ("iPhone 16 Plus".to_string(), (430, 932)),
        ("iPhone 16".to_string(), (393, 852)),
        ("iPhone 15 Pro Max".to_string(), (430, 932)),
        ("iPhone 15 Plus".to_string(), (430, 932)),
        ("iPhone 14 Pro Max".to_string(), (430, 932)),
        ("iPhone 14 Plus".to_string(), (428, 926)),
        ("iPhone 13 Pro Max".to_string(), (428, 926)),
        ("iPhone 12 Pro Max".to_string(), (428, 926)),
        ("iPhone 11 Pro Max".to_string(), (414, 896)),
        ("iPhone XS Max".to_string(), (414, 896)),
        ("iPhone 15 Pro".to_string(), (393, 852)),
        ("iPhone 15".to_string(), (393, 852)),
        ("iPhone 14 Pro".to_string(), (393, 852)),
        ("iPhone 14".to_string(), (390, 844)),
        ("iPhone 13".to_string(), (390, 844)),
        ("iPhone 13 Pro".to_string(), (390, 844)),
        ("iPhone 12".to_string(), (390, 844)),
        ("iPhone 12 Pro".to_string(), (390, 844)),
        ("iPhone 11 Pro".to_string(), (375, 812)),
        ("iPhone XS".to_string(), (375, 812)),
        ("iPhone X".to_string(), (375, 812)),
        ("iPhone 13 mini".to_string(), (375, 812)),
        ("iPhone 12 mini".to_string(), (375, 812)),
        ("iPhone 8 Plus".to_string(), (414, 736)),
        ("iPhone 7 Plus".to_string(), (414, 736)),
        ("iPhone 6S Plus".to_string(), (414, 736)),
        ("iPhone 6 Plus".to_string(), (414, 736)),
        ("iPhone 11".to_string(), (414, 896)),
        ("iPhone XR".to_string(), (414, 896)),
        ("iPhone SE 3rd gen".to_string(), (375, 667)),
        ("iPhone SE 2nd gen".to_string(), (375, 667)),
        ("iPhone 8".to_string(), (375, 667)),
        ("iPhone 7".to_string(), (375, 667)),
        ("iPhone 6S".to_string(), (375, 667)),
        ("iPhone 6".to_string(), (375, 667)),
        ("iPhone SE 1st gen".to_string(), (320, 568)),
        ("iPhone 5C".to_string(), (320, 568)),
        ("iPhone 5S".to_string(), (320, 568)),
        ("iPhone 5".to_string(), (320, 568)),
        ("iPhone 4S".to_string(), (320, 480)),
        ("iPhone 4".to_string(), (320, 480)),
        ("iPhone 3GS".to_string(), (320, 480)),
        ("iPhone 3G".to_string(), (320, 480)),
        ("iPhone".to_string(), (320, 480)),
    ];

    // Print the received resolution for debugging
    println!("Received resolution: {}x{}", data.screen_width, data.screen_height);

    let target_resolution = (data.screen_width, data.screen_height);
    let mut matching_models = Vec::new();

    // Iterate through the models and add the ones that match the resolution
    for (model, resolution) in iphone_models.iter() {
        if *resolution == target_resolution {
            matching_models.push(model.clone());
        }
    }

    // Respond with the matching models
    if matching_models.is_empty() {
        matching_models.push("No matching iPhone model found".to_string());
    }

    (StatusCode::OK, Json(IphoneModelResponse { iphone_model: matching_models }))
}

async fn process_device(Json(data): Json<DeviceData>) -> (StatusCode, Json<String>) {
    println!("Received data: {:?}", data);

    let response = format!(
        "Processed data for device: {}, Screen: {}x{}",
        data.user_device, data.screen_width, data.screen_height
    );

    (StatusCode::OK, Json(response))
}

async fn process_samsung_devices(Json(mut data): Json<DeviceData>) -> (StatusCode, Json<String>) {
    // Extended mapping of Samsung device builds to popular names
    let samsung_mapping: HashMap<&str, &str> = HashMap::from([
        ("SM-G900P", "Galaxy S5"),
        ("SM-G950F", "Galaxy S8"),
        ("SM-G950U", "Galaxy S8"),
        ("SM-G955F", "Galaxy S8+"),
        ("SM-G955U", "Galaxy S8+"),
        ("SM-G960F", "Galaxy S9"),
        ("SM-G960U", "Galaxy S9"),
        ("SM-G965F", "Galaxy S9+"),
        ("SM-G965U", "Galaxy S9+"),
        ("SM-G970F", "Galaxy S10e"),
        ("SM-G970U", "Galaxy S10e"),
        ("SM-G973F", "Galaxy S10"),
        ("SM-G973U", "Galaxy S10"),
        ("SM-G975F", "Galaxy S10+"),
        ("SM-G975U", "Galaxy S10+"),
        ("SM-G980F", "Galaxy S20"),
        ("SM-G980U", "Galaxy S20"),
        ("SM-G985F", "Galaxy S20+"),
        ("SM-G985U", "Galaxy S20+"),
        ("SM-G986F", "Galaxy S20+ 5G"),
        ("SM-G986U", "Galaxy S20+ 5G"),
        ("SM-G988F", "Galaxy S20 Ultra"),
        ("SM-G988U", "Galaxy S20 Ultra"),
        ("SM-G981B", "Galaxy S20 Ultra"),
        ("SM-G991F", "Galaxy S21"),
        ("SM-G991U", "Galaxy S21"),
        ("SM-G996F", "Galaxy S21+"),
        ("SM-G996U", "Galaxy S21+"),
        ("SM-G998F", "Galaxy S21 Ultra"),
        ("SM-G998U", "Galaxy S21 Ultra"),
        ("SM-A520F", "Galaxy A5 (2017)"),
        ("SM-A520U", "Galaxy A5 (2017)"),
        ("SM-A705F", "Galaxy A70"),
        ("SM-A705U", "Galaxy A70"),
        ("SM-A715F", "Galaxy A71"),
        ("SM-A715U", "Galaxy A71"),
        ("SM-N950F", "Galaxy Note 8"),
        ("SM-N950U", "Galaxy Note 8"),
        ("SM-N960F", "Galaxy Note 9"),
        ("SM-N960U", "Galaxy Note 9"),
        ("SM-N970F", "Galaxy Note 10"),
        ("SM-N970U", "Galaxy Note 10"),
        ("SM-N975F", "Galaxy Note 10+"),
        ("SM-N975U", "Galaxy Note 10+"),
        ("SM-N980F", "Galaxy Note 20"),
        ("SM-N980U", "Galaxy Note 20"),
        ("SM-N986F", "Galaxy Note 20 Ultra"),
        ("SM-N986U", "Galaxy Note 20 Ultra"),
        ("SM-F900F", "Galaxy Fold"),
        ("SM-F900U", "Galaxy Fold"),
        ("SM-F700F", "Galaxy Z Flip"),
        ("SM-F700U", "Galaxy Z Flip"),
        ("SM-F707B", "Galaxy Z Flip 5G"),
        ("SM-F707U", "Galaxy Z Flip 5G"),
        ("SM-F926B", "Galaxy Z Fold 3"),
        ("SM-F926U", "Galaxy Z Fold 3"),
        ("SM-F936B", "Galaxy Z Fold 4"),
        ("SM-F936U", "Galaxy Z Fold 4"),
        ("SM-F731B", "Galaxy Z Flip 5"),
        ("SM-F731U", "Galaxy Z Flip 5"),
        ("SM-S901B", "Galaxy S22"),
        ("SM-S901U", "Galaxy S22"),
        ("SM-S906B", "Galaxy S22+"),
        ("SM-S906U", "Galaxy S22+"),
        ("SM-S908B", "Galaxy S22 Ultra"),
        ("SM-S908U", "Galaxy S22 Ultra"),
        ("SM-S911B", "Galaxy S23"),
        ("SM-S911U", "Galaxy S23"),
        ("SM-S916B", "Galaxy S23+"),
        ("SM-S916U", "Galaxy S23+"),
        ("SM-S918B", "Galaxy S23 Ultra"),
        ("SM-S918U", "Galaxy S23 Ultra"),
        // Tab Series
        ("SM-T837A", "Galaxy Tab S4"),
        ("SM-T870", "Galaxy Tab S7"),
        ("SM-T875", "Galaxy Tab S7+"),
        ("SM-T970", "Galaxy Tab S7 FE"),
        ("SM-T820", "Galaxy Tab S3"),
        ("SM-T700", "Galaxy Tab S2"),
        ("SM-T805", "Galaxy Tab S2 8.0"),
        ("SM-T350", "Galaxy Tab A 8.0"),
        ("SM-T510", "Galaxy Tab A 10.1"),
        ("SM-T395", "Galaxy Tab A 8.0 (2017)"),
        // Additional A-series models
        ("SM-A125F", "Galaxy A12"),
        ("SM-A125U", "Galaxy A12"),
        ("SM-A135F", "Galaxy A13"),
        ("SM-A135U", "Galaxy A13"),
        ("SM-A145F", "Galaxy A14"),
        ("SM-A145U", "Galaxy A14"),
        // Add additional models as needed
        ]);
    
        // Extract the build type from the user device string
        let build_type = data
            .user_device
            .split_whitespace()
            .next()
            .unwrap_or_default();
    
        // Map the build type to a popular name
        let device_name = samsung_mapping
            .get(build_type)
            .unwrap_or(&data.user_device.as_str())
            .to_string();

        println!("Resolved device: {}", device_name);
        data.user_device = device_name.clone();
        println!("Received data: {:?}", data);
        (StatusCode::OK, Json(device_name))    
}

async fn get_device_battery(Json(data): Json<DeviceData>) -> (StatusCode, Json<String>) {
    let mut rdr = match ReaderBuilder::new().has_headers(true).from_path("C:/Users/ikemn/Documents/PROGRAMMING/estimated_carbon_footprint/smartprix_final_manipulated.csv") {
        Ok(rdr) => rdr,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to read CSV file".to_string()),
            );
        }
    };

    for result in rdr.records() {
        match result {
            Ok(record) => {
                let device_name_in_csv = record.get(1).unwrap_or_default().to_string();
                let battery_capacity = record.get(0).unwrap_or_default().to_string();
                println!("Device name: {:?}, Device Name in CSV: {:?}", data.user_device, device_name_in_csv);
                if device_name_in_csv == data.user_device {
                    return (
                        StatusCode::OK,
                        Json(battery_capacity),
                    );
                }
            }
            Err(_) => continue, // Continue to next record in case of error reading
        }
    }

    // If no match is found
    (
        StatusCode::NOT_FOUND,
        Json("Device not found".to_string()),
    )
}

// Function to get battery capacity using GPT-4o-mini API
async fn get_battery_capacity_for_laptops(Json(data): Json<BatteryQuery>) -> (StatusCode, Json<String>) {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let client = Client::new();
    
    let query = format!("What is the battery capacity of the {}? Provide only the highest capacity in watt-hour (Wh) and nothing else (Only the number don't add the unit). If the device given isn't real just return Invalid and nothing else", data.user_device);

    let body = json!({
        "model": "gpt-4o-mini",
        "messages": [
            {"role": "system", "content": "You are an expert in providing technical specifications."},
            {"role": "user", "content": query}
        ],
        "max_tokens": 3, // Limit to short numerical responses
        "temperature": 0.2 // Lower temperature for accuracy
    });

    match client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                if let Ok(json_response) = response.json::<serde_json::Value>().await {
                    if let Some(answer) = json_response["choices"][0]["message"]["content"].as_str() {
                        return (StatusCode::OK, Json(answer.to_string()));
                    }
                }
            }
            (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to get battery capacity".to_string()))
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Request to OpenAI API failed".to_string())),
    }
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/device", post(process_device))
        .route("/ios-options", post(get_ios_options)) // Add a route for the new functionality
        .route("/samsung_devices", post(process_samsung_devices))
        .route("/get_device_battery", post(get_device_battery))
        .route("/get_battery_capacity_for_laptops", post(get_battery_capacity_for_laptops))
        .layer(
            CorsLayer::new()
                .allow_origin("https://verdant-melba-b66cfd.netlify.app") // Allow requests from my site
                .allow_methods(Any) // Allow any HTTP method
                .allow_headers(Any), // Allow any headers
        );

    // Get the PORT from the environment variable, defaulting to 10000 if not set
    let port = env::var("PORT")
        .unwrap_or_else(|_| "10000".to_string())  // Default Render port
        .parse::<u16>()
        .expect("PORT must be a valid number");

    // Bind to 0.0.0.0 to be accessible externally on Render
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
