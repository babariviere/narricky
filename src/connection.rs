use account::Account;
use error::*;
use imap::client::Client;
use imap::mailbox::Mailbox;
use mail::Mail;
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

    /// Set debug for connection
    pub fn set_debug(&mut self, debug: bool) {
        match &mut self.0 {
            &mut ConnectionResult::Normal(ref mut s) => s.debug = debug,
            &mut ConnectionResult::Secure(ref mut s) => s.debug = debug,
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

    /// Fetch mail
    pub fn fetch_mail(&mut self, index: usize) -> Result<Mail> {
        let mut headers = self.fetch(
            &index.to_string(),
            "body.peek[header.fields (FROM TO CC SUBJECT)]",
        )?;
        let mut text = self.fetch(&index.to_string(), "body.peek[1]")?;
        headers.remove(0);
        headers.pop();
        headers.pop();
        text.remove(0);
        text.pop();
        text.pop();
        headers.append(&mut text);
        Mail::parse_fetched(headers)
    }

    /// Create a mailbox
    pub fn create(&mut self, mailbox_name: &str) -> Result<()> {
        // TODO test subfolder
        let mut list: Vec<String> = mailbox_name.split('/').map(|s| s.to_owned()).collect();
        let name = list.pop().unwrap_or(mailbox_name.to_owned());
        let mut folder_name = list.iter().map(|s| format!("{}/", s)).collect::<String>();
        if folder_name.is_empty() {
            folder_name = "/".to_owned();
        }
        if self.list(&folder_name, &name)?.len() >= 2 {
            return Ok(());
        }
        match &mut self.0 {
            &mut ConnectionResult::Normal(ref mut s) => {
                s.create(mailbox_name).chain_err(|| "fail when creating")
            }
            &mut ConnectionResult::Secure(ref mut s) => {
                s.create(mailbox_name).chain_err(|| "fail when creating")
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

    /// Send noop
    pub fn noop(&mut self) -> Result<()> {
        match &mut self.0 {
            &mut ConnectionResult::Normal(ref mut s) => s.noop().chain_err(|| "fail with noop"),
            &mut ConnectionResult::Secure(ref mut s) => s.noop().chain_err(|| "fail with noop"),
        }
    }

    /// Get number of mails
    pub fn mail_number(&mut self, mailbox_name: &str) -> Result<usize> {
        let status = self.status(mailbox_name, "(messages)")?;
        let num = status[0]
            .matches(char::is_numeric)
            .map(|c| c)
            .collect::<String>();
        num.parse::<usize>().chain_err(
            || "fail parsing number of mails",
        )
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
