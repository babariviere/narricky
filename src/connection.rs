use account::Account;
use error::*;
use imap::client::Client;
use imap::mailbox::Mailbox;
use openssl::ssl::{SslConnectorBuilder, SslMethod, SslStream};
use std::net::TcpStream;

enum ConnectionResult {
    Normal(Client<TcpStream>),
    Secure(Client<SslStream<TcpStream>>),
}

pub struct Connection(ConnectionResult);

impl Connection {
    /// Establish connection with this account
    pub fn connect(account: &Account) -> Result<Connection> {
        if account.secure {
            let ssl_connector = SslConnectorBuilder::new(SslMethod::tls())
                .chain_err(|| "fail with ssl")?
                .build();
            let mut imap_socket = Client::secure_connect(
                (account.domain.as_str(), account.port),
                &account.domain,
                ssl_connector,
            ).chain_err(|| "fail with connect")?;
            imap_socket
                .login(&account.username, &account.password)
                .chain_err(|| "fail when login")?;
            Ok(Connection(ConnectionResult::Secure(imap_socket)))
        } else {
            let mut imap_socket = Client::connect((account.domain.as_str(), account.port))?;
            imap_socket
                .login(&account.username, &account.password)
                .chain_err(|| "fail when login")?;
            Ok(Connection(ConnectionResult::Normal(imap_socket)))
        }
    }

    /// Selects a mailbox
    pub fn select(&mut self, mailbox_name: &str) -> Result<Mailbox> {
        match &mut self.0 {
            &mut ConnectionResult::Normal(ref mut s) => {
                s.select(mailbox_name).chain_err(|| "fail when selecting")
            }
            &mut ConnectionResult::Secure(ref mut s) => {
                s.select(mailbox_name).chain_err(|| "fail when selecting")
            }
        }
    }

    /// Examine a mailbox
    pub fn examine(&mut self, mailbox_name: &str) -> Result<Mailbox> {
        match &mut self.0 {
            &mut ConnectionResult::Normal(ref mut s) => {
                s.examine(mailbox_name).chain_err(|| "fail when examining")
            }
            &mut ConnectionResult::Secure(ref mut s) => {
                s.examine(mailbox_name).chain_err(|| "fail when examining")
            }
        }
    }

    /// Fetch data
    pub fn fetch(&mut self, sequence_set: &str, query: &str) -> Result<Vec<String>> {
        match &mut self.0 {
            &mut ConnectionResult::Normal(ref mut s) => {
                s.fetch(sequence_set, query).chain_err(
                    || "fail when fetching",
                )
            }
            &mut ConnectionResult::Secure(ref mut s) => {
                s.fetch(sequence_set, query).chain_err(
                    || "fail when fetching",
                )
            }
        }
    }

    /// Create a mailbox
    pub fn create(&mut self, mailbox_name: &str) -> Result<()> {
        match &mut self.0 {
            &mut ConnectionResult::Normal(ref mut s) => {
                s.create(mailbox_name).chain_err(|| "fail when creating")
            }
            &mut ConnectionResult::Secure(ref mut s) => {
                s.create(mailbox_name).chain_err(|| "fail when creating")
            }
        }
    }

    /// Delete a mailbox
    pub fn delete(&mut self, mailbox_name: &str) -> Result<()> {
        match &mut self.0 {
            &mut ConnectionResult::Normal(ref mut s) => {
                s.delete(mailbox_name).chain_err(|| "fail when deleting")
            }
            &mut ConnectionResult::Secure(ref mut s) => {
                s.delete(mailbox_name).chain_err(|| "fail when deleting")
            }
        }
    }

    /// Rename a mailbox
    pub fn rename(&mut self, mailbox_name: &str, new_mailbox_name: &str) -> Result<()> {
        match &mut self.0 {
            &mut ConnectionResult::Normal(ref mut s) => {
                s.rename(mailbox_name, new_mailbox_name).chain_err(
                    || "fail when renaming",
                )
            }
            &mut ConnectionResult::Secure(ref mut s) => {
                s.rename(mailbox_name, new_mailbox_name).chain_err(
                    || "fail when renaming",
                )
            }
        }
    }

    /// List of capabilites
    pub fn capability(&mut self) -> Result<Vec<String>> {
        match &mut self.0 {
            &mut ConnectionResult::Normal(ref mut s) => {
                s.capability().chain_err(|| "fail when getting capabilites")
            }
            &mut ConnectionResult::Secure(ref mut s) => {
                s.capability().chain_err(|| "fail when getting capabilites")
            }
        }
    }

    /// Removes all messages that have the \Deleted flag
    pub fn expunge(&mut self) -> Result<()> {
        match &mut self.0 {
            &mut ConnectionResult::Normal(ref mut s) => {
                s.expunge().chain_err(|| "fail with expunge")
            }
            &mut ConnectionResult::Secure(ref mut s) => {
                s.expunge().chain_err(|| "fail with expunge")
            }
        }
    }

    /// Alters data with a message
    pub fn store(&mut self, sequence_set: &str, query: &str) -> Result<Vec<String>> {
        match &mut self.0 {
            &mut ConnectionResult::Normal(ref mut s) => {
                s.store(sequence_set, query).chain_err(|| "fail with store")
            }
            &mut ConnectionResult::Secure(ref mut s) => {
                s.store(sequence_set, query).chain_err(|| "fail with store")
            }
        }
    }

    /// Copy message to mailbox
    pub fn copy(&mut self, sequence_set: &str, mailbox_name: &str) -> Result<()> {
        match &mut self.0 {
            &mut ConnectionResult::Normal(ref mut s) => {
                s.copy(sequence_set, mailbox_name).chain_err(
                    || "fail with copying",
                )
            }
            &mut ConnectionResult::Secure(ref mut s) => {
                s.copy(sequence_set, mailbox_name).chain_err(
                    || "fail with copying",
                )
            }
        }
    }

    /// List mails
    pub fn list(&mut self, ref_name: &str, mailbox_search_pattern: &str) -> Result<Vec<String>> {
        match &mut self.0 {
            &mut ConnectionResult::Normal(ref mut s) => {
                s.list(ref_name, mailbox_search_pattern).chain_err(
                    || "fail when getting list",
                )
            }
            &mut ConnectionResult::Secure(ref mut s) => {
                s.list(ref_name, mailbox_search_pattern).chain_err(
                    || "fail when getting list",
                )
            }
        }
    }

    /// Get status of mailbox
    pub fn status(&mut self, mailbox_name: &str, status_data_items: &str) -> Result<Vec<String>> {
        match &mut self.0 {
            &mut ConnectionResult::Normal(ref mut s) => {
                s.status(mailbox_name, status_data_items).chain_err(
                    || "fail when getting status",
                )
            }
            &mut ConnectionResult::Secure(ref mut s) => {
                s.status(mailbox_name, status_data_items).chain_err(
                    || "fail when getting status",
                )
            }
        }
    }

    /// Wait for new event
    pub fn wait(&mut self) -> Result<()> {
        match &mut self.0 {
            &mut ConnectionResult::Normal(ref mut s) => {
                s.idle().chain_err(|| "fail when waiting")?.wait().map_err(
                    |e| {
                        e.into()
                    },
                )
            }
            &mut ConnectionResult::Secure(ref mut s) => {
                s.idle().chain_err(|| "fail when waiting")?.wait().map_err(
                    |e| {
                        e.into()
                    },
                )
            }
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        let _ = match &mut self.0 {
            &mut ConnectionResult::Normal(ref mut s) => s.logout(),
            &mut ConnectionResult::Secure(ref mut s) => s.logout(),
        };
    }
}
