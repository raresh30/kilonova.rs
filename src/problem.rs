use crate::{
    error::{APIError, Error},
    Response,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ProblemResponse {
    pub status: String,
    pub data: Problem,
}

#[derive(Deserialize, Debug)]
pub struct Problem {
    pub id: i32,
    pub created_at: String,
    pub name: String,
    pub test_name: String,
    pub default_points: i32,
    pub visible: bool,
    pub visible_tests: bool,
    pub time_limit: f32,
    pub memory_limit: i32,
    pub source_size: i32,
    pub source_credits: String,
    pub score_scale: i32,
    pub console_input: bool,
    pub score_precision: i32,
    pub published_at: String,
    pub scoring_strategy: String,
}

impl Problem {
    pub async fn from_id(id: i32) -> Result<Problem, Error> {
        let url = format!("https://kilonova.ro/api/problem/{}", id);
        let resp = Response::build(&url).await?;
        match resp {
            Response::ProblemSuccess(prob_resp) => Ok(prob_resp.data),
            Response::Error(err_resp) => Err(Box::new(APIError {
                error: err_resp.error,
            })),
            _ => panic!("Incorrect API call"),
        }
    }
}
