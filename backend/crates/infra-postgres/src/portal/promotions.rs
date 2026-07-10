use chrono::{DateTime, Utc};
use domain_shared::TenantId;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

#[derive(Clone, FromRow)]
pub struct PromotionRow {
    pub id: Uuid,
    pub headline: String,
    pub discount_text: String,
    pub background: String,
    pub category_slug: Option<String>,
    pub link_url: Option<String>,
    pub image_file_id: Option<Uuid>,
    pub sort_order: i32,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, FromRow)]
pub struct PromotionFileRow {
    pub id: Uuid,
    pub headline: String,
    pub discount_text: String,
    pub background: String,
    pub category_slug: Option<String>,
    pub link_url: Option<String>,
    pub bucket: Option<String>,
    pub object_key: Option<String>,
    pub sort_order: i32,
}

pub struct PromotionInsert {
    pub id: Uuid,
    pub headline: String,
    pub discount_text: String,
    pub background: String,
    pub category_slug: Option<String>,
    pub link_url: Option<String>,
    pub image_file_id: Option<Uuid>,
    pub sort_order: i32,
    pub active: bool,
}

pub struct PromotionUpdate {
    pub headline: Option<String>,
    pub discount_text: Option<String>,
    pub background: Option<String>,
    pub category_slug: Option<Option<String>>,
    pub link_url: Option<Option<String>>,
    pub image_file_id: Option<Option<Uuid>>,
    pub sort_order: Option<i32>,
    pub active: Option<bool>,
}

pub async fn list_active_promotions(
    pool: &PgPool,
    tenant_id: TenantId,
    limit: i64,
) -> Result<Vec<PromotionFileRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, PromotionFileRow>(
        "SELECT p.id, p.headline, p.discount_text, p.background, p.category_slug, p.link_url,
                f.bucket, f.object_key, p.sort_order
         FROM portal.promotions p
         LEFT JOIN media.files f ON f.id = p.image_file_id AND f.tenant_id = p.tenant_id
         WHERE p.tenant_id = $1 AND p.active = true
         ORDER BY p.sort_order ASC, p.created_at ASC
         LIMIT $2",
    )
    .bind(tenant_id.as_uuid())
    .bind(limit)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows)
}

pub async fn list_promotions(
    pool: &PgPool,
    tenant_id: TenantId,
    limit: i64,
) -> Result<Vec<PromotionRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, PromotionRow>(
        "SELECT id, headline, discount_text, background, category_slug, link_url, image_file_id,
                sort_order, active, created_at, updated_at
         FROM portal.promotions
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

pub async fn find_promotion_by_id(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
) -> Result<Option<PromotionRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<_, PromotionRow>(
        "SELECT id, headline, discount_text, background, category_slug, link_url, image_file_id,
                sort_order, active, created_at, updated_at
         FROM portal.promotions WHERE tenant_id = $1 AND id = $2",
    )
    .bind(tenant_id.as_uuid())
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row)
}

pub async fn insert_promotion(
    pool: &PgPool,
    tenant_id: TenantId,
    promotion: PromotionInsert,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO portal.promotions
         (id, tenant_id, headline, discount_text, background, category_slug, link_url,
          image_file_id, sort_order, active)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
    )
    .bind(promotion.id)
    .bind(tenant_id.as_uuid())
    .bind(promotion.headline)
    .bind(promotion.discount_text)
    .bind(promotion.background)
    .bind(promotion.category_slug)
    .bind(promotion.link_url)
    .bind(promotion.image_file_id)
    .bind(promotion.sort_order)
    .bind(promotion.active)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn update_promotion(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
    update: &PromotionUpdate,
) -> Result<bool, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let result = sqlx::query(
        "UPDATE portal.promotions SET
           headline = COALESCE($3, headline),
           discount_text = COALESCE($4, discount_text),
           background = COALESCE($5, background),
           category_slug = CASE WHEN $6::bool THEN $7 ELSE category_slug END,
           link_url = CASE WHEN $8::bool THEN $9 ELSE link_url END,
           image_file_id = CASE WHEN $10::bool THEN $11 ELSE image_file_id END,
           sort_order = COALESCE($12, sort_order),
           active = COALESCE($13, active),
           updated_at = now()
         WHERE tenant_id = $1 AND id = $2",
    )
    .bind(tenant_id.as_uuid())
    .bind(id)
    .bind(update.headline.as_deref())
    .bind(update.discount_text.as_deref())
    .bind(update.background.as_deref())
    .bind(update.category_slug.is_some())
    .bind(update.category_slug.as_ref().and_then(|value| value.as_deref()))
    .bind(update.link_url.is_some())
    .bind(update.link_url.as_ref().and_then(|value| value.as_deref()))
    .bind(update.image_file_id.is_some())
    .bind(update.image_file_id.as_ref().and_then(|value| *value))
    .bind(update.sort_order)
    .bind(update.active)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(result.rows_affected() == 1)
}

pub async fn delete_promotion(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
) -> Result<bool, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let result = sqlx::query("DELETE FROM portal.promotions WHERE tenant_id = $1 AND id = $2")
        .bind(tenant_id.as_uuid())
        .bind(id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(result.rows_affected() == 1)
}
