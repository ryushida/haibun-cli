use chrono::NaiveDate;
use comfy_table::presets::ASCII_MARKDOWN;
use comfy_table::*;
use dialoguer::{Confirm, Input};
use postgres::{NoTls, Row};
use r2d2_postgres::PostgresConnectionManager;
use rust_decimal::prelude::*;

use crate::datetime;
use crate::sql;

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

pub fn user_input_confirm(displayed_text: &str) -> bool {
    let mut proceed = false;
    if Confirm::new()
        .with_prompt(displayed_text)
        .interact()
        .expect("msg")
    {
        proceed = true;
    };
    proceed
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
        let amount: &str = row.get(3);
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

pub fn expense_category_rows_to_table(rows: Vec<Row>) -> String {
    let mut table = comfy_table::Table::new();
    table
        .load_preset(ASCII_MARKDOWN)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["id", "Category"]);

    for row in rows {
        let id: i32 = row.get(0);
        let category: String = row.get(1);

        table.add_row(vec![Cell::new(id), Cell::new(category)]);
    }

    table.to_string()
}

pub fn portfolio_rows_to_table(rows: Vec<Row>) -> String {
    let mut table = comfy_table::Table::new();
    table
        .load_preset(ASCII_MARKDOWN)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["ID", "Item", "Value", "Proportion"]);

    for row in rows {
        let id: i32 = row.get(0);
        let item: &str = row.get(1);
        let value: Decimal = row.get(2);
        let proportion: &str = row.get(3);

        table.add_row(vec![
            Cell::new(id),
            Cell::new(item),
            Cell::new(value),
            Cell::new(proportion),
        ]);
    }

    table.to_string()
}

pub fn subscription_rows_to_table(rows: Vec<Row>) -> String {
    let mut table = comfy_table::Table::new();
    table
        .load_preset(ASCII_MARKDOWN)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Name", "Category", "Yearly", "Monthly"]);

    for row in rows {
        let name: &str = row.get(0);
        let category: &str = row.get(1);
        let yearly: Decimal = row.get(2);
        let monthly: &str = row.get(3);

        table.add_row(vec![
            Cell::new(name),
            Cell::new(category),
            Cell::new(yearly),
            Cell::new(monthly),
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

        table.add_row(vec![Cell::new(id), Cell::new(account)]);
    }

    table.to_string()
}

pub fn account_values_to_table(rows: &Vec<Row>) -> String {
    let mut table = comfy_table::Table::new();
    table
        .load_preset(ASCII_MARKDOWN)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["id", "Account", "Value"]);

    for row in rows {
        let id: i32 = row.get(0);
        let account: &str = row.get(1);
        let value: Decimal = row.get(2);

        table.add_row(vec![Cell::new(id), Cell::new(account), Cell::new(value)]);
    }

    table.to_string()
}

fn expense_category_table(pool: r2d2::Pool<PostgresConnectionManager<NoTls>>) -> String {
    let expense_vec: Vec<Row> = sql::get_expense_categories(pool.clone()).unwrap();
    let expense_table_string = expense_category_rows_to_table(expense_vec);
    expense_table_string
}

pub fn add_expense_prompt(pool: r2d2::Pool<PostgresConnectionManager<NoTls>>) {
    let date = user_input_date("Enter date");

    let table_vec: Vec<Row> = sql::get_account_ids(pool.clone()).unwrap();
    let table_string = account_rows_to_table(table_vec);
    println!("{}", table_string);

    let account_id = user_input_int("Enter ID");

    let expense_input = user_input_float("Enter Amount");
    let expense_value: Decimal = Decimal::from_str(&expense_input.to_string()).unwrap();

    println!("{}", expense_category_table(pool.clone()));
    let category_id = user_input_int("Enter number");

    let note = user_input_text("Note");

    sql::add_expense(
        pool.clone(),
        &date,
        &account_id,
        &expense_value,
        &category_id,
        note,
    )
    .expect("Could not add");
}

pub fn add_subscription_prompt(pool: r2d2::Pool<PostgresConnectionManager<NoTls>>) {
    let subscription_name = user_input_text("Subscription Name");

    println!("{}", expense_category_table(pool.clone()));
    let category_id = user_input_int("Enter number");

    let price_input = user_input_float("Price");
    let subscription_price: Decimal = Decimal::from_str(&price_input.to_string()).unwrap();

    sql::add_subscription(
        pool.clone(),
        subscription_name,
        category_id,
        subscription_price,
    )
    .expect("Could not add");
}

pub fn update_account_values(pool: r2d2::Pool<PostgresConnectionManager<NoTls>>) {
    let table_vec: Vec<Row> = sql::get_account_values(pool.clone()).unwrap();
    let table_string = account_values_to_table(&table_vec);
    println!("{}", table_string);

    let id = user_input_int("ID of Account to Update");

    let value = user_input_float("New Value");
    let value_decimal = Decimal::from_f64(value).unwrap();

    let rows_updated =
        sql::update_account_value(pool.clone(), &value_decimal, &id).expect("Problem Updating");

    println!("{} rows updated", rows_updated);

    let table_vec: Vec<Row> = sql::get_account_values(pool.clone()).unwrap();
    let table_string = account_values_to_table(&table_vec);
    println!("{}", table_string);
}

pub fn account_types_to_table(rows: &Vec<Row>) -> String {
    let mut table = comfy_table::Table::new();
    table
        .load_preset(ASCII_MARKDOWN)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["id", "Account Type"]);

    for row in rows {
        let id: i32 = row.get(0);
        let account_type: &str = row.get(1);

        table.add_row(vec![Cell::new(id), Cell::new(account_type)]);
    }

    table.to_string()
}

pub fn add_account_prompt(pool: r2d2::Pool<PostgresConnectionManager<NoTls>>) {

    let account_name = user_input_text("Account Name");

    println!("{}", account_type_table(pool.clone()));
    let account_type_id = user_input_int("Enter number");

    let value_input = user_input_float("Account Value");
    let account_value: Decimal = Decimal::from_str(&value_input.to_string()).unwrap();

    sql::add_account(
        pool.clone(),
        account_name,
        account_type_id,
        account_value,
    )
    .expect("Could not add");
}

fn account_type_table(pool: r2d2::Pool<PostgresConnectionManager<NoTls>>) -> String {
    let account_type_vec: Vec<Row> = sql::get_account_types(pool.clone()).unwrap();
    let table_string = account_types_to_table(&account_type_vec);
    table_string
}
