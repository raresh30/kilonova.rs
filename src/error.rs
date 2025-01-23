use serde::Deserialize;
use std::fmt;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub struct APIError {
    pub error: String,
}

impl std::error::Error for APIError {}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    #[serde(rename = "data")]
    pub error: String,
}
