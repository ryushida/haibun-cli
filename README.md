Command Line Application for managing your finances. Built with Rust and PostgreSQL.

# Features

- Add and view expenses/subscriptions
- Manage accounts
- Read CSV of portfolio

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
