#[macro_use]
extern crate error_chain;
extern crate imap;
extern crate mailparse;
extern crate openssl;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod account;
mod connection;
mod config;
mod error;
mod mail;
mod rule;

use std::env;

use connection::Connection;
use config::Config;

fn main() {
    let mut args = env::args();
    let config;
    args.next();
    if let Some(file) = args.next() {
        config = Config::from_file(file).unwrap();
    } else {
        panic!("Missing file");
    }
    let mut connection = Connection::connect(&config.account).unwrap();
    connection.select("INBOX").unwrap();
    println!(
        "{:?}",
        connection.status("INBOX", "(MESSAGES UNSEEN RECENT)")
    );
    let mut i = 0;
    let mut len = connection.mail_number("INBOX").unwrap() + 1;
    // TODO test delete and else
    // TODO fix condition is for sender, just parse mail
    // TODO add sender_mail and sender_name
    while i < len {
        i += 1;
        let mail = connection.fetch_mail(i).unwrap();
        'rule_loop: for rule in &config.rules {
            for condition in &rule.conditions {
                if !condition.check(&mail) {
                    println!(
                        "mail does not meet condition {:?}: {:?}",
                        condition,
                        mail.from
                    );
                    continue 'rule_loop;
                }
            }
            println!("mail does meet conditions: {}", mail.subject);
            for action in &rule.actions {
                action.apply(&mut connection, i).unwrap();
                if action.remove_mail() {
                    i -= 1;
                    len -= 1;
                }
            }
        }
    }
}
