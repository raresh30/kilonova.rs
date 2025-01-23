use crate::{
    error::{APIError, Error},
    Response,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: User,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub admin: bool,
    pub proposer: bool,
    pub display_name: String,
    pub generated: bool,
}

impl User {
    pub async fn by_name(name: &str) -> Result<User, Error> {
        let url = format!("https://kilonova.ro/api/user/byName/{}", name);
        let resp = Response::build(&url).await?;
        match resp {
            Response::UserSuccess(user_resp) => Ok(user_resp.data),
            Response::Error(err_resp) => Err(Box::new(APIError {
                error: err_resp.error,
            })),
            _ => panic!("Incorrect API call"),
        }
    }

    pub async fn by_id(id: i32) -> Result<User, Error> {
        let url = format!("https://kilonova.ro/api/user/byID/{}", id);
        let resp = Response::build(&url).await?;
        match resp {
            Response::UserSuccess(user_resp) => Ok(user_resp.data),
            Response::Error(err_resp) => Err(Box::new(APIError {
                error: err_resp.error,
            })),
            _ => panic!("Incorrect API call"),
        }
    }
}
