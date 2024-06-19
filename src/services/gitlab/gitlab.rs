use crate::models::api::Api;
use dotenv::dotenv;
use std::env;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::time::{sleep, Duration};


#[derive(Debug)]
pub struct GitlabApi {
    pub api: Api,
}
#[allow(dead_code)]
impl GitlabApi {
    pub fn new() -> Self {
        dotenv().ok();
        Self {
            api: Api::new(
                &env::var("GITLAB_API").expect("GITLAB_API must be set."),
                &env::var("GITLAB_API_KEY").expect("GITLAB_API_KEY must be set.")
            ),
        }
    }
}