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
use mail::Mail;

fn apply_rules(mail: &Mail, connection: &mut Connection, config: &Config, i: usize) -> bool {
    'rule_loop: for rule in &config.rules {
        for condition in &rule.conditions {
            if !condition.check(&mail) {
                println!(
                    "mail does not meet condition {:?}: {:?}",
                    condition,
                    mail.to
                );
                continue 'rule_loop;
            }
        }
        println!("mail does meet conditions: {}", mail.subject);
        for action in &rule.actions {
            action.apply(connection, i).unwrap();
            if action.remove_mail() {
                return true;
            }
        }
    }
    false
}

// TODO check that there is no duplicate delete

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
    let mut len = connection.mail_number("INBOX").unwrap();
    // TODO test delete and else
    while i < len {
        i += 1;
        let mail = connection.fetch_mail(i).unwrap();
        if apply_rules(&mail, &mut connection, &config, i) {
            i -= 1;
            len -= 1;
        }
    }
    loop {
        connection.wait().expect("error while waiting");
        i += 1;
        let mail = connection.fetch_mail(i).unwrap();
        if apply_rules(&mail, &mut connection, &config, i) {
            i -= 1;
        }
    }
}
