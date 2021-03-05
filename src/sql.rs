use chrono::NaiveDate;
use postgres::{Error, NoTls, Row};
use r2d2;
use r2d2_postgres::PostgresConnectionManager;
use rust_decimal::prelude::*;

use crate::interface;

pub fn get_account_ids(pool: r2d2::Pool<PostgresConnectionManager<NoTls>>) -> Result<Vec<Row>, Error> {
    let mut client: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager<NoTls>> =
        pool.get().unwrap();

    let rows = client.query("SELECT account_id, account_name FROM account", &[])?;

    Ok(rows)
}

fn get_expense_categories(pool: r2d2::Pool<PostgresConnectionManager<NoTls>>) -> Result<(), Error> {
    let mut client: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager<NoTls>> =
        pool.get().unwrap();

    for row in client.query(
        "SELECT category_id, category_name FROM expense_category",
        &[],
    )? {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        println!("{} {}", id, name);
    }

    Ok(())
}

pub fn get_expense(pool: r2d2::Pool<PostgresConnectionManager<NoTls>>) -> Result<Vec<Row>, Error> {
    let mut client: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager<NoTls>> =
        pool.get().unwrap();

    let q = "SELECT expense.expense_id, expense.date,
                         account.account_name, expense.amount,
                         expense_category.category_name, expense.note
                  FROM expense
                  LEFT JOIN expense_category
                  ON expense.category_id = expense_category.category_id
                  LEFT JOIN account
                  ON expense.account_id = account.account_id
                  ORDER BY date";

    let rows = client.query(q, &[])?;

    Ok(rows)
}

pub fn get_expense_num(
    pool: r2d2::Pool<PostgresConnectionManager<NoTls>>,
    n: i64,
) -> Result<Vec<Row>, Error> {
    let mut client: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager<NoTls>> =
        pool.get().unwrap();

    // Get last n expense
    let q = "WITH t AS (
                    SELECT expense.expense_id, expense.date,
                            account.account_name, expense.amount,
                            expense_category.category_name, expense.note
                    FROM expense
                    LEFT JOIN expense_category
                    ON expense.category_id = expense_category.category_id
                    LEFT JOIN account
                    ON expense.account_id = account.account_id
                    ORDER BY date
                    DESC LIMIT $1
                )
                SELECT * FROM t ORDER BY date ASC;";

    let rows = client.query(q, &[&n])?;

    Ok(rows)
}

pub fn get_subscriptions(
    pool: r2d2::Pool<PostgresConnectionManager<NoTls>>,
) -> Result<Vec<Row>, Error> {
    let mut client: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager<NoTls>> =
        pool.get().unwrap();

    let rows = client.query(
        "SELECT subscription.subscription_name,
                                            expense_category.category_name,
                                            subscription.subscription_price
                                     FROM subscription
                                     JOIN expense_category
                                     ON subscription.category_id = expense_category.category_id",
        &[],
    )?;

    Ok(rows)
}

pub fn add_expense(pool: r2d2::Pool<PostgresConnectionManager<NoTls>>) -> Result<(), Error> {
    let mut client: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager<NoTls>> =
        pool.get().unwrap();

    let date = interface::user_input_date("Enter date");

    let accounts_vec: Vec<Row> = get_account_ids(pool.clone()).unwrap();
    interface::print_account_rows(accounts_vec);
    let account_id = interface::user_input_int("Enter Number");

    let expense_input = interface::user_input_float("Enter Amount");
    let expense_value: Decimal = Decimal::from_str(&expense_input.to_string()).unwrap();

    get_expense_categories(pool.clone())?;
    let category_id = interface::user_input_int("Enter number");

    let note = interface::user_input_text("Note");

    client.execute(
        "INSERT INTO expense (expense_id, date, account_id, amount, category_id, note)
                   VALUES (DEFAULT, $1, $2, $3, $4, $5)",
        &[&date, &account_id, &expense_value, &category_id, &note],
    )?;

    Ok(())
}
