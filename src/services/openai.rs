use crate::models::api::{Api, Header};
use dotenv::dotenv;
use std::env;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::time::{sleep, Duration};


#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug)]
pub struct OpenAi {
    pub api: Api,
    pub model: String,
}

#[allow(dead_code)]
impl OpenAi {
    pub fn new() -> Self {
        dotenv().ok();
        Self {
            api: Api::new(
                &env::var("OPENAI_API").expect("OPENAI_API must be set."),
                &env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set."),
                vec![Header{
                    key: "OpenAI-Beta".to_string(),
                    value: "assistants=v1".to_string()
                }],
            ),
            model: env::var("OPENAI_MODEL").expect("OPENAI_MODEL must be set."),
        }
    }
    pub async fn create_completion(&self, text:&str, mut history: Vec<Value> ) -> Result<String, Box<dyn std::error::Error>> {
        history.push(serde_json::json!({
                "role": "user".to_string(),
                "content": text.to_string()
            })
        );
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
    
    pub async fn create_assistant(&self, name:&str) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.api._post("assistants", &serde_json::json!({
            "name": name,
            "language": "es"
        })).await;
        if response.is_err() {
            return Err(response.err().unwrap());
        }
        let response = response.unwrap();
        let assistant_id = response["id"].to_string().replace("\"", "");
        Ok(assistant_id)
    }

    pub async fn ask_assistant(&self, message: &str, thread_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        
        self.add_message(message, thread_id).await?;
        
        let run = self.create_run(&thread_id).await?;
        let run_id = run.id;

        let mut run_status = self.check_run_status(&run_id, &thread_id).await?;
        while run_status != "completed".to_string() {
            sleep(Duration::from_secs(1)).await; // Add a delay between checks
            run_status = self.check_run_status(&run_id, &thread_id).await?;
        }

        let messages = self.get_thread_messages(&thread_id).await?;
        
        match messages.first() {
            Some(message) => {
                let content = &message["content"];
                let last_message = content.as_array().unwrap().first().unwrap();
                let last_message_content = last_message["text"]["value"].to_string().replace("\"", "").replace("¿Hay algo más en lo que pueda ayudarte?", "");
                Ok(last_message_content)
            },
            None => Err("No messages found in the thread".into()), // Convert the error into Box<dyn Error>
        }
    }

    pub async fn create_thread(&self) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.api._post("threads", &serde_json::json!({})).await;
        if response.is_err() {
            return Err(response.err().unwrap());
        }
        let response = response.unwrap();
        let thread_id = response["id"].to_string().replace("\"", "");
        Ok(thread_id)
    }

    pub async fn add_message(&self, message: &str, thread_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.api._post(format!("threads/{}/messages", thread_id).as_str(), &serde_json::json!({
            "role": "user",
            "content": message
        })).await;
        if response.is_err() {
            return Err(response.err().unwrap());
        }
        let response = response.unwrap();
        Ok(response.to_string())
    }

    pub async fn create_run(&self,  thread_id: &str) -> Result<Run,  Box<dyn std::error::Error>> {
        let url = format!("threads/{}/runs", thread_id.to_string());
        let response = self.api._post( &url , &serde_json::json!({ "assistant_id": "asst_nELXBaiWQZC4qHqbwVOIAK7z" })).await;
        if response.is_err() {
            return Err(response.err().unwrap());
        }
        let response = response.unwrap();
        let run = Run {
            id: response["id"].to_string().replace("\"", ""),
        };
        Ok(run)
    }

    pub async fn check_run_status(&self, run_id: &str,  thread_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("threads/{}/runs/{}", thread_id.to_string(), run_id.to_string());
        let response = self.api._get(&url ).await ;
        if response.is_err() {
            return Err(response.err().unwrap());
        }
        let response = response.unwrap();
        let status = &response["status"];
        Ok(status.to_string().replace("\"", ""))
    }

    pub async fn get_thread_messages(&self,  thread_id: &str) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        let response = self.api._get(format!("threads/{}/messages", thread_id.to_string()).as_str()).await;
        if response.is_err() {
            return Err(response.err().unwrap());
        }
        let response = response.unwrap();
        let messages = response["data"].as_array().unwrap().to_vec();
        Ok(messages)
    }
}


#[derive(Debug)]
pub struct Run {
    pub id: String,
}