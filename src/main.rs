extern crate directories;
use directories::ProjectDirs;
use postgres::Row;
use r2d2;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use toml;

mod csv;
mod datetime;
mod interface;
mod sql;

#[derive(StructOpt)]
pub struct Opts {
    /// expense, subscription, portfolio, account
    main: String,

    // SUBCOMMAND
    #[structopt(subcommand)]
    subcommand: Option<Sub>,
}

#[derive(StructOpt)]
enum Sub {
    /// View items
    #[structopt(name = "view")]
    View(ViewOpts),

    /// Add new
    #[structopt(name = "add")]
    Add(AddOpts),

    /// Configure
    #[structopt(name = "manage")]
    Manage(ManageOpts),
}

#[derive(StructOpt, Debug)]
struct ViewOpts {
    /// Specify Number of Items to Display
    #[structopt(short)]
    number: Option<String>,
}

#[derive(StructOpt, Debug)]
struct AddOpts {
    /// Read CSV
    #[structopt(short)]
    file: Option<String>,
}

#[derive(StructOpt, Debug)]
struct ManageOpts {}

#[derive(Serialize, Deserialize)]
struct Config {
    database: Database,
    csv: Csv,
}

#[derive(Serialize, Deserialize)]
struct Database {
    ip: String,
    port: i32,
    dbname: String,
    dbuser: String,
    dbpassword: String,
}

#[derive(Serialize, Deserialize)]
struct Csv {
    currency: String,
    skiprows: usize,
    stoprows: usize,
    item_column: usize,
    value_column: usize,
}

fn main() {
    let mut login: Database = Database {
        ip: "".to_string(),
        port: 0,
        dbname: "".to_string(),
        dbuser: "".to_string(),
        dbpassword: "".to_string(),
    };

    let mut csv: Csv = Csv {
        currency: "".to_string(),
        skiprows: 0,
        stoprows: 0,
        item_column: 1,
        value_column: 2,
    };

    if let Some(proj_dirs) = ProjectDirs::from("haibun", "haibun", "haibun") {
        let path = proj_dirs.config_dir();
        let config_path = path.join("config.toml");

        // If configuration file does not exist
        if !Path::new(&config_path).exists() {
            create_config(&path, &config_path);
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

            csv = Csv {
                currency: config.csv.currency,
                skiprows: config.csv.skiprows,
                stoprows: config.csv.stoprows,
                item_column: config.csv.item_column,
                value_column: config.csv.value_column,
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
    let args = Opts::from_args();

    if args.main == "expense" {
        if let Some(subcommand) = args.subcommand {
            match subcommand {
                Sub::View(opt) => {
                    if opt.number.is_none() {
                        let table_vec: Vec<Row> = sql::get_expense(pool.clone()).unwrap();
                        let table_string = interface::expense_rows_to_table(table_vec);
                        println!("{}", table_string);
                    } else {
                        let count = opt.number.as_deref().unwrap().parse::<i32>().unwrap();

                        if count > 0 {
                            let table_vec: Vec<Row> =
                                sql::get_expense_num(pool.clone(), &i64::from(count)).unwrap();
                            let table_string = interface::expense_rows_to_table(table_vec);
                            println!("{}", table_string);
                        }
                    }
                }
                Sub::Add(_opt) => {
                    interface::add_expense_prompt(pool.clone());
                }
                Sub::Manage(_opt) => {
                    unimplemented!();
                }
            }
        } else {
            unimplemented!();
        }
    } else if args.main == "subscription" {
        if let Some(subcommand) = args.subcommand {
            match subcommand {
                Sub::View(_opt) => {
                    let table_vec: Vec<Row> = sql::get_subscriptions(pool.clone()).unwrap();
                    let table_string = interface::subscription_rows_to_table(table_vec);
                    println!("{}", table_string);
                }
                Sub::Add(_opt) => {
                    interface::add_subscription_prompt(pool.clone());
                }
                Sub::Manage(_opt) => {
                    unimplemented!();
                }
            }
        } else {
            let table_vec: Vec<Row> = sql::get_subscriptions(pool.clone()).unwrap();
            let table_string = interface::subscription_rows_to_table(table_vec);
            println!("{}", table_string);
        }
    } else if args.main == "account" {
        if let Some(subcommand) = args.subcommand {
            match subcommand {
                Sub::View(_opt) => {
                    let table_vec: Vec<Row> = sql::get_account_values(pool.clone()).unwrap();
                    let table_string = interface::account_values_to_table(table_vec);
                    println!("{}", table_string);
                }
                Sub::Add(_opt) => {
                    unimplemented!();
                }
                Sub::Manage(_opt) => {
                    interface::update_account_values(pool.clone());
                }
            }
        } else {
            unimplemented!();
        }
    } else if args.main == "portfolio" {
        if let Some(subcommand) = args.subcommand {
            match subcommand {
                Sub::View(_opt) => {
                    let mut table_vec: Vec<Row> = sql::get_portfolio(pool.clone()).unwrap();
                    let table_vec_sum: Row = sql::get_portfolio_sum(pool.clone()).unwrap();
                    table_vec.push(table_vec_sum);
                    let table_string = interface::portfolio_rows_to_table(table_vec);
                    println!("{}", table_string);
                }
                Sub::Add(opt) => {
                    let dir = env::current_dir().unwrap();
                    let path = dir.join(opt.file.unwrap().replace(".\\", ""));
                    csv::read_csv(
                        pool.clone(),
                        path.to_str().unwrap(),
                        csv.currency,
                        csv.skiprows,
                        csv.stoprows,
                        csv.item_column,
                        csv.value_column,
                    )
                    .expect("Could not add from csv");
                }
                Sub::Manage(_opt) => {
                    unimplemented!();
                }
            }
        } else {
            unimplemented!();
        }
    }
}

fn create_config(path: &Path, config_path: &PathBuf) {
    let config = Config {
        database: Database {
            ip: "127.0.0.1".to_string(),
            port: 5432,
            dbname: "database_name".to_string(),
            dbuser: "postgres_user".to_string(),
            dbpassword: "postgres_password".to_string(),
        },
        csv: Csv {
            currency: "".to_string(),
            skiprows: 0,
            stoprows: 0,
            item_column: 1,
            value_column: 2,
        },
    };

    let toml = toml::to_string(&config).unwrap();
    create_dir_all(&path).expect("Unable to create path");
    let mut f = File::create(&config_path).expect("Unable to create file");
    f.write_all(toml.as_bytes()).expect("Unable to write data");

    println!("A configuration file has been created at {:?}", config_path);
    println!("Please update the file and re-run");
}
