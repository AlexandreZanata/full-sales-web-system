use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

pub struct ProductImageInsert {
    pub id: Uuid,
    pub product_id: Uuid,
    pub file_id: Uuid,
    pub sort_order: i32,
    pub is_primary: bool,
}

pub async fn insert_product_image(
    pool: &PgPool,
    tenant_id: TenantId,
    row: ProductImageInsert,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO inventory.product_images
         (id, tenant_id, product_id, file_id, sort_order, is_primary)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(row.id)
    .bind(tenant_id.as_uuid())
    .bind(row.product_id)
    .bind(row.file_id)
    .bind(row.sort_order)
    .bind(row.is_primary)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub struct ProductImageRow {
    pub id: Uuid,
    pub product_id: Uuid,
    pub file_id: Uuid,
    pub sort_order: i32,
    pub is_primary: bool,
}

pub async fn list_product_images(
    pool: &PgPool,
    tenant_id: TenantId,
    product_id: Uuid,
) -> Result<Vec<ProductImageRow>, PostgresError> {
    list_product_images_cursor(pool, tenant_id, product_id, None, i64::MAX / 2).await
}

pub async fn list_product_images_cursor(
    pool: &PgPool,
    tenant_id: TenantId,
    product_id: Uuid,
    after_id: Option<Uuid>,
    limit: i64,
) -> Result<Vec<ProductImageRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, (Uuid, Uuid, Uuid, i32, bool)>(
        "SELECT id, product_id, file_id, sort_order, is_primary
         FROM inventory.product_images
         WHERE product_id = $1
           AND ($2::uuid IS NULL OR id > $2)
         ORDER BY id ASC
         LIMIT $3",
    )
    .bind(product_id)
    .bind(after_id)
    .bind(limit)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows
        .into_iter()
        .map(
            |(id, product_id, file_id, sort_order, is_primary)| ProductImageRow {
                id,
                product_id,
                file_id,
                sort_order,
                is_primary,
            },
        )
        .collect())
}

pub struct ProductGalleryImageRow {
    pub file_id: Uuid,
    pub sort_order: i32,
    pub is_primary: bool,
    pub bucket: String,
    pub object_key: String,
}

pub async fn find_gallery_images_for_product(
    pool: &PgPool,
    tenant_id: TenantId,
    product_id: Uuid,
) -> Result<Vec<ProductGalleryImageRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, (Uuid, i32, bool, String, String)>(
        "SELECT pi.file_id, pi.sort_order, pi.is_primary, mf.bucket, mf.object_key
         FROM inventory.product_images pi
         JOIN media.files mf ON mf.id = pi.file_id
         WHERE pi.product_id = $1
         ORDER BY pi.is_primary DESC, pi.sort_order, pi.created_at",
    )
    .bind(product_id)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows
        .into_iter()
        .map(
            |(file_id, sort_order, is_primary, bucket, object_key)| ProductGalleryImageRow {
                file_id,
                sort_order,
                is_primary,
                bucket,
                object_key,
            },
        )
        .collect())
}

pub async fn find_product_image_by_id(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
) -> Result<Option<ProductImageRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<_, (Uuid, Uuid, Uuid, i32, bool)>(
        "SELECT id, product_id, file_id, sort_order, is_primary
         FROM inventory.product_images WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(
        |(id, product_id, file_id, sort_order, is_primary)| ProductImageRow {
            id,
            product_id,
            file_id,
            sort_order,
            is_primary,
        },
    ))
}

pub async fn clear_primary_for_product(
    pool: &PgPool,
    tenant_id: TenantId,
    product_id: Uuid,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query("UPDATE inventory.product_images SET is_primary = false WHERE product_id = $1")
        .bind(product_id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn delete_product_image(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
) -> Result<bool, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let result = sqlx::query("DELETE FROM inventory.product_images WHERE id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(result.rows_affected() == 1)
}

pub struct PrimaryProductImageRow {
    pub product_id: Uuid,
    pub file_id: Uuid,
    pub bucket: String,
    pub object_key: String,
}

pub struct PublicProductMediaRow {
    pub bucket: String,
    pub object_key: String,
    pub mime_type: String,
}

pub async fn find_primary_images_for_products(
    pool: &PgPool,
    tenant_id: TenantId,
    product_ids: &[Uuid],
) -> Result<Vec<PrimaryProductImageRow>, PostgresError> {
    if product_ids.is_empty() {
        return Ok(Vec::new());
    }
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, (Uuid, Uuid, String, String)>(
        "SELECT pi.product_id, mf.id, mf.bucket, mf.object_key
         FROM inventory.product_images pi
         JOIN media.files mf ON mf.id = pi.file_id
         WHERE pi.is_primary = true AND pi.product_id = ANY($1)",
    )
    .bind(product_ids)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows
        .into_iter()
        .map(
            |(product_id, file_id, bucket, object_key)| PrimaryProductImageRow {
                product_id,
                file_id,
                bucket,
                object_key,
            },
        )
        .collect())
}

pub async fn find_active_product_media(
    pool: &PgPool,
    tenant_id: TenantId,
    file_id: Uuid,
) -> Result<Option<PublicProductMediaRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<_, (String, String, String)>(
        "SELECT mf.bucket, mf.object_key, mf.mime_type
         FROM media.files mf
         JOIN inventory.product_images pi ON pi.file_id = mf.id
         JOIN inventory.products p ON p.id = pi.product_id
         WHERE mf.id = $1 AND mf.entity_type = 'Product' AND p.active = true",
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
