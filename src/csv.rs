
use csv::Reader;
use postgres::{NoTls};
use r2d2_postgres::PostgresConnectionManager;
use regex::Regex;
use rust_decimal::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use crate::datetime;
use crate::interface;
use crate::sql;

pub fn read_csv(pool: r2d2::Pool<PostgresConnectionManager<NoTls>>, path: &str, currency: String, skiprows: usize, stoprows: usize, item_col: usize, value_col: usize) -> Result<(), Box<dyn Error>> {

    println!("{}", path);

    // Get date of csv
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let date_string = re.captures(path).unwrap()[0].to_string();
    let mut date = datetime::parse_date(&date_string).unwrap();
    let confirm_string = [&date.to_string(), " correct?"].join("");
    if !interface::user_input_confirm(&confirm_string) {
        date = interface::user_input_date("Which date is this from?");
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut csv_rows: Vec<String> = Vec::new();
    for line in reader.lines() {
        csv_rows.push(line.unwrap());
    }

    let rows = remove_first_last_rows(csv_rows, skiprows, stoprows);
    let data = rows.join("\n");
    let mut rdr = Reader::from_reader(data.as_bytes());
    
    for result in rdr.records() {
        let record = result?;
        let item = &record[item_col-1];
        let value = &record[value_col-1].replace(&currency, "").parse::<Decimal>().unwrap();
        let exists = sql::check_portfolio(pool.clone(), date, item, value)?;
        if !exists {
            sql::insert_portfolio(pool.clone(), date, item, value)?;
            println!("{} {} {}", date, item, value);
            println!("Added");
        } else {
            println!("Already Exists");
        }
    }

    Ok(())
}

fn remove_first_last_rows(rows_vec: Vec<String>, skiprows: usize, stoprows: usize) -> Vec<String> {
    let mut r: Vec<String> = Vec::new();

    for (num, row) in rows_vec.iter().enumerate() {
        if num >= skiprows && num < rows_vec.len() - stoprows {
            r.push(row.to_string());
        }
    }

    r
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_rows() {
        let v: Vec<String> = vec!["a".to_string(), "b".to_string(),
                                  "c".to_string(), "d".to_string(),
                                  "e".to_string(), "f".to_string()];

        let v2: Vec<String> = vec!["b".to_string(), "c".to_string(),
                                   "d".to_string()];

        let v3: Vec<String> = vec!["a".to_string(), "b".to_string(),
                                   "c".to_string(), "d".to_string()];

        let v4: Vec<String> = vec!["c".to_string(), "d".to_string(),
                                  "e".to_string(), "f".to_string()];

        assert_eq!(v, remove_first_last_rows(v.clone(), 0, 0));
        assert_eq!(v2, remove_first_last_rows(v.clone(), 1, 2));
        assert_eq!(v3, remove_first_last_rows(v.clone(), 0, 2));
        assert_eq!(v4, remove_first_last_rows(v.clone(), 2, 0));
    }
}
