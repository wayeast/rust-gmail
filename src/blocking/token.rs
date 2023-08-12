use crate::{
    common::token::{create_jwt, GoogleAuthRequest, GoogleAuthResponse},
    error::{GoogleApiError, Result},
    service_account::ServiceAccount,
};

pub fn retrieve_token_blocking(
    service_account: &ServiceAccount,
    send_from_email: &str,
) -> Result<String> {
    let jwt = create_jwt(service_account, send_from_email)?;

    let client = reqwest::blocking::Client::new();

    // GoogleAuthResponse
    let response_text = client
        .post(&service_account.token_uri)
        .form(&GoogleAuthRequest::new(jwt))
        .send()?
        .text()?;

    let response: GoogleAuthResponse = serde_json::from_str(&response_text)
        .map_err(|_| GoogleApiError::TokenRetrivalError(response_text))?;

    Ok(response.access_token)
}
