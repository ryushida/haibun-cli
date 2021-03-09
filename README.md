# Haibun

Haibun is a command line application for managing your finances. Built with Rust and PostgreSQL.

## Features

- Add and view expenses/subscriptions
- Manage accounts
- Import CSV of portfolio

# Set up

1. Start PostgreSQL server

2. Create database and tables

```shell
psql postgres_user
CREATE DATABASE haibun;
\c haibun postgres_user
\i init.sql

\l
\dt
```

3. Run Haibun. A config file should be created.

4. Update config

Default config:
```
[database]
ip = "127.0.0.1"
port = 5432
dbname = "haibun"
dbuser = "postgres_user"
dbpassword = "postgres_password"

[csv]
currency = "$"
skiprows = 0
stoprows = 0
item_column = 1
value_column = 2
```

`skiprows` is the number of rows to skip when reading the csv and `stoprows` is the number of rows to stop before at the end.

`item_column` is the column number of the column in the csv with the names of each item, and `value_column` is the column number with the values of each item.
