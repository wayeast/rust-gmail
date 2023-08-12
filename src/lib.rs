#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! # A Rust library to interact with the google Gmail API using a service account.
//!
//! Currently focused only on support for sending emails but this may expand in the future.

use std::path::Path;

use async_impl::{send_email::send_email, token::retrieve_token};
use error::Result;
use service_account::ServiceAccount;

#[doc = "inline"]
pub mod error;

mod async_impl;
mod common;
mod service_account;

#[cfg(feature = "blocking")]
mod blocking;

/// TODO
#[derive(Debug, Clone)]
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

    /// TODO
    #[cfg(feature = "blocking")]
    pub fn build_blocking(self) -> Result<GmailClient> {
        use blocking::token::retrieve_token_blocking;

        let token = retrieve_token_blocking(&self.service_account, &self.send_from_email)?;

        Ok(GmailClient {
            send_from_email: self.send_from_email,
            token,
            mock_mode: self.mock_mode,
        })
    }
}

/// TODO
#[derive(Debug, Clone)]
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

    /// TODO
    #[cfg(feature = "blocking")]
    pub fn send_email_blocking(
        &self,
        send_to_email: &str,
        subject: &str,
        content: &str,
    ) -> Result<()> {
        use blocking::send_email::send_email_blocking;

        send_email_blocking(
            send_to_email,
            subject,
            content,
            &self.token,
            &self.send_from_email,
            self.mock_mode,
        )
    }
}
