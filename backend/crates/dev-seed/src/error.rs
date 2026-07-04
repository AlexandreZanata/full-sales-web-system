use infra_postgres::PostgresError;

#[derive(Debug, thiserror::Error)]
pub enum DevSeedError {
    #[error("ALLOW_DEV_SEED=1 is required to run dev seed")]
    GuardNotSet,

    #[error("DATABASE_URL is required")]
    MissingDatabaseUrl,

    #[error("dev seed aborted: {0}")]
    Aborted(String),

    #[error(transparent)]
    Database(#[from] PostgresError),
}

pub type DevSeedResult<T> = Result<T, DevSeedError>;
