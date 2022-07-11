use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KlinesRequest {
    pub ticker: String,
    pub interval: String,
    pub samples: u64,
}