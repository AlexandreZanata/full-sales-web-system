use sqlx::Error as SqlxError;

#[derive(Debug, thiserror::Error)]
pub enum PostgresError {
    #[error("database error: {0}")]
    Database(#[from] SqlxError),

    #[error("migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
}
