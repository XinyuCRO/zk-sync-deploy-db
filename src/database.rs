use sqlx::any::Any;
use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::PgPoolOptions;

use sqlx::migrate::MigrateError;
use sqlx::Error as SqlxError;

pub enum DatabaseError {
    Sqlx(SqlxError),
    Migrate(MigrateError),
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DatabaseError::Sqlx(e) => write!(f, "Sqlx error: {}", e),
            DatabaseError::Migrate(e) => write!(f, "Migrate error: {}", e),
        }
    }
}

impl From<SqlxError> for DatabaseError {
    fn from(error: SqlxError) -> Self {
        DatabaseError::Sqlx(error)
    }
}

impl From<MigrateError> for DatabaseError {
    fn from(error: MigrateError) -> Self {
        DatabaseError::Migrate(error)
    }
}

pub struct Database {
    url: String,
}

impl Database {
    pub fn new(
        username: String,
        password: String,
        host: String,
        port: String,
        database: String,
    ) -> Database {
        Database {
            url: format!(
                "postgres://{}:{}@{}:{}/{}",
                username, password, host, port, database
            ),
        }
    }

    pub async fn drop(&self) -> Result<(), DatabaseError> {
        Any::drop_database(&self.url).await.map_err(|e| e.into())
    }

    pub async fn setup(&self) -> Result<(), DatabaseError> {
        Any::create_database(&self.url).await?;

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&self.url)
            .await
            .unwrap();

        println!("Running migrations...");
        sqlx::migrate!("./schema/migrations")
            .run(&pool)
            .await
            .map_err(|e| e.into())
    }
}
