use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Write;
use uuid::Uuid;

#[derive(serde::Serialize, Deserialize, Debug)]
struct ProductEvent {
    ip_address: String,
    anonymous_id: String,
    timestamp: String,
    method: String,
    login_id: String,
    product_id: i32,
    location: Location,
    user_agent: String,
    event_name: String,
    domain: String,
    url: String,
    referrer: String,
    tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Location {
    latitude: i32,
    longditude: i32,
}

fn generate_product_event() -> ProductEvent {
    let mut tags = HashMap::new();
    tags.insert(String::from("experiment-a"), String::from("control"));
    tags.insert(String::from("experiment-b"), String::from("b"));
    tags.insert(String::from("site_version"), String::from("921103db"));

    let anonymous_id = Uuid::new_v4();
    let product_event = ProductEvent {
        ip_address: String::from("127.0.0.1"), 
        anonymous_id: anonymous_id.to_string(),
        timestamp: String::from("2024-07-29T09:56:23Z"),
        method: String::from("GET"),
        login_id: String::from("user_995a12f6-a8f1-4cfb-8c9e-d2e6103dc41f"),
        product_id: 10,
        location: Location{
            latitude: 180,
            longditude: 37,
        },
        user_agent: String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36"),
        event_name: String::from("product_viewed"),
        domain: String::from("www.example.com"),
        url: String::from("https://www.example.com/products/10"),
        referrer: String::from("https://www.example.com/"),
        tags,
    };
    product_event
}

pub fn write_file(path: &str, item_count: u32) -> io::Result<()> {
    let mut output = File::create(path)?;
    for _ in 0..item_count {
        let product_event = generate_product_event();
        let serialized = serde_json::to_string(&product_event).unwrap();
        writeln!(output, "{}", serialized)?;
    }
    Ok(())
}
