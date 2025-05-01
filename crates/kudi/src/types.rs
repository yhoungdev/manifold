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
pub struct  SenderIdStruct {
   pub token: String,
   pub message: String,
   pub sender_id: String,
}

pub struct KudiMessage {}
