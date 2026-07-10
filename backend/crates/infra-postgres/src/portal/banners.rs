use chrono::{DateTime, Utc};
use domain_shared::TenantId;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

#[derive(Clone, FromRow)]
pub struct BannerRow {
    pub id: Uuid,
    pub placement: String,
    pub image_file_id: Uuid,
    pub link_url: Option<String>,
    pub alt_text: Option<String>,
    pub sort_order: i32,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, FromRow)]
pub struct BannerFileRow {
    pub id: Uuid,
    pub placement: String,
    pub image_file_id: Uuid,
    pub bucket: String,
    pub object_key: String,
    pub link_url: Option<String>,
    pub alt_text: Option<String>,
    pub sort_order: i32,
}

pub struct BannerInsert {
    pub id: Uuid,
    pub placement: String,
    pub image_file_id: Uuid,
    pub link_url: Option<String>,
    pub alt_text: Option<String>,
    pub sort_order: i32,
    pub active: bool,
}

pub struct BannerUpdate {
    pub placement: Option<String>,
    pub image_file_id: Option<Uuid>,
    pub link_url: Option<Option<String>>,
    pub alt_text: Option<Option<String>>,
    pub sort_order: Option<i32>,
    pub active: Option<bool>,
}

pub async fn list_active_banners_with_files(
    pool: &PgPool,
    tenant_id: TenantId,
    placement: &str,
    limit: i64,
) -> Result<Vec<BannerFileRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, BannerFileRow>(
        "SELECT b.id, b.placement, b.image_file_id, f.bucket, f.object_key,
                b.link_url, b.alt_text, b.sort_order
         FROM portal.banners b
         INNER JOIN media.files f ON f.id = b.image_file_id AND f.tenant_id = b.tenant_id
         WHERE b.tenant_id = $1 AND b.placement = $2 AND b.active = true
         ORDER BY b.sort_order ASC, b.created_at ASC
         LIMIT $3",
    )
    .bind(tenant_id.as_uuid())
    .bind(placement)
    .bind(limit)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows)
}

pub async fn find_active_banner_media(
    pool: &PgPool,
    tenant_id: TenantId,
    file_id: Uuid,
) -> Result<Option<crate::inventory::product_images::PublicProductMediaRow>, PostgresError> {
    use crate::inventory::product_images::PublicProductMediaRow;

    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<_, (String, String, String)>(
        "SELECT mf.bucket, mf.object_key, mf.mime_type
         FROM media.files mf
         JOIN portal.banners b ON b.image_file_id = mf.id
         WHERE mf.id = $1 AND mf.entity_type = 'PortalBanner' AND b.active = true",
    )
    .bind(file_id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(
        row.map(|(bucket, object_key, mime_type)| PublicProductMediaRow {
            bucket,
            object_key,
            mime_type,
        }),
    )
}

pub async fn list_banners(
    pool: &PgPool,
    tenant_id: TenantId,
    limit: i64,
) -> Result<Vec<BannerRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, BannerRow>(
        "SELECT id, placement, image_file_id, link_url, alt_text, sort_order, active, created_at, updated_at
         FROM portal.banners
         WHERE tenant_id = $1
         ORDER BY sort_order ASC, created_at ASC
         LIMIT $2",
    )
    .bind(tenant_id.as_uuid())
    .bind(limit)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows)
}

pub async fn find_banner_by_id(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
) -> Result<Option<BannerRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<_, BannerRow>(
        "SELECT id, placement, image_file_id, link_url, alt_text, sort_order, active, created_at, updated_at
         FROM portal.banners WHERE tenant_id = $1 AND id = $2",
    )
    .bind(tenant_id.as_uuid())
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row)
}

pub async fn insert_banner(
    pool: &PgPool,
    tenant_id: TenantId,
    banner: BannerInsert,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO portal.banners
         (id, tenant_id, placement, image_file_id, link_url, alt_text, sort_order, active)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(banner.id)
    .bind(tenant_id.as_uuid())
    .bind(banner.placement)
    .bind(banner.image_file_id)
    .bind(banner.link_url)
    .bind(banner.alt_text)
    .bind(banner.sort_order)
    .bind(banner.active)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn update_banner(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
    update: &BannerUpdate,
) -> Result<bool, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let result = sqlx::query(
        "UPDATE portal.banners SET
           placement = COALESCE($3, placement),
           image_file_id = COALESCE($4, image_file_id),
           link_url = CASE WHEN $5::bool THEN $6 ELSE link_url END,
           alt_text = CASE WHEN $7::bool THEN $8 ELSE alt_text END,
           sort_order = COALESCE($9, sort_order),
           active = COALESCE($10, active),
           updated_at = now()
         WHERE tenant_id = $1 AND id = $2",
    )
    .bind(tenant_id.as_uuid())
    .bind(id)
    .bind(update.placement.as_deref())
    .bind(update.image_file_id)
    .bind(update.link_url.is_some())
    .bind(update.link_url.as_ref().and_then(|value| value.as_deref()))
    .bind(update.alt_text.is_some())
    .bind(update.alt_text.as_ref().and_then(|value| value.as_deref()))
    .bind(update.sort_order)
    .bind(update.active)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(result.rows_affected() == 1)
}

pub async fn delete_banner(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
) -> Result<bool, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let result = sqlx::query("DELETE FROM portal.banners WHERE tenant_id = $1 AND id = $2")
        .bind(tenant_id.as_uuid())
        .bind(id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(result.rows_affected() == 1)
}
