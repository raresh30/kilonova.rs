use crate::error::Error;
use serde::Deserialize;

pub mod error;
pub mod problem;
pub mod submissions;
pub mod user;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Response {
    UserSuccess(crate::user::UserResponse),
    ProblemSuccess(crate::problem::ProblemResponse),
    SubmissionsSuccess(crate::submissions::SubmissionsResponse),
    Error(crate::error::ErrorResponse),
}

impl Response {
    pub async fn build(url: &str) -> Result<Response, Error> {
        let resp = reqwest::get(url).await?.json::<Response>().await?;
        Ok(resp)
    }
}
