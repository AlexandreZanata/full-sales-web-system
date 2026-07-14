use sqlx::Error as SqlxError;

#[derive(Debug, thiserror::Error)]
pub enum PostgresError {
    #[error("database error: {0}")]
    Database(#[from] SqlxError),

    #[error("migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("insufficient available stock for reservation")]
    InsufficientAvailableStock,
}

impl PostgresError {
    pub fn is_row_not_found(&self) -> bool {
        matches!(self, Self::Database(SqlxError::RowNotFound))
    }
}
