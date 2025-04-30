# Plunk Email SDK

A simple Rust SDK for sending transactional emails using the Plunk API.

## Setup

1. Add dependencies to your `Cargo.toml`:
```toml
[dependencies]
reqwest = "0.12.15"
tokio = "1.44.2"
serde = "1.0.219"
common_manifold = "0.1.0"
dotenv = "0.15.0"
```

2. Create a `.env` file with your credentials:
```env
PLUNK_PRIVATE_KEY=your_plunk_api_key_here
```

## Quick Start

```rust
use client::{PlunkClientTrait, PlunkPayloads};
use common_manifold::types::plunk_types::PlunkClient;

// Initialize the client
let plunk_key = env::var("PLUNK_PRIVATE_KEY").expect("Set PLUNK_PRIVATE_KEY in .env");
let client = PlunkClient::new(plunk_key);

// Create an email payload
let payload = PlunkPayloads {
    to: "recipient@example.com".to_string(),
    subject: Some("Hello! 👋".to_string()),
    body: "Your email content here".to_string(),
};

// Send the email
match client.send_transactional_email(payload).await {
    Ok(msg) => println!("{}", msg),
    Err(err) => eprintln!("Error sending email: {}", err),
}
```

## API Reference

### PlunkClient

Initialize a new Plunk client:
```rust
let client = PlunkClient::new(plunk_key);
```

### PlunkPayloads

Structure for creating email payloads:

| Field   | Type              | Description                    |
|---------|-------------------|--------------------------------|
| to      | String           | Recipient's email address      |
| subject | Option<String>   | Email subject (optional)       |
| body    | String           | Email content (supports HTML)  |

### Sending Emails

The `send_transactional_email` function returns a Result with either a success message or an error:
```rust
async fn send_transactional_email(payload: PlunkPayloads) -> Result<String, Error>
```

## Error Handling

The SDK handles two main types of errors:
- Environment variable errors (missing API key)
- Email sending failures

Always wrap the email sending operation in a proper error handling block:
```rust
match client.send_transactional_email(payload).await {
    Ok(msg) => println!("Success: {}", msg),
    Err(err) => eprintln!("Error: {}", err),
}
```

## License
MIT