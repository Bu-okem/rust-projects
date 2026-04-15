use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct ServiceEntry {
    username: String,
    website: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Vault {
    map: HashMap<String, ServiceEntry>,
}

fn main() {
    println!("Hello, world!");
}
