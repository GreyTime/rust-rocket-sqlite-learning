#[macro_use]
extern crate rocket;

use rocket::State;
use rusqlite::{Connection, params};
use std::sync::Mutex;

struct Db {
    conn: Mutex<Connection>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/add_user/<name>/<age>")]
fn add_user(name: &str, age: u8, db: &State<Db>) -> Result<String, String> {
    let conn = db.conn.lock().expect("failed to lock db");

    conn.execute("INSERT INTO users (name, age) VALUES (?1, ?2)", (name, age))
        .map_err(|e| e.to_string())?;

    Ok(format!("User {} was added", name))
}

#[post("/add_login/<username>/<password>")]
fn add_login(username: &str, password: &str, db: &State<Db>) -> Result<String, String> {
    let conn = db.conn.lock().expect("failed to lock db");

    conn.execute(
        "INSERT INTO credentials (username, password) VALUES (?1, ?2)",
        (username, password),
    )
    .map_err(|e| e.to_string())?;

    Ok(format!("login with username: {} was added", username))
}

#[post("/login/<username>/<password>")]
fn login(username: &str, password: &str, db: &State<Db>) -> Result<String, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let password_result: Result<String, rusqlite::Error> = conn.query_one(
        "SELECT password FROM credentials WHERE username = ?1",
        params![username],
        |row| row.get(0),
    );

    match password_result {
        Ok(password_result) => {
            if password_result == password {
                Ok("login is possible".to_string())
            } else {
                Err("password is wrong".to_string())
            }
        }
        Err(e) => {
            eprint!("{}", e);
            Err("Fehler".to_string())
        }
    }
}

#[launch]
fn rocket() -> _ {
    let conn = Connection::open("users.db").expect("couldn't open users db");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            name TEXT NOT NULL,
            age INTEGER NOT NULL
        )",
        [],
    )
    .expect("failed to create users table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS credentials (
            username TEXT NOT NULL,
            password text NOT NULL
        )",
        [],
    )
    .expect("failed to create credentials table");

    rocket::build()
        .manage(Db {
            conn: Mutex::new(conn),
        })
        .mount("/", routes![index, add_user, add_login, login])
}
