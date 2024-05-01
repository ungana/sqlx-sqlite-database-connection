#[macro_use]
extern crate dotenv_codegen;

use database::Database;
use dotenv::dotenv;
use sqlx::{Error as SqlXError, SqlitePool};

mod database;

async fn add_user(pool: SqlitePool) -> Result<(), SqlXError> {
    let query = "INSERT INTO Users (first_name, last_name, email) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind("John")
        .bind("Doe")
        .bind("john.doe@example.com")
        .execute(&pool)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    match Database::connect().await {
        Ok(pool) => match add_user(pool).await {
            Ok(_) => println!("User created."),
            Err(e) => panic!("Unable to create user: {}", e),
        },
        Err(e) => panic!("Unable to connect to database: {}", e),
    }
}
