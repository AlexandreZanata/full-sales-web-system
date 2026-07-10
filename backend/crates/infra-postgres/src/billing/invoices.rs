use chrono::{DateTime, Utc};
use domain_billing::InvoiceStatus;
use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;

#[derive(Debug, Clone)]
pub struct InvoiceRow {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub subscription_id: Uuid,
    pub amount_minor: i64,
    pub amount_currency: String,
    pub due_date: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
    pub status: InvoiceStatus,
    pub asaas_payment_id: Option<String>,
    pub pdf_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InvoiceUpsert {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub subscription_id: Uuid,
    pub amount_minor: i64,
    pub due_date: DateTime<Utc>,
    pub status: InvoiceStatus,
    pub asaas_payment_id: String,
    pub paid_at: Option<DateTime<Utc>>,
    pub pdf_url: Option<String>,
}

pub async fn upsert_invoice(pool: &PgPool, invoice: InvoiceUpsert) -> Result<bool, PostgresError> {
    let row = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO billing.invoices
            (id, tenant_id, subscription_id, amount_minor, due_date, status,
             asaas_payment_id, paid_at, pdf_url)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         ON CONFLICT (asaas_payment_id) WHERE asaas_payment_id IS NOT NULL
         DO UPDATE SET
            status = EXCLUDED.status,
            paid_at = COALESCE(billing.invoices.paid_at, EXCLUDED.paid_at),
            updated_at = now()
         RETURNING id",
    )
    .bind(invoice.id)
    .bind(invoice.tenant_id.as_uuid())
    .bind(invoice.subscription_id)
    .bind(invoice.amount_minor)
    .bind(invoice.due_date)
    .bind(invoice.status.as_str())
    .bind(&invoice.asaas_payment_id)
    .bind(invoice.paid_at)
    .bind(&invoice.pdf_url)
    .fetch_optional(pool)
    .await?;
    Ok(row.is_some())
}

pub async fn list_invoices(
    pool: &PgPool,
    tenant_id: TenantId,
    after_id: Option<Uuid>,
    limit: i64,
) -> Result<Vec<InvoiceRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    crate::rls::apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, InvoiceRecord>(
        "SELECT id, tenant_id, subscription_id, amount_minor, amount_currency,
                due_date, paid_at, status, asaas_payment_id, pdf_url
         FROM billing.invoices
         WHERE tenant_id = $1
           AND ($2::uuid IS NULL OR id < $2)
         ORDER BY due_date DESC, id DESC
         LIMIT $3",
    )
    .bind(tenant_id.as_uuid())
    .bind(after_id)
    .bind(limit)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows.into_iter().map(InvoiceRow::from).collect())
}

pub async fn find_invoice(
    pool: &PgPool,
    tenant_id: TenantId,
    invoice_id: Uuid,
) -> Result<Option<InvoiceRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    crate::rls::apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<_, InvoiceRecord>(
        "SELECT id, tenant_id, subscription_id, amount_minor, amount_currency,
                due_date, paid_at, status, asaas_payment_id, pdf_url
         FROM billing.invoices
         WHERE id = $1 AND tenant_id = $2",
    )
    .bind(invoice_id)
    .bind(tenant_id.as_uuid())
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(InvoiceRow::from))
}

#[derive(sqlx::FromRow)]
struct InvoiceRecord {
    id: Uuid,
    tenant_id: Uuid,
    subscription_id: Uuid,
    amount_minor: i64,
    amount_currency: String,
    due_date: DateTime<Utc>,
    paid_at: Option<DateTime<Utc>>,
    status: String,
    asaas_payment_id: Option<String>,
    pdf_url: Option<String>,
}

impl From<InvoiceRecord> for InvoiceRow {
    fn from(row: InvoiceRecord) -> Self {
        Self {
            id: row.id,
            tenant_id: TenantId::from_uuid(row.tenant_id),
            subscription_id: row.subscription_id,
            amount_minor: row.amount_minor,
            amount_currency: row.amount_currency,
            due_date: row.due_date,
            paid_at: row.paid_at,
            status: InvoiceStatus::parse(&row.status).unwrap_or(InvoiceStatus::Open),
            asaas_payment_id: row.asaas_payment_id,
            pdf_url: row.pdf_url,
        }
    }
}
