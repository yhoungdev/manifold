use crate::types::{KudiClient, KudiMessage, SenderIdStruct, SmsOtpPayload};

use reqwest::Client;

impl KudiClient {
    pub fn new() -> Self {
        Self { token, sender_id }
    }


    pub fn send_sms_otp(&self, payload: SmsOtpPayload) -> Result<(), Box<dyn std::error::Error>> {
        let api_url = format!(
            "https://my.kudisms.net/api/otp?token={}&senderID={}&recipients={}&otp={}&appnamecode={}&templatecode={}",
            payload.token,
            payload.sender_id,
            payload.recipients,
            payload.otp,
            payload.app_name_code,
            payload.template_code
        );
        let client = Client::new();
        let response = client.post(&api_url).send()?;

        let response_text = response.text()?;

        Ok(response_text);
    }


    pub fn submit_sender_id(&self, payload: SenderIdStruct) -> Result<(), Box<dyn std::error::Error>> {
        let api_url = "https://my.kudisms.net/api/senderID";
        let client = Client::new();

        let response = client
            .post(api_url)
            .json(&payload)
            .send()?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Request failed with status: {}", response.status()).into())
        }
    }
}
