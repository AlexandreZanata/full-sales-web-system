use dev_seed::DevSeedError;
use dev_seed::{SeedPools, dev_seed_allowed, seed_dev_dataset};
use infra_postgres::{connect, migrate};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "dev_seed=info".into()),
        )
        .init();

    if let Err(err) = run().await {
        eprintln!("dev-seed failed: {err}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), DevSeedError> {
    if !dev_seed_allowed() {
        return Err(DevSeedError::GuardNotSet);
    }

    let _ = dotenvy::from_filename(".env");
    let database_url =
        std::env::var("DATABASE_URL").map_err(|_| DevSeedError::MissingDatabaseUrl)?;
    let admin_url = std::env::var("DATABASE_ADMIN_URL").unwrap_or(database_url.clone());

    info!("connecting to database");
    let admin_pool = connect(&admin_url).await?;
    migrate(&admin_pool).await?;
    let app_pool = connect(&database_url).await?;

    seed_dev_dataset(&SeedPools {
        admin: admin_pool,
        app: app_pool,
    })
    .await?;
    info!("done");
    Ok(())
}
