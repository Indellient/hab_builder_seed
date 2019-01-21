extern crate clap;
extern crate postgres;
extern crate builder_core;
extern crate habitat_builder_protocol;

use clap::{Arg, App, SubCommand, crate_version};
use postgres::{Connection, TlsMode};
use crate::builder_core::integrations::{decrypt, encrypt};
use crate::habitat_builder_protocol::{message, originsrv};

fn main() {
    let matches = App::new("BLDR Seed")
                      .version(crate_version!())
                      .author("Skyler L. <skylerl@indellient.com>")
                      .about("Seed bldr database with auth token and initial user")
                      .arg(Arg::with_name("db-host")
                           .short("h")
                           .long("db-host")
                           .value_name("DB HOST")
                           .help("Sets database Host")
                           .takes_value(true))
                      .arg(Arg::with_name("db-port")
                           .short("po")
                           .long("db-port")
                           .value_name("DB PORT")
                           .help("Sets database Port")
                           .takes_value(true))
                      .arg(Arg::with_name("db-user")
                           .short("u")
                           .long("db-user")
                           .value_name("DB USERNAME")
                           .help("Sets database User")
                           .takes_value(true))
                      .arg(Arg::with_name("db-name")
                           .short("n")
                           .long("db-name")
                           .value_name("DB NAME")
                           .help("Sets database name")
                           .takes_value(true))
                      .arg(Arg::with_name("db-pass")
                           .short("p")
                           .long("db-pass")
                           .value_name("DB PASSWORD")
                           .help("Sets database password")
                           .takes_value(true))
                      .arg(Arg::with_name("keys-dir")
                           .short("k")
                           .long("keys-dir")
                           .value_name("KEYS DIR")
                           .help("Sets the path to the BLDR Keys")
                           .takes_value(true))
                      .subcommand(SubCommand::with_name("seed")
                                  .about("Seeds the database with a user and auth token")
                                  .version(crate_version!())
                                  .author("Skyler L. <skylerl@indellient.com>")
                                  .arg(Arg::with_name("seed_user")
                                      .help("print debug information verbosely")))
                      .subcommand(SubCommand::with_name("extract")
                                  .about("Extracts information from a token")
                                  .version(crate_version!())
                                  .author("Skyler L. <skylerl@indellient.com>")
                                  .arg(Arg::with_name("token")
                                      .help("print debug information verbosely")))
                      .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let db_host = matches.value_of("db-host").unwrap_or("localhost");
    println!("Value for db_host: {}", db_host);

    let db_port = matches.value_of("db-port").unwrap_or("5432");
    println!("Value for db_port: {}", db_port);

    let db_user = matches.value_of("db-user").unwrap_or("hab");
    println!("Value for db_user: {}", db_user);

    let db_name = matches.value_of("db-name").unwrap_or("builder");
    println!("Value for db_name: {}", db_name);

    let db_pass = matches.value_of("db-pass").unwrap_or("");
    println!("Value for db_pass: {}", db_pass);

    let keys_dir = matches.value_of("keys-dir").unwrap_or("/hab/svc/builder-api/files");
    println!("Value for keys_dir: {}", keys_dir);

    if let Some(matches) = matches.subcommand_matches("seed") {
        let seed_user = matches.value_of("seed_user").unwrap_or("");
        if matches.is_present("seed_user") {
            println!("Seeding with user: {}", seed_user);

            let db = Database {
                host: db_host.to_string(),
                port: db_port.to_string(),
                user: db_user.to_string(),
                name: db_name.to_string(),
                pass: percent_encode(db_pass),
            };

            let acc = Account {
                name: seed_user.to_string(),
                email: "".to_string(),
                id: 0,
            };
            let created_user = create_user(acc, db);
            println!("Successfully Created User {}", created_user.name);

            let generated_token = generate_token(&created_user, keys_dir);
            println!("Successfully Generated Token {}", generated_token);

        } else {
            println!("Please pass a user to seed with");
        }
    } else if let Some(matches) = matches.subcommand_matches("extract") {
        let token = matches.value_of("token").unwrap_or("");
        if matches.is_present("token") {
            extract_info_from_token(token, keys_dir);
        }
    }
}

pub struct Account {
    pub id: i64,
    pub email: String,
    pub name: String,
}

pub struct Database {
    pub host: String,
    pub port: String,
    pub user: String,
    pub name: String,
    pub pass: String,
}

fn create_user(acc: Account, db: Database) -> Account {
    let conn = Connection::connect(format!("postgres://{}:{}@{}:{}/{}", db.user, db.pass, db.host, db.port, db.name), TlsMode::None).unwrap();
    let mut person = find_account(&acc, &conn);
    if person.name == "" {
        conn.execute("INSERT INTO accounts (name, email) VALUES ($1, $2)",
                     &[&acc.name, &acc.email]).unwrap();
    }
    person = find_account(&acc, &conn);

    println!("Found person {}: {}", person.id, person.name);
    return person;
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

fn generate_token(account: &Account, key_dir: &str) -> String {
    // These values are hard coded, not exactly sure what they mean.
    // Extracted from an existing token in the builder database.
    let flags = 0;
    let expires = 8210298326400;

    let mut token = originsrv::AccessToken::new();
    token.set_account_id(account.id as u64);
    token.set_flags(flags);
    token.set_expires(expires);

    let bytes = message::encode(&token);
    let ciphertext = encrypt(key_dir, &bytes.unwrap());

    return ciphertext.unwrap();
}

fn extract_info_from_token(token: &str, key_dir: &str) {
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
