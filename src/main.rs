extern crate imap;
extern crate openssl;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use openssl::ssl::{SslConnectorBuilder, SslMethod};
use openssl::ssl::SslStream;
use imap::client::Client;
use std::env;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::net::TcpStream;

#[derive(Deserialize)]
struct Config {
    account: Account,
}

#[derive(Deserialize)]
struct Account {
    username: String,
    password: String,
    domain: String,
    port: u16,
}

fn parse_file<P: AsRef<Path>>(path: P) -> Client<SslStream<TcpStream>> {
    let mut f = File::open(path).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    let config: Config = toml::from_str(&buf).unwrap();
    let account = config.account;
    let ssl_connector = SslConnectorBuilder::new(SslMethod::tls()).unwrap().build();
    let mut imap_socket = Client::secure_connect(
        (account.domain.as_str(), account.port),
        &account.domain,
        ssl_connector,
    ).unwrap();
    imap_socket
        .login(&account.username, &account.password)
        .unwrap();
    imap_socket
}

// To connect to the gmail IMAP server with this you will need to allow unsecure apps access.
// See: https://support.google.com/accounts/answer/6010255?hl=en
// Look at the gmail_oauth2.rs example on how to connect to a gmail server securely.
fn main() {
    let mut args = env::args();
    let mut imap_socket;
    args.next();
    if let Some(file) = args.next() {
        imap_socket = parse_file(file);
    } else {
        panic!("Missing file");
    }
    match imap_socket.capability() {
        Ok(capabilities) => {
            for capability in capabilities.iter() {
                println!("{}", capability);
            }
        }
        Err(e) => println!("Error parsing capability: {}", e),
    };

    match imap_socket.select("INBOX") {
        Ok(mailbox) => {
            println!("{}", mailbox);
        }
        Err(e) => println!("Error selecting INBOX: {}", e),
    };

    //imap_socket.create("NEWBOW/SubBox").unwrap();

    match imap_socket.list("/", "*") {
        Ok(a) => {
            for b in a {
                println!("{}", b);
            }
        }
        Err(e) => println!("Error listing: {}", e),
    }


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
