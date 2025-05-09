use crate::types::{
    KudiClient, SendBulkSmsPayload, SenderIdCheckResponse, SenderIdStruct, SmsOtpPayload,
};

use crate::prelude::SendCorporateEmail;
use reqwest::Client;

impl KudiClient {
    pub fn new(token: String, sender_id: String) -> Self {
        Self { token, sender_id }
    }

    pub async fn send_sms_otp(
        &self,
        payload: SmsOtpPayload,
    ) -> Result<String, Box<dyn std::error::Error>> {
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
        let response = client.post(&api_url).send().await?;
        let response_text = response.text().await?;
        Ok(response_text)
    }

    pub async fn submit_sender_id(
        &self,
        payload: SenderIdStruct,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let api_url = "https://my.kudisms.net/api/senderID";
        let client = Client::new();

        let response = client.post(api_url).json(&payload).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Request failed with status: {}", response.status()).into())
        }
    }

    pub async fn check_sender_id(
        &self,
        sender_id_to_check: &str,
    ) -> Result<SenderIdCheckResponse, Box<dyn std::error::Error>> {
        let api_url = "https://my.kudisms.net/api/check_senderID";

        let form = [
            ("token", self.token.as_str()),
            ("senderID", sender_id_to_check),
        ];

        let client = Client::new();
        let response = client.post(api_url).form(&form).send().await?;

        if response.status().is_success() {
            let json = response.json().await?;
            Ok(json)
        } else {
            Err(format!("Failed to check sender ID. Status: {}", response.status()).into())
        }
    }
    pub async fn send_bulk_sms(
        &self,
        payload: SendBulkSmsPayload,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let recipients_combined = payload.recipients.join(",");

        let api_url = format!(
            "https://my.kudisms.net/api/sms?token={}&senderID={}&recipients={}&message={}&gateway=2",
            payload.token,
            payload.sender_id,
            urlencoding::encode(&recipients_combined),
            urlencoding::encode(&payload.message),
        );

        let client = Client::new();
        let response = client.post(&api_url).send().await?;

        if response.status().is_success() {
            let text = response.text().await?;
            Ok(text)
        } else {
            Err(format!("Failed to send SMS: {}", response.status()).into())
        }
    }

    pub async fn send_corporate_sms(
        &self,
        payload: SendCorporateEmail,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
        let response = client
            .post("https://my.kudisms.net/api/corporate")
            .json(&payload)
            .send()
            .await?;

        let status = response.status();
        if status.is_success() {
            println!("Corporate SMS sent successfully");
            Ok(())
        } else {
            Err(format!("Failed to send SMS: {}", status).into())
        }
    }
}
