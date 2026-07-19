# Rust Rocket SQLite Learning Project

A small learning project to explore [Rocket](https://rocket.rs/) web framework with SQLite via [rusqlite](https://github.com/rusqlite/rusqlite).

## What it does

- `GET /` — Hello World
- `POST /add_user/<name>/<age>` — Add a user to the database
- `POST /add_login/<username>/<password>` — Add login credentials
- `POST /login/<username>/<password>` — Login with credentials

## What I learned

- Rocket basics (routes, state, path parameters)
- SQLite with rusqlite (CRUD operations)
- Error handling with `Result`, `?` operator, and `map_err`
- Mutex for thread-safe database access
- Passwords should be hashed (not stored in plaintext!)

## Run

```bash
cargo run
```

Server starts at `http://localhost:8000`.
