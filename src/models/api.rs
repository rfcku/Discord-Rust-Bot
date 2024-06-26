use reqwest;
use reqwest::Client;
use serde_json::Value;

#[derive(Debug)]
pub struct Header {
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub struct Api {
    url: String,
    pub token: String,
    client: Client,
    pub headers: Vec<Header>,
}


impl Api {
    pub fn new(url: &str, token: &str, headers: Vec<Header> ) -> Self {
        Self {
            url: url.to_string(),
            token: token.to_string(),
            client: Client::new(),
            headers: headers,
        }
    }

    pub async fn _get(&self, url: &str) -> Result<Value, Box<dyn std::error::Error>>{
        let req = self.client.get(format!("{}{}", self.url, url))
        .header("Authorization",  format!("Bearer {}", self.token));
        let res = req.send().await?;
        let text = res.text().await?;
        let json: Value = serde_json::from_str(&text)?;
        Ok(json)
    }  
    
    pub async fn _post(&self, url: &str, payload: &Value ) -> Result<Value, Box<dyn std::error::Error>>{
        let req = self.client.post(format!("{}{}", self.url, url))
        .header("Authorization",  format!("Bearer {}", self.token));
        let res = req.json(&payload).send().await?;
        let text = res.text().await?;
        let json: Value = serde_json::from_str(&text)?;
        Ok(json)
    }
    
}
