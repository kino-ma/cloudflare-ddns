use std::{error::Error, fs::File};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateConfigs {
    pub token: String,
    pub key: String,
    pub zone_identifier: String,
    pub ttl: Option<String>,
    pub proxied: Option<String>,
}

#[derive(Debug)]
pub struct Params {
    pub name: String,
    pub content: String,
}

impl UpdateConfigs {
    pub fn read_yaml(file: &str) -> Result<Self, Box<dyn Error>> {
        let f = File::open(file)?;
        Ok(serde_yaml::from_reader(f)?)
    }
}
