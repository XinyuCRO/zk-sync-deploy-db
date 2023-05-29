use sqlx::any::Any;
use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::PgPoolOptions;

use sqlx::migrate::MigrateError;
use sqlx::{Error as SqlxError, Row};

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
    pub url: String,
}

impl Database {
    /// Creates a new `Database` instance with the given connection details.
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

    /// Drops the database 
    pub async fn drop(&self) -> Result<(), DatabaseError> {
        Any::drop_database(&self.url).await.map_err(|e| e.into())
    }

    /// Creates the database 
    pub async fn create(&self) -> Result<(), DatabaseError> {
        Any::create_database(&self.url).await.map_err(|e| e.into())
    }

    /// Migrates the database
    pub async fn migrate(&self) -> Result<(), DatabaseError> {
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

    /// Sets up the database associated with this `Database` instance by creating it and running migrations.
    pub async fn setup(&self) -> Result<(), DatabaseError> {
        self.create().await?;
        self.migrate().await
    }

    /// Prints all tables in the database 
    pub async fn print_all_tables(&self) {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&self.url)
            .await
            .unwrap();

        let rows = sqlx::query(
            r#"
            SELECT * FROM pg_catalog.pg_tables WHERE schemaname != 'pg_catalog' AND schemaname != 'information_schema';
            "#,
        )
        .fetch_all(&pool)
        .await
        .unwrap();

        println!("Tables:");
        for row in rows {
            let table_name: &str = row.get("tablename");
            println!("  {}", table_name);
        }
    }
}
