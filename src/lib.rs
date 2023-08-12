#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! # A Rust library to interact with the google Gmail API using a service account.
//!
//! Currently focused only on support for sending emails but this may expand in the future.

use std::path::Path;

use error::Result;
use send_email::send_email;
use service_account::ServiceAccount;
use token::retrieve_token;

#[doc = "inline"]
pub mod error;

mod send_email;
mod service_account;
mod token;

/// TODO
pub struct GmailClientBuilder {
    service_account: ServiceAccount,
    send_from_email: String,
    mock_mode: bool,
}

impl<'a> GmailClientBuilder {
    /// TODO
    pub fn new<P: AsRef<Path>, S: Into<String>>(
        service_account_path: P,
        send_from_email: S,
    ) -> Result<Self> {
        Ok(Self {
            service_account: ServiceAccount::load_from_file(service_account_path)?,
            send_from_email: send_from_email.into(),
            mock_mode: false,
        })
    }

    /// Enables "mock mode" meaning that instead of sending emails, they will be printed in a log message.
    pub fn mock_mode(mut self) -> Self {
        self.mock_mode = true;
        self
    }

    /// TODO
    pub async fn build(self) -> Result<GmailClient> {
        let token = retrieve_token(&self.service_account, &self.send_from_email).await?;

        Ok(GmailClient {
            send_from_email: self.send_from_email,
            token,
            mock_mode: self.mock_mode,
        })
    }
}

/// TODO
pub struct GmailClient {
    send_from_email: String,
    token: String,
    mock_mode: bool,
}

impl GmailClient {
    /// Alias for `GmailClientBuilder::new`
    pub fn builder<P: AsRef<Path>, S: Into<String>>(
        service_account_path: P,
        send_from_email: S,
    ) -> Result<GmailClientBuilder> {
        GmailClientBuilder::new(service_account_path, send_from_email)
    }

    /// TODO
    pub async fn send_email(
        &self,
        send_to_email: &str,
        subject: &str,
        content: &str,
    ) -> Result<()> {
        send_email(
            send_to_email,
            subject,
            content,
            &self.token,
            &self.send_from_email,
            self.mock_mode,
        )
        .await
    }
}
