use std::collections::BTreeMap;

use crate::error::Result;
use chrono::{Duration, Utc};
use jwt::{PKeyWithDigest, SignWithKey};
use openssl::{hash::MessageDigest, pkey::PKey};
use serde::{Deserialize, Serialize};

use crate::service_account::ServiceAccount;

pub const GMAIL_SEND_EMAIL_SCOPE: &str = "https://www.googleapis.com/auth/gmail.send";
pub const GOOGLE_AUD_VALUE: &str = "https://oauth2.googleapis.com/token";

#[derive(Serialize, Deserialize)]
pub struct GoogleAuthResponse {
    pub access_token: String,
    pub expires_in: u32,
    pub token_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct GoogleAuthRequest {
    grant_type: String,
    assertion: String,
}

const GRANT_TYPE_SERVICE_ACCOUNT: &str = "urn:ietf:params:oauth:grant-type:jwt-bearer";
impl GoogleAuthRequest {
    pub fn new(assertion: String) -> Self {
        Self {
            grant_type: GRANT_TYPE_SERVICE_ACCOUNT.to_string(),
            assertion,
        }
    }
}

pub fn create_jwt(service_account: &ServiceAccount, send_from_email: &str) -> Result<String> {
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
