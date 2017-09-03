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
        let mut condition_check = true;
        for condition in &rule.conditions {
            condition_check = condition.check(&mail);
            if !condition_check && !rule.any {
                println!(
                    "mail does not meet condition {:?}: {}",
                    condition,
                    mail.subject
                );
                continue 'rule_loop;
            } else if !condition_check {
                continue;
            }
            if rule.any {
                break;
            }
        }
        if !condition_check && rule.any {
            continue 'rule_loop;
        }
        for exception in &rule.exceptions {
            if exception.check(&mail) {
                println!("mail does meet exception {:?}: {}", exception, mail.subject);
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
    let mut i = 0;
    let mut len = connection.mail_number("INBOX").unwrap();
    // TODO test delete and else
    // TODO add any or every and else
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
