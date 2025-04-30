mod client;

use dotenv::dotenv;
use std::env;

use client::{PlunkClientTrait, PlunkPayloads};
use common::types::plunk_types::PlunkClient;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let plunk_key = env::var("PLUNK_PRIVATE_KEY").expect("Set PLUNK_PRIVATE_KEY in .env");
    let test_mail = env::var("TEST_MAIL").expect("Set TEST_MAIL in .env");
    let client = PlunkClient::new(plunk_key);

    let payload = PlunkPayloads {
        to: test_mail.to_string(),
        subject: Some("Welcome! 👋🏾".to_string()),
        body: " 🍟 Yo, moment of truth, i'm testing out Plunk mail ✉️".to_string(),
    };

    match client.send_transactional_email(payload).await {
        Ok(msg) => println!("{}", msg),
        Err(err) => eprintln!("Error sending email: {}", err),
    }
}
