#[macro_use]
extern crate error_chain;
extern crate imap;
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
    println!("{:?}", connection.list("INBOX", "*").unwrap());
    println!(
        "{:?}",
        connection
            .status("INBOX", "(MESSAGES UNSEEN RECENT)")
            .unwrap()
    );
    //println!(
    //    "{:?}",
    //    connection
    //        .fetch("10", "body.peek[header.fields (FROM TO CC SUBJECT)]")
    //        .unwrap()
    //);
    //println!("{:?}", connection.fetch("10", "body.peek[1]").unwrap());
    println!("{:?}", connection.fetch_mail(10).unwrap());
    //println!(
    //    "{:?}",
    //    connection
    //        .fetch("1:*", "(BODY[HEADER.FIELDS (SUBJECT FROM)])")
    //        .unwrap()
    //);
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
