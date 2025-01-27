use crate::{
    error::{APIError, Error},
    Response,
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct SubmissionsResponse {
    pub status: String,
    pub data: SubmissionsData,
}

#[derive(Deserialize, Debug)]
pub struct SubmissionsData {
    pub submissions: Vec<Submission>,
    pub count: i32,
    pub truncated_count: bool,
    pub users: HashMap<String, crate::user::User>,
    pub problems: HashMap<String, crate::problem::Problem>,
}

#[derive(Deserialize, Debug)]
pub struct Submission {
    pub id: i32,
    pub created_at: String,
    pub user_id: i32,
    pub problem_id: i32,
    pub language: String,
    pub code_size: i32,
    pub status: String,
    pub compile_error: bool,
    pub contest_id: Option<i32>,
    pub max_time: f32,
    pub max_memory: i32,
    pub score: i32,
    pub score_precision: i32,
    pub score_scale: i32,
    pub compile_time: Option<f32>,
    pub submission_type: String,
    pub icpc_verdict: Option<String>,
}

pub struct SubmissionFilter {
    pub submission_id: Option<i32>,
    pub user_id: Option<i32>,
    pub problem_id: Option<i32>,
}

impl Default for SubmissionFilter {
    fn default() -> SubmissionFilter {
        SubmissionFilter {
            submission_id: None,
            user_id: None,
            problem_id: None,
        }
    }
}

pub async fn fetch(filter: SubmissionFilter) -> Result<SubmissionsData, Error> {
    let mut url = String::from("https://kilonova.ro/api/submissions/get?");
    if let Some(id) = filter.submission_id {
        url.push_str(format!("&id={}", id).as_str());
    }
    if let Some(user_id) = filter.user_id {
        url.push_str(format!("&user_id={}", user_id).as_str());
    }
    if let Some(problem_id) = filter.problem_id {
        url.push_str(format!("&problem_id={}", problem_id).as_str());
    }
    let resp = Response::build(&url).await?;
    match resp {
        Response::SubmissionsSuccess(sub_resp) => Ok(sub_resp.data),
        Response::Error(err_resp) => Err(Box::new(APIError {
            error: err_resp.error,
        })),
        _ => panic!("Incorrect API call"),
    }
}
