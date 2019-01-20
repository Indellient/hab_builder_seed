// use base64;
// extern crate sodiumoxide;
extern crate builder_core;
extern crate habitat_builder_protocol;
use std::path::PathBuf;
use crate::builder_core::integrations::{decrypt, encrypt};
use crate::habitat_builder_protocol::{message, originsrv};
// use crate::builder_core::integrations::{decrypt, encrypt, validate};
extern crate postgres;

use postgres::{Connection, TlsMode};
// extern crate percent_encoding;
// use percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
// use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};

// extern crate chrono;
// use chrono::NaiveDateTime;


// use crate::builder_core;
//
// use std::error::Error;
// use sodiumoxide::crypto::box_;
// use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::Nonce;
// use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::PublicKey; // as BoxPublicKey;
// use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::SecretKey; // as BoxSecretKey;
// use sodiumoxide::crypto::sealedbox;
// use crate::habitat_core::crypto::BoxKeyPair;
// use crate::bldr_core::access_token::{BUILDER_ACCOUNT_ID, BUILDER_ACCOUNT_NAME};
//
//
extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let msg = "Hello, world!";
    println!("{}", msg);
    let matches = App::new("BLDR Seed")
                      .version("1.0")
                      .author("Skyler L. <skylerl@indellient.com>")
                      .about("Seed bldr database with auth token and initial user")
                      .arg(Arg::with_name("db-url")
                           .short("u")
                           .long("db-url")
                           .help("Sets database URL")
                           .takes_value(true))
                      .arg(Arg::with_name("db-user")
                           .short("a")
                           .long("db-user")
                           .value_name("FILE")
                           .help("Sets database User")
                           .takes_value(true))
                      .arg(Arg::with_name("db-name")
                           .short("n")
                           .long("db-name")
                           .value_name("FILE")
                           .help("Sets database name")
                           .takes_value(true))
                      .arg(Arg::with_name("db-pass")
                           .short("p")
                           .long("db-pass")
                           .value_name("FILE")
                           .help("Sets database password")
                           .takes_value(true))
                      .subcommand(SubCommand::with_name("seed")
                                  .about("Seeds the database with a user and auth token")
                                  .version("1.0")
                                  .author("Skyler L. <skylerl@indellient.com>")
                                  .arg(Arg::with_name("seed_user")
                                      .help("print debug information verbosely")))
                      .get_matches();




    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let db_url = matches.value_of("db-url").unwrap_or("localhost:5432");
    println!("Value for db_url: {}", db_url);

    let db_user = matches.value_of("db-user").unwrap_or("hab");
    println!("Value for db_user: {}", db_user);

    let db_name = matches.value_of("db-name").unwrap_or("builder");
    println!("Value for db_name: {}", db_name);

    let db_pass = matches.value_of("db-pass").unwrap_or("");
    println!("Value for db_pass: {}", db_pass);

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("seed") {

        let seed_user = matches.value_of("seed_user").unwrap_or("");
        if matches.is_present("seed_user") {
            println!("Seeding with user: {}", seed_user);
        } else {
            println!("Please pass a user to seed with");
        }
    }







    // let token = "_Qk9YLTEKYmxkci0yMDE4MTEyOTE1NTM0MgpibGRyLTIwMTgxMTI5MTU1MzQyClVxMmZUd21naG1hL3p4U2pua25sODVCWTRXNlJtdVdDCnh5RzE4VXpTeGRkNXBKcU5vdmFDZ2hTSEV2Y0Z0b1dHTUFYWXAyUk5OdnFodmxmRw==";
    // let token = "_Qk9YLTEKYmxkci0yMDE4MTEyOTE1NTM0MgpibGRyLTIwMTgxMTI5MTU1MzQyCmlkSjhPU3htQXVGV0lCYjdiaXFuc20zQmxmYUpHT0cxCnhSOEIvQUdVZW9GOFhXbFZsL1N0NlJKenB2SVJGc3pzeS9lV2kxdG9HOW5OWWc9PQ==";
    // extract_info_from_token(token);
    // generate_token();
    // let acc = Account {
    //     name: "sky".to_string(),
    //     email: "".to_string(),
    //     id: 0,
    // };
    // create_user(acc);
}

pub struct Account {
    pub id: i64,
    pub email: String,
    pub name: String,
}

fn create_user(acc: Account) {
    let user = "hab";
    let pass = percent_encode("AlS16jkaGeXfe/ZlwvJ2ghi3IQrrxcqQIvKL9y96DhM=");
    let host = "10.10.1.142";
    let port = 5432;
    let db = "builder";


    let conn = Connection::connect(format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, db), TlsMode::None).unwrap();
    // conn.execute("CREATE TABLE person (
    //                 id              SERIAL PRIMARY KEY,
    //                 name            VARCHAR NOT NULL,
    //                 data            BYTEA
    //               )", &[]).unwrap();
    // let me = Person {
    //     id: 0,
    //     name: "Steven".to_string(),
    //     data: None,
    // };
    let mut person = find_account(&acc, &conn);
    if person.name == "" {
        conn.execute("INSERT INTO accounts (name, email) VALUES ($1, $2)",
                     &[&acc.name, &acc.email]).unwrap();
    }
    person = find_account(&acc, &conn);

    println!("Found person {}: {}", person.id, person.name);
}
fn find_account(acc: &Account, conn: &Connection) -> Account {

    let mut person = Account {
        id: 0,
        name: "".to_string(),
        email: "".to_string(),
    };
    for row in &conn.query("SELECT id, name, email FROM accounts", &[]).unwrap() {
        person = Account {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
        };

        if person.name == acc.name {
            return person;
        }
    }
    return person;
}
fn generate_token() {
    let key_dir = PathBuf::from(r"/Users/skylerl/dev/rust/hello_cargo/keys");
    let account_id = 1162273542817996800;
    let flags = 0;
    let expires = 8210298326400;

    let mut token = originsrv::AccessToken::new();
    token.set_account_id(account_id);
    token.set_flags(flags);
    token.set_expires(expires);

    let bytes = message::encode(&token);
    let ciphertext = encrypt(key_dir, &bytes.unwrap());

    println!("_{}", ciphertext.unwrap());
}

fn extract_info_from_token(token: &str) {

    let key_dir = PathBuf::from(r"/Users/skylerl/dev/rust/hello_cargo/keys");
    let (_, encoded) = token.split_at(1);
    let bytes = decrypt(key_dir, encoded);

    let payload: originsrv::AccessToken = match message::decode(&bytes.unwrap()) {
        Ok(p) => p,
        Err(e) => {
            panic!("Unable to deserialize access token, err={:?}", e);
        }
    };

    println!("Account ID: {}", payload.get_account_id());
    println!("Expires: {}", payload.get_expires());
    println!("Flags {}", payload.get_flags());
}

fn percent_encode(my_str: &str) -> String {
    let mut out: String = "".to_string();

    for c in my_str.chars() {
        let mut add = c.to_string();
        if c == '!' {
            add = "%21".to_string();
        }
        if c == '#' {
            add = "%23".to_string();
        }
        if c == '$' {
            add = "%24".to_string();
        }
        if c == '&' {
            add = "%26".to_string();
        }
        if c == '\'' {
            add = "%27".to_string();
        }
        if c == '(' {
            add = "%28".to_string();
        }
        if c == ')' {
            add = "%29".to_string();
        }
        if c == '*' {
            add = "%2A".to_string();
        }
        if c == '+' {
            add = "%2B".to_string();
        }
        if c == ',' {
            add = "%2C".to_string();
        }
        if c == '/' {
            add = "%2F".to_string();
        }
        if c == ':' {
            add = "%3A".to_string();
        }
        if c == ';' {
            add = "%3B".to_string();
        }
        if c == '=' {
            add = "%3D".to_string();
        }
        if c == '?' {
            add = "%3F".to_string();
        }
        if c == '@' {
            add = "%40".to_string();
        }
        if c == '[' {
            add = "%5B".to_string();
        }
        if c == ']' {
            add = "%5D".to_string();
        }
        out.push_str(&add);
    }
    return out;
}
