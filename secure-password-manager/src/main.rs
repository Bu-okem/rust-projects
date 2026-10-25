use argon2::Argon2;
use rand::fill;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::{collections::HashMap, error::Error};

#[derive(Serialize, Deserialize)]
struct ServiceEntry {
    username: String,
    website: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Vault {
    map: HashMap<String, ServiceEntry>,
    salt: Vec<u8>,
}

impl Vault {
    fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::create(path)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
        let argon2 = Argon2::default();
        let mut output_bytes = [0u8; 32];
        argon2
            .hash_password_into(password.as_bytes(), &salt, &mut output_bytes)
            .unwrap();

        output_bytes
    }

    fn generate_salt() -> [u8; 16] {
        let mut salt_arr = [0u8; 16];
        fill(&mut salt_arr);
        salt_arr
    }
}

fn main() {
    println!("Main");
}
