use chrono::{DateTime, Utc};
use sqlx::Row;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;

pub struct BlocklistRow {
    pub id: Uuid,
    pub email: Option<String>,
    pub cnpj: Option<String>,
    pub ip: Option<String>,
    pub card_fingerprint: Option<String>,
    pub reason: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

pub struct NewBlocklistEntry {
    pub id: Uuid,
    pub email: Option<String>,
    pub cnpj: Option<String>,
    pub ip: Option<String>,
    pub card_fingerprint: Option<String>,
    pub reason: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_by: Uuid,
}

pub async fn insert_blocklist_entry(
    pool: &PgPool,
    entry: NewBlocklistEntry,
) -> Result<(), PostgresError> {
    sqlx::query(
        "INSERT INTO fraud.blocklist_entries
         (id, email, cnpj, ip, card_fingerprint, reason, expires_at, created_by)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(entry.id)
    .bind(entry.email.as_deref().map(str::to_lowercase))
    .bind(entry.cnpj.as_deref())
    .bind(entry.ip.as_deref())
    .bind(entry.card_fingerprint.as_deref())
    .bind(entry.reason)
    .bind(entry.expires_at)
    .bind(entry.created_by)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_blocklist_entry(pool: &PgPool, id: Uuid) -> Result<bool, PostgresError> {
    let result = sqlx::query("DELETE FROM fraud.blocklist_entries WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn find_active_blocklist_match(
    pool: &PgPool,
    email: Option<&str>,
    cnpj: Option<&str>,
    ip: Option<&str>,
    card_fingerprint: Option<&str>,
) -> Result<Option<BlocklistRow>, PostgresError> {
    let email = email.map(str::to_lowercase);
    let cnpj = cnpj.map(|v| v.chars().filter(|c| c.is_ascii_digit()).collect::<String>());
    let row = sqlx::query_as::<_, BlocklistRow>(
        "SELECT id, email, cnpj, ip, card_fingerprint, reason, expires_at, created_by, created_at
         FROM fraud.blocklist_entries
         WHERE (expires_at IS NULL OR expires_at > now())
           AND (
             ($1::text IS NOT NULL AND lower(email) = lower($1))
             OR ($2::text IS NOT NULL AND cnpj = $2)
             OR ($3::text IS NOT NULL AND ip = $3)
             OR ($4::text IS NOT NULL AND card_fingerprint = $4)
           )
         LIMIT 1",
    )
    .bind(email.as_deref())
    .bind(cnpj.as_deref())
    .bind(ip)
    .bind(card_fingerprint)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

impl sqlx::FromRow<'_, sqlx::postgres::PgRow> for BlocklistRow {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            email: row.try_get("email")?,
            cnpj: row.try_get("cnpj")?,
            ip: row.try_get("ip")?,
            card_fingerprint: row.try_get("card_fingerprint")?,
            reason: row.try_get("reason")?,
            expires_at: row.try_get("expires_at")?,
            created_by: row.try_get("created_by")?,
            created_at: row.try_get("created_at")?,
        })
    }
}
