extern crate directories;
use directories::ProjectDirs;
use r2d2;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, read_to_string, File};
use std::io::Write;
use std::path::Path;
use structopt::StructOpt;
use toml;

mod datetime;
mod interface;
mod sql;

#[derive(StructOpt)]
struct Cli {
    /// view, add
    main: String,

    ///  expense, subscriptions
    sub: String,

    /// Value
    value: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Config {
    database: Database,
}

#[derive(Serialize, Deserialize)]
struct Database {
    ip: String,
    port: i32,
    dbname: String,
    dbuser: String,
    dbpassword: String,
}

fn main() {
    let mut login: Database = Database {
        ip: "".to_string(),
        port: 0,
        dbname: "".to_string(),
        dbuser: "".to_string(),
        dbpassword: "".to_string(),
    };

    if let Some(proj_dirs) = ProjectDirs::from("haibun", "haibun", "haibuncli") {
        let path = proj_dirs.config_dir();
        let config_path = path.join("config.toml");

        // If configuration file does not exist
        if !Path::new(&config_path).exists() {
            let config = Config {
                database: Database {
                    ip: "127.0.0.1".to_string(),
                    port: 5432,
                    dbname: "database_name".to_string(),
                    dbuser: "postgres_user".to_string(),
                    dbpassword: "postgres_password".to_string(),
                },
            };

            let toml = toml::to_string(&config).unwrap();
            create_dir_all(&path).expect("Unable to create path");
            let mut f = File::create(&config_path).expect("Unable to create file");
            f.write_all(toml.as_bytes()).expect("Unable to write data");

            println!("A configuration file has been created at {:?}", config_path);
            println!("Please update the file and re-run");

            // Quit
        }
        // Read postgres config from file if exist
        else {
            let contents = read_to_string(&config_path).expect("Error Reading Config");
            let config: Config = toml::from_str(&contents).unwrap();

            login = Database {
                ip: config.database.ip,
                port: config.database.port,
                dbname: config.database.dbname,
                dbuser: config.database.dbuser,
                dbpassword: config.database.dbpassword,
            };
        }
    }

    let c = format!(
        "host={} port={} dbname={} user={} password={}",
        login.ip, login.port, login.dbname, login.dbuser, login.dbpassword
    );

    let manager = PostgresConnectionManager::new(c.parse().unwrap(), NoTls);
    let pool = r2d2::Pool::new(manager).unwrap();

    // Command line arguments
    let args = Cli::from_args();
    let main = args.main.as_str();
    let sub = args.sub.as_str();

    if main == "view" {
        if sub == "expense" {
            sql::get_expense(pool.clone());
        } else if sub == "subscriptions" {
            sql::get_subscriptions(pool.clone());
        }
    } else if main == "add" {
        if sub == "expense" {
            sql::add_expense(pool.clone());
        }
    }
}
