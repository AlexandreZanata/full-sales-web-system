//! PostgreSQL repository adapters (sqlx) — modular by bounded context.

pub mod audit;
pub mod commerces;
pub mod error;
pub mod identity;
pub mod inventory;
pub mod reports;
pub mod rls;
pub mod sales;
pub mod shared;

pub use error::PostgresError;
pub use rls::set_tenant_context;
pub use sqlx::PgPool;

/// Returns the crate version (health/diagnostics).
pub fn crate_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Connects to PostgreSQL using `DATABASE_URL`.
pub async fn connect(database_url: &str) -> Result<PgPool, PostgresError> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .map_err(PostgresError::from)
}

/// Runs embedded sqlx migrations from `backend/migrations`.
pub async fn migrate(pool: &PgPool) -> Result<(), PostgresError> {
    sqlx::migrate!("../../migrations")
        .run(pool)
        .await
        .map_err(PostgresError::from)
}
