# Simple RUST CRUD API

This is a VERY simple crud api written in rust using the axum framework, SQLx, and Postgres.
This is like a really small base on how to start a rust api.

## Dependencies used

- Tokio - Async runtime
- Axum - Web framework
- Serde - Serialization
- SQLx - Database
- Tracing, Tracing-subscriber, tower-http - Logging
- Hyper, Tower - HTTP client
- Testcontainers & Axum test - Testing

## Running tests

```bash
cargo test # Runs the tests
```

## Running API

```bash
docker build -f deployment/Dockerfile -t rust-api .
docker compose -f deployment/docker-compose.yml up

# to stop
docker compose -f deployment/docker-compose.yml down
```

## Database

The database is handled by Rust's SQLx library. The library has built-in support for migrations and follows the pattern: `<number>_<description>.sql`, where the numbers are run ordered and is run during startup.

## Environment variables

```sh
DATABASE_URL="postgres://postgres:mysecretpassword@localhost:5432/postgres"
PORT="3000"
```

## Project structure

```sh
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── deployment                  # Deployment files
│   ├── Dockerfile              # Dockerfile for the api
│   └── docker-compose.yml      # Docker compose file for the api and database
├── migrations                  # Database migrations
│   └── 1_init.sql
├── src
│   ├── api                     # API endpoints
```
