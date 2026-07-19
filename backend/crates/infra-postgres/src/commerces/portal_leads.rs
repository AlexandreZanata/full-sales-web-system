//! Portal commerce interest leads (self-serve signup requests).

use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

#[derive(Debug, Clone)]
pub struct PortalLeadRow {
    pub id: Uuid,
    pub contact_name: String,
    pub phone: String,
    pub commerce_name: String,
    pub email: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub reviewed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub reviewed_by: Option<Uuid>,
}

pub struct PortalLeadInsert<'a> {
    pub id: Uuid,
    pub contact_name: &'a str,
    pub phone: &'a str,
    pub commerce_name: &'a str,
    pub email: &'a str,
}

pub async fn insert_portal_lead(
    pool: &PgPool,
    tenant_id: TenantId,
    lead: PortalLeadInsert<'_>,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        r#"
        INSERT INTO commerces.portal_leads
            (id, tenant_id, contact_name, phone, commerce_name, email, status)
        VALUES ($1, $2, $3, $4, $5, $6, 'pending')
        "#,
    )
    .bind(lead.id)
    .bind(tenant_id.as_uuid())
    .bind(lead.contact_name)
    .bind(lead.phone)
    .bind(lead.commerce_name)
    .bind(lead.email)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn list_portal_leads(
    pool: &PgPool,
    tenant_id: TenantId,
    status: Option<&str>,
    limit: i64,
) -> Result<Vec<PortalLeadRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<
        _,
        (
            Uuid,
            String,
            String,
            String,
            String,
            String,
            chrono::DateTime<chrono::Utc>,
            Option<chrono::DateTime<chrono::Utc>>,
            Option<Uuid>,
        ),
    >(
        r#"
        SELECT id, contact_name, phone, commerce_name, email, status,
               created_at, reviewed_at, reviewed_by
        FROM commerces.portal_leads
        WHERE ($1::text IS NULL OR status = $1)
        ORDER BY created_at DESC
        LIMIT $2
        "#,
    )
    .bind(status)
    .bind(limit)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows
        .into_iter()
        .map(
            |(
                id,
                contact_name,
                phone,
                commerce_name,
                email,
                status,
                created_at,
                reviewed_at,
                reviewed_by,
            )| PortalLeadRow {
                id,
                contact_name,
                phone,
                commerce_name,
                email,
                status,
                created_at,
                reviewed_at,
                reviewed_by,
            },
        )
        .collect())
}

pub async fn update_portal_lead_status(
    pool: &PgPool,
    tenant_id: TenantId,
    lead_id: Uuid,
    status: &str,
    reviewed_by: Uuid,
) -> Result<Option<PortalLeadRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<
        _,
        (
            Uuid,
            String,
            String,
            String,
            String,
            String,
            chrono::DateTime<chrono::Utc>,
            Option<chrono::DateTime<chrono::Utc>>,
            Option<Uuid>,
        ),
    >(
        r#"
        UPDATE commerces.portal_leads
        SET status = $1,
            reviewed_at = now(),
            reviewed_by = $2
        WHERE id = $3
        RETURNING id, contact_name, phone, commerce_name, email, status,
                  created_at, reviewed_at, reviewed_by
        "#,
    )
    .bind(status)
    .bind(reviewed_by)
    .bind(lead_id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(
        |(
            id,
            contact_name,
            phone,
            commerce_name,
            email,
            status,
            created_at,
            reviewed_at,
            reviewed_by,
        )| PortalLeadRow {
            id,
            contact_name,
            phone,
            commerce_name,
            email,
            status,
            created_at,
            reviewed_at,
            reviewed_by,
        },
    ))
}
