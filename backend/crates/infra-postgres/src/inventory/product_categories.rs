use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

#[derive(Debug, Clone)]
pub struct CategoryRow {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub active: bool,
    pub image_file_id: Option<Uuid>,
}

pub struct CategoryInsert {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub active: bool,
    pub image_file_id: Option<Uuid>,
}

pub struct CategoryUpdate {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<Option<String>>,
    pub sort_order: Option<i32>,
    pub active: Option<bool>,
    pub image_file_id: Option<Option<Uuid>>,
}

pub async fn insert_category(
    pool: &PgPool,
    tenant_id: TenantId,
    category: CategoryInsert,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO inventory.product_categories
         (id, tenant_id, name, slug, description, sort_order, active, image_file_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(category.id)
    .bind(tenant_id.as_uuid())
    .bind(category.name)
    .bind(category.slug)
    .bind(category.description)
    .bind(category.sort_order)
    .bind(category.active)
    .bind(category.image_file_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn find_category_by_id(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
) -> Result<Option<CategoryRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = fetch_category_row(&mut *tx, "id = $1", id).await?;
    tx.commit().await?;
    Ok(row)
}

pub async fn find_category_by_slug(
    pool: &PgPool,
    tenant_id: TenantId,
    slug: &str,
) -> Result<Option<CategoryRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<_, CategoryDbRow>(
        "SELECT id, name, slug, description, sort_order, active, image_file_id
         FROM inventory.product_categories
         WHERE slug = $1",
    )
    .bind(slug)
    .fetch_optional(&mut *tx)
    .await?
    .map(CategoryRow::from);
    tx.commit().await?;
    Ok(row)
}

pub async fn list_categories(
    pool: &PgPool,
    tenant_id: TenantId,
    active: Option<bool>,
    limit: i64,
    offset: i64,
) -> Result<Vec<CategoryRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, CategoryDbRow>(
        "SELECT id, name, slug, description, sort_order, active, image_file_id
         FROM inventory.product_categories
         WHERE ($1::bool IS NULL OR active = $1)
         ORDER BY sort_order ASC, name ASC
         LIMIT $2 OFFSET $3",
    )
    .bind(active)
    .bind(limit)
    .bind(offset)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows.into_iter().map(CategoryRow::from).collect())
}

pub async fn list_active_categories(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<Vec<CategoryRow>, PostgresError> {
    list_categories(pool, tenant_id, Some(true), 500, 0).await
}

pub async fn count_categories(
    pool: &PgPool,
    tenant_id: TenantId,
    active: Option<bool>,
) -> Result<i64, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM inventory.product_categories
         WHERE ($1::bool IS NULL OR active = $1)",
    )
    .bind(active)
    .fetch_one(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(count)
}

pub async fn list_category_slugs(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<Vec<String>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_scalar::<_, String>(
        "SELECT slug FROM inventory.product_categories ORDER BY slug",
    )
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows)
}

pub async fn update_category(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
    update: &CategoryUpdate,
) -> Result<bool, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let result = sqlx::query(
        "UPDATE inventory.product_categories SET
           name = COALESCE($2, name),
           slug = COALESCE($3, slug),
           description = CASE WHEN $4::bool THEN $5 ELSE description END,
           sort_order = COALESCE($6, sort_order),
           active = COALESCE($7, active),
           image_file_id = CASE WHEN $8::bool THEN $9 ELSE image_file_id END,
           updated_at = now()
         WHERE id = $1",
    )
    .bind(id)
    .bind(update.name.as_deref())
    .bind(update.slug.as_deref())
    .bind(update.description.is_some())
    .bind(update.description.as_ref().and_then(|value| value.as_deref()))
    .bind(update.sort_order)
    .bind(update.active)
    .bind(update.image_file_id.is_some())
    .bind(update.image_file_id.as_ref().and_then(|value| value.as_ref()))
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(result.rows_affected() == 1)
}

pub async fn reorder_categories(
    pool: &PgPool,
    tenant_id: TenantId,
    ordered_ids: &[Uuid],
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    for (index, id) in ordered_ids.iter().enumerate() {
        sqlx::query(
            "UPDATE inventory.product_categories
             SET sort_order = $2, updated_at = now()
             WHERE id = $1",
        )
        .bind(id)
        .bind(index as i32)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(())
}

pub async fn count_products_in_category(
    pool: &PgPool,
    tenant_id: TenantId,
    category_id: Uuid,
) -> Result<i64, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM inventory.products WHERE category_id = $1",
    )
    .bind(category_id)
    .fetch_one(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(count)
}

async fn fetch_category_row<'e, E>(
    executor: E,
    predicate: &str,
    id: Uuid,
) -> Result<Option<CategoryRow>, PostgresError>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres>,
{
    let query = format!(
        "SELECT id, name, slug, description, sort_order, active, image_file_id
         FROM inventory.product_categories
         WHERE {predicate}"
    );
    let row = sqlx::query_as::<_, CategoryDbRow>(&query)
        .bind(id)
        .fetch_optional(executor)
        .await?;
    Ok(row.map(CategoryRow::from))
}

#[derive(sqlx::FromRow)]
struct CategoryDbRow {
    id: Uuid,
    name: String,
    slug: String,
    description: Option<String>,
    sort_order: i32,
    active: bool,
    image_file_id: Option<Uuid>,
}

impl From<CategoryDbRow> for CategoryRow {
    fn from(row: CategoryDbRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            slug: row.slug,
            description: row.description,
            sort_order: row.sort_order,
            active: row.active,
            image_file_id: row.image_file_id,
        }
    }
}

pub async fn find_active_category_media(
    pool: &PgPool,
    tenant_id: TenantId,
    file_id: Uuid,
) -> Result<Option<super::product_images::PublicProductMediaRow>, PostgresError> {
    use super::product_images::PublicProductMediaRow;

    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<_, (String, String, String)>(
        "SELECT mf.bucket, mf.object_key, mf.mime_type
         FROM media.files mf
         JOIN inventory.product_categories pc ON pc.image_file_id = mf.id
         WHERE mf.id = $1 AND mf.entity_type = 'ProductCategory' AND pc.active = true",
    )
    .bind(file_id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(|(bucket, object_key, mime_type)| PublicProductMediaRow {
        bucket,
        object_key,
        mime_type,
    }))
}
