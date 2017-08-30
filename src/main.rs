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
    for i in 1..connection.mail_number("INBOX").unwrap() {
        let mail = connection.fetch_mail(i).unwrap();
        'rule_loop: for rule in &config.rules {
            for condition in &rule.conditions {
                if !condition.check(&mail) {
                    println!(
                        "mail does not meet condition {:?}: {}",
                        condition,
                        mail.subject
                    );
                    continue 'rule_loop;
                }
            }
            println!("mail does meet conditions: {}", mail.subject);
        }
    }
    //println!("{:?}", connection.store("1:*", "-FLAGS (\\Seen)").unwrap());
    //match imap_socket.capability() {
    //    Ok(capabilities) => {
    //        for capability in capabilities.iter() {
    //            println!("{}", capability);
    //        }
    //    }
    //    Err(e) => println!("Error parsing capability: {}", e),
    //};

    //match imap_socket.select("INBOX") {
    //    Ok(mailbox) => {
    //        println!("{}", mailbox);
    //    }
    //    Err(e) => println!("Error selecting INBOX: {}", e),
    //};

    ////imap_socket.create("NEWBOW/SubBox").unwrap();

    //match imap_socket.list("/", "*") {
    //    Ok(a) => {
    //        for b in a {
    //            println!("{}", b);
    //        }
    //    }
    //    Err(e) => println!("Error listing: {}", e),
    //}


    // match imap_socket.fetch("2", "body[text]") {
    //     Ok(lines) => {
    //         for line in lines.iter() {
    //             print!("{}", line);
    //         }
    //     }
    //     Err(e) => println!("Error Fetching email 2: {}", e),
    // };

    //imap_socket.logout().unwrap();
}
