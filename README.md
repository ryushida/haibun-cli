Command Line Application for managing your finances. Built with Rust and PostgreSQL.

# Features

- Add and view expenses/subscriptions
- Manage accounts
- Read CSV of portfolio

# Set up

1. Start PostgreSQL server

2. Create database and tables

```shell
psql postgresuser
CREATE DATABASE database_name;
\c databasename postgres_user
\i init.sql

\l
\dt
```
