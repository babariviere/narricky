#[macro_use]
extern crate error_chain;
extern crate imap;
extern crate openssl;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod account;
mod config;
mod error;
mod rule;

use openssl::ssl::SslStream;
use imap::client::Client;
use std::env;
use std::path::Path;
use std::net::TcpStream;

use config::Config;

fn parse_file<P: AsRef<Path>>(path: P) -> Client<SslStream<TcpStream>> {
    let config = match Config::from_file(path) {
        Ok(config) => config,
        Err(e) => {
            println!("{}", e);
            ::std::process::exit(1);
        }
    };
    let account = config.account;
    account.secure_connect().unwrap()
}

fn main() {
    let mut args = env::args();
    let mut imap_socket;
    args.next();
    if let Some(file) = args.next() {
        imap_socket = parse_file(file);
    } else {
        panic!("Missing file");
    }
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

    imap_socket.logout().unwrap();
}
