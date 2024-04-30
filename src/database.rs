use sqlx::{
    migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Error as SqlXError, Sqlite, SqlitePool,
};

pub struct Database {}

impl Database {
    pub async fn connect() -> Result<SqlitePool, SqlXError> {
        Self::setup_database().await?;
        match SqlitePoolOptions::new().connect("./example.db").await {
            Ok(pool) => {
                Self::migrate_database(&pool).await?;
                Ok(pool)
            }
            Err(_) => panic!("Unable to connect to database."),
        }
    }

    async fn migrate_database(pool: &SqlitePool) -> Result<(), SqlXError> {
        sqlx::migrate!().run(pool).await?;
        Ok(())
    }

    async fn setup_database() -> Result<(), SqlXError> {
        match Sqlite::database_exists("./example.db")
            .await
            .unwrap_or(false)
        {
            true => Ok(()),
            false => match Sqlite::create_database("./example.db").await {
                Ok(_) => Ok(()),
                Err(_) => panic!("Unable to create database."),
            },
        }
    }
}
