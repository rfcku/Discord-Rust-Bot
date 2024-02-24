use crate::models::api::Api;
use dotenv::dotenv;
use std::env;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

pub struct OpenAi {
    pub api: Api,
    pub model: String
}
impl OpenAi {
    pub fn new() -> Self {
        dotenv().ok();
        Self {
            api: Api::new(
                &env::var("OPENAI_API").expect("OPENAI_API must be set."),
                &env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set.")
            ),
            model: env::var("OPENAI_MODEL").expect("OPENAI_MODEL must be set.")
        }
    }

    pub async fn create_completion(&self, text:&str, mut history: Vec<Value> ) -> Result<String, Box<dyn std::error::Error>> {
        history.push(serde_json::json!({
            "role": "user".to_string(),
            "content": text.to_string()
        }));

        let response = self.api._post("chat/completions", &serde_json::json!({
            "model": self.model,
            "messages": history
        }) ).await ;

        if response.is_err() {
            return Err(response.err().unwrap());
        }

        let response = response.unwrap();
        let response_text = response["choices"][0]["message"]["content"].to_string().replace("\"", "");
        Ok(response_text)
    }
}
