# Rust Actix - Grocery list API

## Introduction

This project is a simple Grocery list REST API built in [Rust](https://www.rust-lang.org/) and the [actix_web](https://docs.rs/actix-web/3.3.2/actix_web/index.html) framework.

The API allows users to register, login, and create/edit lists.

## Running locally

To run locally, clone this repo. Run `chmod +x ./scripts/*` and then run `scripts/start.sh`.

The api should be accepting requests at `http://localhost:8080/`. You can check the health at `/health`.

## Database

The REST API uses Sqlite. By default, an in memory database is used. If you want to use a database file, either set the `database_type` setting in `Settings.toml` or the env variable `DATABASE_TYPE` to `file`. If you run using the `file` database type, a `database.db` file will automatically be created in `./data/`.

## Migrations

The `sqlx-cli` provides simple database migrations. The following utility scripts can be used for migrating the database (note: you should run the app at least once with `database_type` set to `file` so that the database file can be created).

+ `./scripts/add_migration.sh <name of migration>` - creates a new reverable migration
+ `./scripts/migrate.sh` - migrates the database
+ `./scripts/revert.sh` - reverts the last migration

## Auth

All routes besides `/health`, `/user/login`, and `/user/register` are protected using JWT authentication. To get a valid JWT, you must first register by doing a `post` request to either `/user/register`, which creates a new user and returns a valid JWT, or `/user/login`, which logs in with an existing username and password and returns a valid JWT. Send the JWT in an Authorization header with all subsequent requests.

## Tests

Run `cargo test` to run all unit and integration tests.

## Improvements

Some future improvements that could be added:

+ Switch to Postgres/docker instead of sqlite
+ Use short lived JWT tokens and long lived refresh cookies instead of long lived JWT for better security
+ Add swagger documentation for endpoints
+ Add better migration utility

