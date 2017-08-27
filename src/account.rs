use error::*;
use imap::client::Client;
use openssl::ssl::{SslConnectorBuilder, SslMethod, SslStream};
use std::net::TcpStream;
use toml::Value;

/// Mail account
#[derive(Debug, Deserialize)]
pub struct Account {
    pub username: String,
    pub password: String,
    pub domain: String,
    pub port: u16,
    pub secure: bool,
}

impl Account {
    /// Convert toml table into Account
    pub fn from_toml(toml: &Value) -> Result<Account> {
        toml.clone().try_into().map_err(|e| e.into())
    }

    /// Connect to imap server with ssl
    pub fn secure_connect(&self) -> Result<Client<SslStream<TcpStream>>> {
        let ssl_connector = SslConnectorBuilder::new(SslMethod::tls())
            .chain_err(|| "fail with ssl")?
            .build();
        let mut imap_socket = Client::secure_connect(
            (self.domain.as_str(), self.port),
            &self.domain,
            ssl_connector,
        ).chain_err(|| "can't create connection")?;
        imap_socket
            .login(&self.username, &self.password)
            .chain_err(|| "fail when login")?;
        Ok(imap_socket)
    }
}
