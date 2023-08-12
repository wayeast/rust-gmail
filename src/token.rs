use std::collections::BTreeMap;

use chrono::{Duration, Utc};
use jwt::{PKeyWithDigest, SignWithKey};
use openssl::{hash::MessageDigest, pkey::PKey};
use serde::{Deserialize, Serialize};

use crate::{error::GoogleApiError, service_account::ServiceAccount};

#[derive(Serialize, Deserialize)]
struct GoogleAuthResponse {
    access_token: String,
    expires_in: u32,
    token_type: String,
}

const GRANT_TYPE_SERVICE_ACCOUNT: &str = "urn:ietf:params:oauth:grant-type:jwt-bearer";
#[derive(Serialize, Deserialize)]
struct GoogleAuthRequest {
    grant_type: String,
    assertion: String,
}

impl GoogleAuthRequest {
    fn new(assertion: String) -> Self {
        Self {
            grant_type: GRANT_TYPE_SERVICE_ACCOUNT.to_string(),
            assertion,
        }
    }
}

pub async fn retrieve_token(
    service_account: &ServiceAccount,
    send_from_email: &str,
) -> Result<String, GoogleApiError> {
    let jwt = create_jwt(service_account, send_from_email)?;

    let client = reqwest::Client::new();

    // GoogleAuthResponse
    let response_text = client
        .post(&service_account.token_uri)
        .form(&GoogleAuthRequest::new(jwt))
        .send()
        .await?
        .text()
        .await?;

    let response: GoogleAuthResponse = serde_json::from_str(&response_text)
        .map_err(|_| GoogleApiError::TokenRetrivalError(response_text))?;

    Ok(response.access_token)
}

const GMAIL_SEND_EMAIL_SCOPE: &str = "https://www.googleapis.com/auth/gmail.send";
const GOOGLE_AUD_VALUE: &str = "https://oauth2.googleapis.com/token";

fn create_jwt(
    service_account: &ServiceAccount,
    send_from_email: &str,
) -> Result<String, GoogleApiError> {
    let private_key = PKey::private_key_from_pem(service_account.private_key.as_bytes())?;
    let key_with_digest = PKeyWithDigest {
        digest: MessageDigest::sha256(),
        key: private_key,
    };

    let mut claims: BTreeMap<&str, &str> = BTreeMap::new();

    claims.insert("iss", &service_account.client_email);
    claims.insert("scope", GMAIL_SEND_EMAIL_SCOPE);
    claims.insert("aud", GOOGLE_AUD_VALUE);

    let now = Utc::now();
    let now_timestamp = now.timestamp().to_string();
    claims.insert("iat", &now_timestamp);

    let exp_time = now + Duration::hours(1);
    let exp_time_timestamp = exp_time.timestamp().to_string();
    claims.insert("exp", &exp_time_timestamp);
    claims.insert("sub", send_from_email);

    Ok(claims.sign_with_key(&key_with_digest)?)
}
