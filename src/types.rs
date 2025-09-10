use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Service {
    #[serde(rename = "type")]
    pub r#type: String,
    pub endpoint: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Web5DocumentData {
    #[serde(rename = "verificationMethods")]
    pub verification_methods: BTreeMap<String, String>,
    #[serde(rename = "alsoKnownAs")]
    pub also_known_as: Vec<String>,
    pub services: BTreeMap<String, Service>,
}
