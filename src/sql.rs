use chrono::NaiveDate;
use postgres::{Error, NoTls, Row};
use r2d2;
use r2d2_postgres::PostgresConnectionManager;
use rust_decimal::prelude::*;

pub fn get_account_ids(
    pool: r2d2::Pool<PostgresConnectionManager<NoTls>>,
) -> Result<Vec<Row>, Error> {
    let mut client: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager<NoTls>> =
        pool.get().unwrap();

    let rows = client.query("SELECT account_id, account_name FROM account", &[])?;

    Ok(rows)
}

pub fn get_account_values(
    pool: r2d2::Pool<PostgresConnectionManager<NoTls>>,
) -> Result<Vec<Row>, Error> {
    let mut client: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager<NoTls>> =
        pool.get().unwrap();

    let rows = client.query(
        "SELECT account.account_id, account_name, coalesce(account_value.account_value, 0)
                             FROM account
                             LEFT JOIN account_value
                             ON account.account_id = account_value.account_id
                             ORDER BY account_id",
        &[],
    )?;

    Ok(rows)
}

pub fn get_expense_categories(
    pool: r2d2::Pool<PostgresConnectionManager<NoTls>>,
) -> Result<Vec<Row>, Error> {
    let mut client: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager<NoTls>> =
        pool.get().unwrap();

    let rows = client.query(
        "SELECT category_id, category_name FROM expense_category",
        &[],
    )?;

    Ok(rows)
}

pub fn get_expense(pool: r2d2::Pool<PostgresConnectionManager<NoTls>>) -> Result<Vec<Row>, Error> {
    let mut client: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager<NoTls>> =
        pool.get().unwrap();

    let q = "SELECT expense.expense_id, expense.date,
             account.account_name, to_char(expense.amount, '999999999.00'),
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
                             account.account_name, to_char(expense.amount, '999999999.00'),
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
        "SELECT subscription.subscription_name, expense_category.category_name,
                subscription.subscription_price
         FROM subscription
         JOIN expense_category
         ON subscription.category_id = expense_category.category_id",
        &[],
    )?;

    Ok(rows)
}

pub fn add_expense(
    pool: r2d2::Pool<PostgresConnectionManager<NoTls>>,
    date: NaiveDate,
    account_id: i32,
    expense_value: Decimal,
    category_id: i32,
    note: String,
) -> Result<(), Error> {
    let mut client: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager<NoTls>> =
        pool.get().unwrap();

    client.execute(
        "INSERT INTO expense (expense_id, date, account_id, amount, category_id, note)
        VALUES (DEFAULT, $1, $2, $3, $4, $5)",
        &[&date, &account_id, &expense_value, &category_id, &note],
    )?;

    Ok(())
}

pub fn add_subscription(
    pool: r2d2::Pool<PostgresConnectionManager<NoTls>>,
    subscription_name: String,
    category_id: i32,
    expense_value: Decimal,
) -> Result<(), Error> {
    let mut client: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager<NoTls>> =
        pool.get().unwrap();

    client.execute(
        "INSERT INTO subscription (subscription_id, subscription_name, category_id, subscription_price)
        VALUES (DEFAULT, $1, $2, $3)",
        &[&subscription_name, &category_id, &expense_value],
    )?;

    Ok(())
}

pub fn update_account_value(
    pool: r2d2::Pool<PostgresConnectionManager<NoTls>>,
    value: Decimal,
    id: i32,
) -> Result<u64, Error> {
    let mut client: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager<NoTls>> =
        pool.get().unwrap();

    let rows_updated = client.execute(
        "UPDATE account_value SET account_value = $1 WHERE account_id = $2",
        &[&value, &id],
    )?;

    Ok(rows_updated)
}

pub fn get_portfolio_sum(pool: r2d2::Pool<PostgresConnectionManager<NoTls>>) -> Result<Row, Error> {
    let mut client: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager<NoTls>> =
        pool.get().unwrap();

    let rows = client.query_one(
        "SELECT  0 as id, 'Total' as item, SUM(value), '' as proportion
     FROM portfolio
     WHERE date = (select max (date) from  portfolio)",
        &[],
    )?;

    Ok(rows)
}

pub fn get_portfolio(
    pool: r2d2::Pool<PostgresConnectionManager<NoTls>>,
) -> Result<Vec<Row>, Error> {
    let mut client: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager<NoTls>> =
        pool.get().unwrap();

    let rows = client.query(
        "SELECT portfolio_id, item, value, to_char(100 * (value / sum(value) over ()), '990D99%') as proportion
         FROM portfolio
         WHERE date = (select max (date) from  portfolio)
         ORDER BY value DESC",
        &[],
    )?;

    Ok(rows)
}

pub fn check_portfolio(
    pool: r2d2::Pool<PostgresConnectionManager<NoTls>>,
    date: NaiveDate,
    item: &str,
    value: &Decimal,
) -> Result<bool, Error> {
    let mut client: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager<NoTls>> =
        pool.get().unwrap();

    let rows = client.query_one(
        "SELECT COUNT(*) > 0
                                 FROM portfolio
                                 WHERE date = $1 AND item = $2 AND value = $3",
        &[&date, &item, &value],
    );

    let mut exists = false;
    for row in rows {
        exists = row.get(0);
    }

    Ok(exists)
}

pub fn insert_portfolio(
    pool: r2d2::Pool<PostgresConnectionManager<NoTls>>,
    date: NaiveDate,
    item: &str,
    value: &Decimal,
) -> Result<(), Error> {
    let mut client: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager<NoTls>> =
        pool.get().unwrap();

    client.execute(
        "INSERT INTO portfolio (portfolio_id, date, item, value)
               VALUES (DEFAULT, $1, $2, $3)",
        &[&date, &item, &value],
    )?;

    Ok(())
}
