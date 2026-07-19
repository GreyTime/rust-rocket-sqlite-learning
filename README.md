# Rust Rocket SQLite Learning Project

A small learning project to explore [Rocket](https://rocket.rs/) web framework with SQLite via [rusqlite](https://github.com/rusqlite/rusqlite).

## What it does

- `GET /` — Hello World
- `POST /add_user/<name>/<age>` — Add a user to the database
- `POST /add_login/<username>/<password>` — Add login credentials
- `POST /login/<username>/<password>` — Login with credentials

## Run

```bash
cargo run
```

Server starts at `http://localhost:8000`.
