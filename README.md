# My StackOverflow API (Rocket)

This is a simple [StackOverflow](https://stackoverflow.com/)-like API server built using the [Rocket](https://rocket.rs/) framework in Rust, talking to a Postgres database server, and returning the results over HTTP messages.

## What This App Does

This app models 2 entities:

- `Question`
- `Answer`

A `Question` may have 0 or more `Answers`.

In this app, you can perform CRUD operations on Question and Answer entities.

## HTTP API Documentation

TODO

## Developer Notes

### Entity Relation Diagram

TODO

### How to Set Up a Local Postgres Server

#### 1. Run a Postgres server in Docker.

```
$ docker run --name stack-overflow-db -e POSTGRES_PASSWORD=postgrespw -p 55008:5432 -d postgres
```

#### 2. Create a `.env` file containing the postgres URL.

```
DATABASE_URL=postgres://postgres:postgrespw@localhost:55008
```

#### 3. Set up tables in database.

Install [sqlx-cli](https://github.com/transact-rs/sqlx/tree/main/sqlx-cli).

```
cargo install sqlx-cli
```

Apply database migrations.

```
sqlx migrate run
```
