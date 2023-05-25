use sqlx::any::Any;
use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::{PgPool, PgQueryResult};

pub struct Database {
    database: String,

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
            database: database.clone(),
            url: format!(
                "postgres://{}:{}@{}:{}/{}",
                username, password, host, port, database
            ),
        }
    }

    pub async fn drop(&self) -> Result<(), sqlx::Error> {
        Any::drop_database(&self.url).await
    }
}
