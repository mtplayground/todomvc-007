#[cfg(feature = "ssr")]
use sqlx::{sqlite::SqlitePool, SqlitePool as Pool};

#[cfg(feature = "ssr")]
pub type Db = Pool;

#[cfg(feature = "ssr")]
pub async fn get_db() -> Result<SqlitePool, sqlx::Error> {
    use std::env;
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePool::connect(&database_url).await
}

#[cfg(feature = "ssr")]
pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}
