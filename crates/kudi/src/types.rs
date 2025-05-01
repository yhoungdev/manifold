use serde::Serialize;

pub struct KudiClient {
    pub token: String,
    pub sender_id: String,
}

pub struct SmsOtpPayload {
    pub token: String,
    pub sender_id: String,
    pub recipients: String,
    pub otp: String,
    pub app_name_code: String,
    pub template_code: String,
}

#[derive(Serialize)]
pub struct SenderIdStruct {
    pub token: String,
    pub message: String,
    pub sender_id: String,
}

pub struct SendBulkSmsPayload {
    pub token: String,
    pub recipients: Vec<String>,
    pub message: String,
    pub sender_id: String,
}

#[derive(Serialize)]
pub struct SendCorporateEmail {
    sender_id: String,
    recipients: Vec<String>,
    message: String,
}

pub struct SenderIdCheckResponse {
    pub status: String,
    pub message: Option<String>,
}

pub struct KudiMessage {}
