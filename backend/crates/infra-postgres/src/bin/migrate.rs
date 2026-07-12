//! One-shot sqlx migrate for Kubernetes Jobs.
//! Env: DATABASE_ADMIN_URL (preferred) or DATABASE_URL.

use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let url = match std::env::var("DATABASE_ADMIN_URL").or_else(|_| std::env::var("DATABASE_URL")) {
        Ok(u) => u,
        Err(_) => {
            eprintln!("DATABASE_ADMIN_URL or DATABASE_URL is required");
            return ExitCode::from(2);
        }
    };

    let pool = match infra_postgres::connect(&url).await {
        Ok(p) => p,
        Err(err) => {
            eprintln!("connect failed: {err}");
            return ExitCode::FAILURE;
        }
    };

    if let Err(err) = infra_postgres::migrate(&pool).await {
        eprintln!("migrate failed: {err}");
        return ExitCode::FAILURE;
    }

    println!("migrations applied");
    ExitCode::SUCCESS
}
