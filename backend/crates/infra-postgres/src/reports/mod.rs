use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

pub struct NewReport<'a> {
    pub id: Uuid,
    pub report_type: &'a str,
    pub period_start: chrono::DateTime<chrono::Utc>,
    pub period_end: chrono::DateTime<chrono::Utc>,
    pub canonical_payload: &'a str,
    pub signature: &'a [u8],
    pub public_key_id: &'a str,
}

pub async fn insert_signing_key(
    pool: &PgPool,
    tenant_id: TenantId,
    public_key_id: &str,
    public_key: &[u8],
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO reports.signing_keys (public_key_id, tenant_id, public_key, valid_from)
         VALUES ($1, $2, $3, now())",
    )
    .bind(public_key_id)
    .bind(tenant_id.as_uuid())
    .bind(public_key)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn insert_report(
    pool: &PgPool,
    tenant_id: TenantId,
    report: NewReport<'_>,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO reports.reports
         (id, tenant_id, report_type, period_start, period_end, canonical_payload, signature, public_key_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(report.id)
    .bind(tenant_id.as_uuid())
    .bind(report.report_type)
    .bind(report.period_start)
    .bind(report.period_end)
    .bind(report.canonical_payload)
    .bind(report.signature)
    .bind(report.public_key_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn list_report_ids(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<Vec<Uuid>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows =
        sqlx::query_scalar::<_, Uuid>("SELECT id FROM reports.reports ORDER BY generated_at")
            .fetch_all(&mut *tx)
            .await?;
    tx.commit().await?;
    Ok(rows)
}

pub struct SigningKeyRow {
    pub public_key_id: String,
    pub public_key: Vec<u8>,
}

pub async fn find_active_signing_key(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<Option<SigningKeyRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<_, (String, Vec<u8>)>(
        "SELECT public_key_id, public_key
         FROM reports.signing_keys
         WHERE active = true
         ORDER BY valid_from DESC
         LIMIT 1",
    )
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(|(public_key_id, public_key)| SigningKeyRow {
        public_key_id,
        public_key,
    }))
}

pub struct ReportRow {
    pub id: Uuid,
    pub report_type: String,
    pub period_start: chrono::DateTime<chrono::Utc>,
    pub period_end: chrono::DateTime<chrono::Utc>,
    pub canonical_payload: String,
    pub signature: Vec<u8>,
    pub public_key_id: String,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn find_report_by_id(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
) -> Result<Option<ReportRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<
        _,
        (
            Uuid,
            String,
            chrono::DateTime<chrono::Utc>,
            chrono::DateTime<chrono::Utc>,
            String,
            Vec<u8>,
            String,
            chrono::DateTime<chrono::Utc>,
        ),
    >(
        "SELECT id, report_type, period_start, period_end, canonical_payload,
                signature, public_key_id, generated_at
         FROM reports.reports WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(
        |(
            id,
            report_type,
            period_start,
            period_end,
            canonical_payload,
            signature,
            public_key_id,
            generated_at,
        )| ReportRow {
            id,
            report_type,
            period_start,
            period_end,
            canonical_payload,
            signature,
            public_key_id,
            generated_at,
        },
    ))
}

pub async fn list_reports(
    pool: &PgPool,
    tenant_id: TenantId,
    limit: i64,
    offset: i64,
) -> Result<Vec<ReportRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<
        _,
        (
            Uuid,
            String,
            chrono::DateTime<chrono::Utc>,
            chrono::DateTime<chrono::Utc>,
            String,
            Vec<u8>,
            String,
            chrono::DateTime<chrono::Utc>,
        ),
    >(
        "SELECT id, report_type, period_start, period_end, canonical_payload,
                signature, public_key_id, generated_at
         FROM reports.reports
         ORDER BY generated_at DESC
         LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows
        .into_iter()
        .map(
            |(
                id,
                report_type,
                period_start,
                period_end,
                canonical_payload,
                signature,
                public_key_id,
                generated_at,
            )| ReportRow {
                id,
                report_type,
                period_start,
                period_end,
                canonical_payload,
                signature,
                public_key_id,
                generated_at,
            },
        )
        .collect())
}

pub struct SaleReportFactRow {
    pub sale_id: Uuid,
    pub order_id: Option<Uuid>,
    pub commerce_id: Uuid,
    pub total_amount: i64,
    pub total_currency: String,
    pub status: String,
    pub order_status: Option<String>,
    pub declared_payment_method: String,
    pub declared_payment_received: bool,
}

pub struct SaleReportQuery {
    pub period_start: chrono::DateTime<chrono::Utc>,
    pub period_end: chrono::DateTime<chrono::Utc>,
    pub driver_id: Option<Uuid>,
    pub commerce_id: Option<Uuid>,
}

pub async fn query_sales_for_report(
    pool: &PgPool,
    tenant_id: TenantId,
    query: &SaleReportQuery,
) -> Result<Vec<SaleReportFactRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<
        _,
        (
            Uuid,
            Option<Uuid>,
            Uuid,
            i64,
            String,
            String,
            Option<String>,
            String,
            bool,
        ),
    >(
        "SELECT s.id, s.order_id, s.commerce_id, s.total_amount, s.total_currency,
                s.status, o.status AS order_status,
                s.declared_payment_method, s.declared_payment_received
         FROM sales.sales s
         LEFT JOIN orders.orders o ON o.id = s.order_id
         WHERE COALESCE(s.confirmed_at, s.created_at) >= $1
           AND COALESCE(s.confirmed_at, s.created_at) <= $2
           AND ($3::uuid IS NULL OR s.driver_id = $3)
           AND ($4::uuid IS NULL OR s.commerce_id = $4)
         ORDER BY s.id",
    )
    .bind(query.period_start)
    .bind(query.period_end)
    .bind(query.driver_id)
    .bind(query.commerce_id)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows
        .into_iter()
        .map(
            |(
                sale_id,
                order_id,
                commerce_id,
                total_amount,
                total_currency,
                status,
                order_status,
                declared_payment_method,
                declared_payment_received,
            )| SaleReportFactRow {
                sale_id,
                order_id,
                commerce_id,
                total_amount,
                total_currency,
                status,
                order_status,
                declared_payment_method,
                declared_payment_received,
            },
        )
        .collect())
}

/// Loads a report by id using a pool that bypasses tenant RLS (e.g. admin) — public verify.
pub async fn find_report_by_id_admin(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<ReportRow>, PostgresError> {
    let row = sqlx::query_as::<
        _,
        (
            Uuid,
            String,
            chrono::DateTime<chrono::Utc>,
            chrono::DateTime<chrono::Utc>,
            String,
            Vec<u8>,
            String,
            chrono::DateTime<chrono::Utc>,
        ),
    >(
        "SELECT id, report_type, period_start, period_end, canonical_payload,
                signature, public_key_id, generated_at
         FROM reports.reports WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(
        |(
            id,
            report_type,
            period_start,
            period_end,
            canonical_payload,
            signature,
            public_key_id,
            generated_at,
        )| ReportRow {
            id,
            report_type,
            period_start,
            period_end,
            canonical_payload,
            signature,
            public_key_id,
            generated_at,
        },
    ))
}

pub async fn find_signing_key_by_public_key_id(
    pool: &PgPool,
    public_key_id: &str,
) -> Result<Option<SigningKeyRow>, PostgresError> {
    let row = sqlx::query_as::<_, (String, Vec<u8>)>(
        "SELECT public_key_id, public_key FROM reports.signing_keys WHERE public_key_id = $1",
    )
    .bind(public_key_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(public_key_id, public_key)| SigningKeyRow {
        public_key_id,
        public_key,
    }))
}
