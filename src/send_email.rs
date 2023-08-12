use crate::error::{GoogleApiError, Result};
use base64;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GoogleSendEmailRequest {
    raw: String,
}

impl GoogleSendEmailRequest {
    fn new(from: &str, to: &str, subject: &str, content: &str) -> Self {
        let raw_message = base64::encode(format!(
            "From: {}\r\nTo: {}\r\nSubject: {}\r\n\r\n{}\r\n",
            from, to, subject, content
        ));
        Self { raw: raw_message }
    }
}

// Note: the `/me/` is a parameter for the CLIENT_ID
const SEND_EMAIL_ENDPOINT: &str = "https://gmail.googleapis.com/gmail/v1/users/me/messages/send";

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
#[serde(rename_all = "camelCase")]
struct GoogleSendEmailResponse {
    id: String,
    thread_id: String,
    label_ids: Vec<String>,
}

pub async fn send_email(
    receiver_email: &str,
    subject: &str,
    content: &str,
    token: &str,
    send_from_email: &str,
    mock_mode: bool,
) -> Result<()> {
    if mock_mode {
        println!(
            "MOCK MODE SEND EMAIL
Sending from {} to {}
Subject: {}
Content: {}
        ",
            send_from_email, receiver_email, subject, content
        );
        return Ok(());
    }

    do_send_email(receiver_email, subject, content, token, send_from_email).await
}

async fn do_send_email(
    receiver_email: &str,
    subject: &str,
    content: &str,
    token: &str,
    send_from_email: &str,
) -> Result<()> {
    let send_email_request =
        GoogleSendEmailRequest::new(&send_from_email, receiver_email, subject, content);

    let client = reqwest::Client::new();
    let response_text = client
        .post(SEND_EMAIL_ENDPOINT)
        .query(&[("alt", "json"), ("prettyPrint", "false")])
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
        .json(&send_email_request)
        .send()
        .await?
        .text()
        .await?;

    let _response: GoogleSendEmailResponse = serde_json::from_str(&response_text)
        .map_err(|_| GoogleApiError::EmailSendError(response_text))?;

    Ok(())
}
