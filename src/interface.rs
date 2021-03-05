use chrono::NaiveDate;
use comfy_table::presets::ASCII_MARKDOWN;
use comfy_table::*;
use dialoguer::Input;
use postgres::Row;
use rust_decimal::prelude::*;

use crate::datetime;

/// Ask user for input and return entered integer
pub fn user_input_int(displayed_text: &str) -> i32 {
    let value: i32 = Input::new().with_prompt(displayed_text).interact().unwrap();
    value
}

pub fn user_input_float(displayed_text: &str) -> f64 {
    let value: f64 = Input::new().with_prompt(displayed_text).interact().unwrap();
    value
}

pub fn user_input_text(displayed_text: &str) -> String {
    let value: String = Input::new().with_prompt(displayed_text).interact().unwrap();
    value
}

pub fn user_input_date(displayed_text: &str) -> NaiveDate {
    let value: String = Input::new().with_prompt(displayed_text).interact().unwrap();
    let date = datetime::parse_date(&value).unwrap();
    date
}

pub fn print_account_rows(rows: Vec<Row>) {
    for row in rows {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        println!("{} {}", id, name);
    }
}

pub fn expense_rows_to_table(rows: Vec<Row>) -> String {
    let mut table = comfy_table::Table::new();
    table
        .load_preset(ASCII_MARKDOWN)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["id", "Date", "Account", "Amount", "Category", "Notes"]);

    for row in rows {
        let id: i32 = row.get(0);
        let date: NaiveDate = row.get(1);
        let account: &str = row.get(2);
        let amount: Decimal = row.get(3);
        let category: &str = row.get(4);
        let notes: &str = row.get(5);

        table.add_row(vec![
            Cell::new(id),
            Cell::new(date),
            Cell::new(account),
            Cell::new(amount),
            Cell::new(category),
            Cell::new(notes),
        ]);
    }

    table.to_string()
}

pub fn subscription_rows_to_table(rows: Vec<Row>) -> String {
    let mut table = comfy_table::Table::new();
    table
        .load_preset(ASCII_MARKDOWN)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Name", "Category", "Amount"]);

    for row in rows {
        let name: &str = row.get(0);
        let category: &str = row.get(1);
        let amount: Decimal = row.get(2);

        table.add_row(vec![
            Cell::new(name),
            Cell::new(category),
            Cell::new(amount),
        ]);
    }

    table.to_string()
}

pub fn account_rows_to_table(rows: Vec<Row>) -> String {
    let mut table = comfy_table::Table::new();
    table
        .load_preset(ASCII_MARKDOWN)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["id", "Account"]);

    for row in rows {
        let id: i32 = row.get(0);
        let account: &str = row.get(1);

        table.add_row(vec![
            Cell::new(id),
            Cell::new(account),
        ]);
    }

    table.to_string()
}
