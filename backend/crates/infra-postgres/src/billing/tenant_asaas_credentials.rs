use chrono::{DateTime, Utc};
use domain_shared::TenantId;
use sqlx::PgPool;

use crate::PostgresError;

#[derive(Debug, Clone)]
pub struct TenantCredentialsRow {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub key_version: i16,
    pub api_key_last4: String,
    pub connected_at: DateTime<Utc>,
}

pub async fn find_credentials(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<Option<TenantCredentialsRow>, PostgresError> {
    let row = sqlx::query_as::<_, (Vec<u8>, Vec<u8>, i16, String, DateTime<Utc>)>(
        "SELECT ciphertext, nonce, key_version, api_key_last4, connected_at
         FROM billing.tenant_asaas_credentials
         WHERE tenant_id = $1",
    )
    .bind(tenant_id.as_uuid())
    .fetch_optional(pool)
    .await?;

    Ok(row.map(
        |(ciphertext, nonce, key_version, api_key_last4, connected_at)| TenantCredentialsRow {
            ciphertext,
            nonce,
            key_version,
            api_key_last4,
            connected_at,
        },
    ))
}

pub async fn upsert_credentials(
    pool: &PgPool,
    tenant_id: TenantId,
    ciphertext: &[u8],
    nonce: &[u8],
    key_version: i16,
    api_key_last4: &str,
) -> Result<(), PostgresError> {
    sqlx::query(
        "INSERT INTO billing.tenant_asaas_credentials
         (tenant_id, ciphertext, nonce, key_version, api_key_last4)
         VALUES ($1, $2, $3, $4, $5)
         ON CONFLICT (tenant_id) DO UPDATE SET
           ciphertext = EXCLUDED.ciphertext,
           nonce = EXCLUDED.nonce,
           key_version = EXCLUDED.key_version,
           api_key_last4 = EXCLUDED.api_key_last4,
           connected_at = now(),
           updated_at = now()",
    )
    .bind(tenant_id.as_uuid())
    .bind(ciphertext)
    .bind(nonce)
    .bind(key_version)
    .bind(api_key_last4)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_credentials(pool: &PgPool, tenant_id: TenantId) -> Result<(), PostgresError> {
    sqlx::query("DELETE FROM billing.tenant_asaas_credentials WHERE tenant_id = $1")
        .bind(tenant_id.as_uuid())
        .execute(pool)
        .await?;
    Ok(())
}
