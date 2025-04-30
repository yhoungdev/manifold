use common_manifold::types::plunk_types::PlunkClient;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlunkPayloads {
    pub to: String,
    pub subject: Option<String>,
    pub body: String,
}

const PLUNK_API: &str = "https://api.useplunk.com/v1/send";

pub trait PlunkClientTrait {
    fn new(public_api_key: String) -> Self;

    async fn send_transactional_email(
        &self,
        payload: PlunkPayloads,
    ) -> Result<String, Box<dyn std::error::Error>>;
}

impl PlunkClientTrait for PlunkClient {
    fn new(public_api_key: String) -> Self {
        PlunkClient { public_api_key }
    }

    async fn send_transactional_email(
        &self,
        payload: PlunkPayloads,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let client = Client::new();
        let response = client
            .post(PLUNK_API)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.public_api_key))
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            Ok("Email sent successfully".to_string())
        } else {
            Err(format!("Failed to send email: {}", response.status()).into())
        }
    }
}
