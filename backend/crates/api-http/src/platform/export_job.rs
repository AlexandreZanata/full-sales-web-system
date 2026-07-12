use std::io::{Cursor, Write};

use chrono::Utc;
use domain_shared::TenantId;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

use crate::error::ApiError;
use crate::state::AppState;

pub async fn run_export_job(
    state: &AppState,
    tenant_id: TenantId,
    job_id: uuid::Uuid,
) -> Result<(), String> {
    infra_postgres::ops::mark_export_processing(&state.admin_pool, job_id)
        .await
        .map_err(|e| e.to_string())?;
    let bundle = build_export_bundle(state, tenant_id)
        .await
        .map_err(|_| "export bundle failed".to_string())?;
    let zip_bytes = build_zip(&bundle).map_err(|e| e.to_string())?;
    let bucket = export_bucket();
    let key = format!("exports/{tenant_id}/{job_id}.zip");
    state
        .storage
        .put_object(&bucket, &key, &zip_bytes, "application/zip")
        .await
        .map_err(|e| e.to_string())?;
    infra_postgres::ops::mark_export_completed(&state.admin_pool, job_id, &bucket, &key)
        .await
        .map_err(|e| e.to_string())
}

async fn build_export_bundle(
    state: &AppState,
    tenant_id: TenantId,
) -> Result<serde_json::Value, ApiError> {
    Ok(serde_json::json!({
        "tenantId": tenant_id.as_uuid(),
        "exportedAt": Utc::now(),
        "users": infra_postgres::ops::fetch_export_users(&state.app_pool, tenant_id)
            .await
            .map_err(|_| ApiError::internal())?,
        "commerces": infra_postgres::ops::fetch_export_commerces(&state.app_pool, tenant_id)
            .await
            .map_err(|_| ApiError::internal())?,
        "orders": infra_postgres::ops::fetch_export_orders(&state.app_pool, tenant_id)
            .await
            .map_err(|_| ApiError::internal())?,
        "sales": infra_postgres::ops::fetch_export_sales(&state.app_pool, tenant_id)
            .await
            .map_err(|_| ApiError::internal())?,
    }))
}

fn build_zip(bundle: &serde_json::Value) -> Result<Vec<u8>, String> {
    let mut buffer = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(&mut buffer);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    for name in [
        "users.json",
        "commerces.json",
        "orders.json",
        "sales.json",
        "manifest.json",
    ] {
        let content = if name == "manifest.json" {
            serde_json::to_vec_pretty(bundle).map_err(|e| e.to_string())?
        } else {
            let key = name
                .strip_suffix(".json")
                .ok_or_else(|| "expected .json suffix".to_owned())?;
            serde_json::to_vec_pretty(&bundle[key]).map_err(|e| e.to_string())?
        };
        zip.start_file(name, options).map_err(|e| e.to_string())?;
        zip.write_all(&content).map_err(|e| e.to_string())?;
    }
    zip.finish().map_err(|e| e.to_string())?;
    Ok(buffer.into_inner())
}

fn export_bucket() -> String {
    std::env::var("EXPORT_BUCKET").unwrap_or_else(|_| "exports".into())
}
