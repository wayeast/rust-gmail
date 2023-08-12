use crate::{
    common::send_email::{
        mock_print_email, GoogleSendEmailRequest, GoogleSendEmailResponse, SEND_EMAIL_ENDPOINT,
        SEND_EMAIL_QUERY_PARAMETERS,
    },
    error::{GoogleApiError, Result},
};

pub fn send_email_blocking(
    receiver_email: &str,
    subject: &str,
    content: &str,
    token: &str,
    send_from_email: &str,
    mock_mode: bool,
) -> Result<()> {
    if mock_mode {
        mock_print_email(receiver_email, subject, content, send_from_email);
        return Ok(());
    }

    do_send_email_blocking(receiver_email, subject, content, token, send_from_email)
}

fn do_send_email_blocking(
    receiver_email: &str,
    subject: &str,
    content: &str,
    token: &str,
    send_from_email: &str,
) -> Result<()> {
    let send_email_request =
        GoogleSendEmailRequest::new(&send_from_email, receiver_email, subject, content);

    let client = reqwest::blocking::Client::new();
    let response_text = client
        .post(SEND_EMAIL_ENDPOINT)
        .query(&SEND_EMAIL_QUERY_PARAMETERS)
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
        .json(&send_email_request)
        .send()?
        .text()?;

    let _response: GoogleSendEmailResponse = serde_json::from_str(&response_text)
        .map_err(|_| GoogleApiError::EmailSendError(response_text))?;

    Ok(())
}
