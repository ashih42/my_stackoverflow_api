# My StackOverflow API (Rocket)

This is a simple [StackOverflow](https://stackoverflow.com/)-like API server, which talks to a Postgres database server and returns data over HTTP messages through a REST API.

This project includes 2 implementations in Rust, one built with [Rocket](https://rocket.rs/) framework, another built with [Axum](https://github.com/tokio-rs/axum) framework.

## What This App Does

This app models 2 entities:

- `Question`
- `Answer`

A `Question` may have 0 or more `Answers`.

In this app, you can perform CRUD operations on Question and Answer entities with HTTP methods.

## How to Run

Start the server with Rocket framework.

```
cargo run --bin server_rocket
```

Start the server with Axum framework.

```
cargo run --bin server_axum
```

## REST API Documentation

See [HERE](https://documenter.getpostman.com/view/8662105/2sBXwtp8ws).

## Developer Notes

### Entity Relationship Diagram

![Entity Relationship Diagram](./doc/entity_relationship_diagram.png)

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
