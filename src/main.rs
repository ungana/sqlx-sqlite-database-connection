use database::Database;

mod database;

#[tokio::main]
async fn main() {
    match Database::connect().await {
        Ok(_pool) => {
            println!("Hello, world!");
        }
        Err(e) => panic!("Unable to connect to database: {}", e),
    }
}
