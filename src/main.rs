#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate imap;
extern crate mailparse;
extern crate openssl;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate unix_daemonize;

mod account;
mod connection;
mod config;
mod error;
mod mail;
mod rule;

use clap::{App, Arg};
use connection::Connection;
use config::Config;
use error::*;
use mail::Mail;
use std::path::Path;
use std::thread;
use unix_daemonize::{daemonize_redirect, ChdirMode};

fn apply_rules(
    mail: &Mail,
    connection: &mut Connection,
    config: &Config,
    i: usize,
) -> Result<bool> {
    'rule_loop: for rule in &config.rules {
        let mut condition_check = true;
        for condition in &rule.conditions {
            condition_check = condition.check(&mail);
            if !condition_check && !rule.any {
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
                continue 'rule_loop;
            }
        }
        println!("[{}] mail meet conditions: {}", rule.name, mail.subject);
        for action in &rule.actions {
            action.apply(connection, i)?;
            if action.is_rules_stop() {
                return Ok(false);
            }
            if action.is_remove() {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn manage_account<P: AsRef<Path>>(path: P) -> Result<()> {
    let config = Config::from_file(path)?;
    let mut connection = Connection::connect(&config.account)?;
    connection.set_debug(false);
    connection.select("INBOX")?;
    let mut i = 0;
    let mut len = connection.mail_number("INBOX")?;
    while i < len {
        i += 1;
        let mail = connection.fetch_mail(i)?;
        if apply_rules(&mail, &mut connection, &config, i)? {
            i -= 1;
            len -= 1;
        }
    }
    let sync = config.account.sync.unwrap_or(60);
    loop {
        for _ in 0..(sync / 5) {
            connection.noop()?;
            thread::sleep(::std::time::Duration::from_secs(5));
        }
        println!("Syncing...");
        connection.noop()?;
        match connection.mail_number("INBOX")? {
            a if a == i => continue,
            a if a < i => i = a - 1,
            _ => {}
        }
        println!("New mail");
        i += 1;
        let mail = connection.fetch_mail(i)?;
        if apply_rules(&mail, &mut connection, &config, i)? {
            i -= 1;
        }
    }
}

fn run_threads(accounts: Vec<String>) {
    let mut handlers = Vec::new();
    for account in accounts {
        handlers.push(thread::spawn(move || match manage_account(account) {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        }));
    }

    for handler in handlers {
        println!("{:?}", handler.join());
    }
}

// TODO test delete and else
// TODO add any or every and else
// TODO automatic imap
// TODO account manager
// TODO rule manager
// TODO list all created folders
fn main() {
    let app = App::new("narricky")
        .version(crate_version!())
        .author("Bastien Badzioch <fourdotfiveg@gmail.com>")
        .about("Apply rules to mail")
        .arg(
            Arg::with_name("account")
                .takes_value(true)
                .multiple(true)
                .help("Path to file(s) describing your account(s)")
                .required(true),
        )
        .arg(Arg::with_name("daemon").short("b").long("daemon").help(
            "Daemonize process",
        ))
        .get_matches();

    let accounts: Vec<String> = app.values_of("account")
        .unwrap_or_default()
        .map(|s| s.to_string())
        .collect();
    if app.is_present("daemon") {
        let _ = ::std::fs::create_dir_all("/tmp/narricky");
        daemonize_redirect(
            Some("/tmp/narricky/stdout.log"),
            Some("/tmp/narricky/stderr.log"),
            ChdirMode::NoChdir,
        ).unwrap();
    }
    run_threads(accounts);
}
