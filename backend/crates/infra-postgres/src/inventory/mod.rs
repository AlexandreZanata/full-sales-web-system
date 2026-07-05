use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

pub mod product_categories;
pub mod product_images;
pub mod reservations;
pub mod stock_overview;

const PRODUCT_SELECT: &str = "SELECT p.id, p.sku, p.name, p.price_amount, p.price_currency, p.active,
         p.unit_of_measure, p.category_id, c.name AS category_name, c.slug AS category_slug,
         p.description
         FROM inventory.products p
         LEFT JOIN inventory.product_categories c
           ON c.id = p.category_id AND c.tenant_id = p.tenant_id";

pub struct ProductInsert {
    pub id: Uuid,
    pub sku: String,
    pub name: String,
    pub price_amount: i64,
    pub price_currency: String,
    pub category_id: Option<Uuid>,
    pub unit_of_measure: String,
    pub description: Option<String>,
}

pub async fn insert_product(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
    sku: &str,
    name: &str,
    price_amount: i64,
    price_currency: &str,
) -> Result<(), PostgresError> {
    insert_product_with_catalog(
        pool,
        tenant_id,
        ProductInsert {
            id,
            sku: sku.to_owned(),
            name: name.to_owned(),
            price_amount,
            price_currency: price_currency.to_owned(),
            category_id: None,
            unit_of_measure: "Unit".to_owned(),
            description: None,
        },
    )
    .await
}

pub async fn insert_product_with_catalog(
    pool: &PgPool,
    tenant_id: TenantId,
    product: ProductInsert,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO inventory.products
         (id, tenant_id, sku, name, price_amount, price_currency, category_id, unit_of_measure, description)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
    )
    .bind(product.id)
    .bind(tenant_id.as_uuid())
    .bind(product.sku)
    .bind(product.name)
    .bind(product.price_amount)
    .bind(product.price_currency)
    .bind(product.category_id)
    .bind(product.unit_of_measure)
    .bind(product.description.as_deref())
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn list_product_ids(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<Vec<Uuid>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_scalar::<_, Uuid>("SELECT id FROM inventory.products ORDER BY sku")
        .fetch_all(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(rows)
}

pub struct ProductRow {
    pub id: Uuid,
    pub sku: String,
    pub name: String,
    pub price_amount: i64,
    pub price_currency: String,
    pub active: bool,
    pub unit_of_measure: String,
    pub category_id: Option<Uuid>,
    pub category_name: Option<String>,
    pub category_slug: Option<String>,
    pub description: Option<String>,
}

type ProductDbRow = (
    Uuid,
    String,
    String,
    i64,
    String,
    bool,
    String,
    Option<Uuid>,
    Option<String>,
    Option<String>,
    Option<String>,
);

fn map_product_row(
    (
        id,
        sku,
        name,
        price_amount,
        price_currency,
        active,
        unit_of_measure,
        category_id,
        category_name,
        category_slug,
        description,
    ): ProductDbRow,
) -> ProductRow {
    ProductRow {
        id,
        sku,
        name,
        price_amount,
        price_currency,
        active,
        unit_of_measure,
        category_id,
        category_name,
        category_slug,
        description,
    }
}

pub async fn list_products(
    pool: &PgPool,
    tenant_id: TenantId,
    active: Option<bool>,
    limit: i64,
    offset: i64,
) -> Result<Vec<ProductRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, ProductDbRow>(&format!(
        "{PRODUCT_SELECT}
         WHERE ($1::bool IS NULL OR p.active = $1)
         ORDER BY p.sku LIMIT $2 OFFSET $3"
    ))
    .bind(active)
    .bind(limit)
    .bind(offset)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows.into_iter().map(map_product_row).collect())
}

pub async fn list_portal_products(
    pool: &PgPool,
    tenant_id: TenantId,
    category_slug: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<Vec<ProductRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, ProductDbRow>(&format!(
        "{PRODUCT_SELECT}
         WHERE p.active = true
           AND ($1::text IS NULL OR c.slug = $1)
         ORDER BY p.sku LIMIT $2 OFFSET $3"
    ))
    .bind(category_slug)
    .bind(limit)
    .bind(offset)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows.into_iter().map(map_product_row).collect())
}

pub async fn count_portal_products(
    pool: &PgPool,
    tenant_id: TenantId,
    category_slug: Option<&str>,
) -> Result<i64, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*)
         FROM inventory.products p
         LEFT JOIN inventory.product_categories c
           ON c.id = p.category_id AND c.tenant_id = p.tenant_id
         WHERE p.active = true
           AND ($1::text IS NULL OR c.slug = $1)",
    )
    .bind(category_slug)
    .fetch_one(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(count)
}

pub async fn count_products(
    pool: &PgPool,
    tenant_id: TenantId,
    active: Option<bool>,
) -> Result<i64, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM inventory.products WHERE ($1::bool IS NULL OR active = $1)",
    )
    .bind(active)
    .fetch_one(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(count)
}

pub async fn find_products_by_ids(
    pool: &PgPool,
    tenant_id: TenantId,
    ids: &[Uuid],
) -> Result<Vec<ProductRow>, PostgresError> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, ProductDbRow>(&format!(
        "{PRODUCT_SELECT} WHERE p.id = ANY($1)"
    ))
    .bind(ids)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows.into_iter().map(map_product_row).collect())
}

pub async fn find_product_by_id(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
) -> Result<Option<ProductRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<_, ProductDbRow>(&format!("{PRODUCT_SELECT} WHERE p.id = $1"))
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(map_product_row))
}

pub async fn find_portal_product_by_id(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
) -> Result<Option<ProductRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<_, ProductDbRow>(&format!(
        "{PRODUCT_SELECT} WHERE p.id = $1 AND p.active = true"
    ))
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(map_product_row))
}

pub struct ProductUpdate {
    pub name: Option<String>,
    pub price_amount: Option<i64>,
    pub price_currency: Option<String>,
    pub active: Option<bool>,
    pub category_id: Option<Option<Uuid>>,
    pub unit_of_measure: Option<String>,
    pub description: Option<Option<String>>,
}

pub async fn update_product(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
    update: &ProductUpdate,
) -> Result<bool, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let result = sqlx::query(
        "UPDATE inventory.products SET
           name = COALESCE($2, name),
           price_amount = COALESCE($3, price_amount),
           price_currency = COALESCE($4, price_currency),
           active = COALESCE($5, active),
           category_id = CASE WHEN $6::bool THEN $7 ELSE category_id END,
           unit_of_measure = COALESCE($8, unit_of_measure),
           description = CASE WHEN $9::bool THEN $10 ELSE description END,
           updated_at = now()
         WHERE id = $1",
    )
    .bind(id)
    .bind(update.name.as_deref())
    .bind(update.price_amount)
    .bind(update.price_currency.as_deref())
    .bind(update.active)
    .bind(update.category_id.is_some())
    .bind(update.category_id.as_ref().and_then(|value| *value))
    .bind(update.unit_of_measure.as_deref())
    .bind(update.description.is_some())
    .bind(update.description.as_ref().and_then(|value| value.as_deref()))
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(result.rows_affected() == 1)
}

pub async fn list_stock_movements_by_product(
    pool: &PgPool,
    tenant_id: TenantId,
    product_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<StockMovementRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, (
        Uuid,
        Uuid,
        Uuid,
        String,
        i32,
        Option<Uuid>,
        Option<String>,
        chrono::DateTime<chrono::Utc>,
    )>(
        "SELECT id, product_id, responsible_id, movement_type, quantity, reference_id, reason, created_at
         FROM inventory.stock_movements
         WHERE product_id = $1
         ORDER BY created_at DESC
         LIMIT $2 OFFSET $3",
    )
    .bind(product_id)
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
                product_id,
                responsible_id,
                movement_type,
                quantity,
                reference_id,
                reason,
                created_at,
            )| StockMovementRow {
                id,
                product_id,
                responsible_id,
                movement_type,
                quantity,
                reference_id,
                reason,
                created_at,
            },
        )
        .collect())
}

pub struct StockMovementInsert {
    pub id: Uuid,
    pub product_id: Uuid,
    pub responsible_id: Uuid,
    pub movement_type: String,
    pub quantity: i32,
    pub reference_id: Option<Uuid>,
    pub reason: Option<String>,
}

pub struct StockMovementRow {
    pub id: Uuid,
    pub product_id: Uuid,
    pub responsible_id: Uuid,
    pub movement_type: String,
    pub quantity: i32,
    pub reference_id: Option<Uuid>,
    pub reason: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn insert_stock_movement(
    pool: &PgPool,
    tenant_id: TenantId,
    movement: StockMovementInsert,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO inventory.stock_movements
         (id, tenant_id, product_id, responsible_id, movement_type, quantity, reference_id, reason)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(movement.id)
    .bind(tenant_id.as_uuid())
    .bind(movement.product_id)
    .bind(movement.responsible_id)
    .bind(movement.movement_type)
    .bind(movement.quantity)
    .bind(movement.reference_id)
    .bind(movement.reason)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn decrement_stock_balance(
    pool: &PgPool,
    tenant_id: TenantId,
    driver_id: Uuid,
    product_id: Uuid,
    quantity: i32,
) -> Result<bool, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let result = sqlx::query(
        "UPDATE inventory.stock_balances
         SET quantity = quantity - $4, updated_at = now()
         WHERE tenant_id = $1 AND driver_id = $2 AND product_id = $3 AND quantity >= $4",
    )
    .bind(tenant_id.as_uuid())
    .bind(driver_id)
    .bind(product_id)
    .bind(quantity)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(result.rows_affected() == 1)
}

pub async fn upsert_stock_balance(
    pool: &PgPool,
    tenant_id: TenantId,
    driver_id: Uuid,
    product_id: Uuid,
    quantity: i32,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO inventory.stock_balances (tenant_id, driver_id, product_id, quantity)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (tenant_id, driver_id, product_id)
         DO UPDATE SET quantity = EXCLUDED.quantity, updated_at = now()",
    )
    .bind(tenant_id.as_uuid())
    .bind(driver_id)
    .bind(product_id)
    .bind(quantity)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn get_stock_quantity(
    pool: &PgPool,
    tenant_id: TenantId,
    driver_id: Uuid,
    product_id: Uuid,
) -> Result<Option<i32>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let qty = sqlx::query_scalar::<_, i32>(
        "SELECT quantity FROM inventory.stock_balances
         WHERE tenant_id = $1 AND driver_id = $2 AND product_id = $3",
    )
    .bind(tenant_id.as_uuid())
    .bind(driver_id)
    .bind(product_id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(qty)
}

pub async fn list_stock_movements_by_reference(
    pool: &PgPool,
    tenant_id: TenantId,
    reference_id: Uuid,
) -> Result<Vec<StockMovementRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, (
        Uuid,
        Uuid,
        Uuid,
        String,
        i32,
        Option<Uuid>,
        Option<String>,
        chrono::DateTime<chrono::Utc>,
    )>(
        "SELECT id, product_id, responsible_id, movement_type, quantity, reference_id, reason, created_at
         FROM inventory.stock_movements
         WHERE reference_id = $1
         ORDER BY created_at",
    )
    .bind(reference_id)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows
        .into_iter()
        .map(
            |(
                id,
                product_id,
                responsible_id,
                movement_type,
                quantity,
                reference_id,
                reason,
                created_at,
            )| StockMovementRow {
                id,
                product_id,
                responsible_id,
                movement_type,
                quantity,
                reference_id,
                reason,
                created_at,
            },
        )
        .collect())
}

/// Inserts an Adjustment movement with reason and applies signed balance delta atomically.
pub async fn insert_adjustment_movement(
    pool: &PgPool,
    tenant_id: TenantId,
    driver_id: Uuid,
    product_id: Uuid,
    quantity: i32,
    reason: &str,
    balance_delta: i32,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;

    let movement = StockMovementInsert {
        id: Uuid::now_v7(),
        product_id,
        responsible_id: driver_id,
        movement_type: "Adjustment".to_owned(),
        quantity,
        reference_id: None,
        reason: Some(reason.to_owned()),
    };
    sqlx::query(
        "INSERT INTO inventory.stock_movements
         (id, tenant_id, product_id, responsible_id, movement_type, quantity, reference_id, reason)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(movement.id)
    .bind(tenant_id.as_uuid())
    .bind(movement.product_id)
    .bind(movement.responsible_id)
    .bind(movement.movement_type)
    .bind(movement.quantity)
    .bind(movement.reference_id)
    .bind(movement.reason)
    .execute(&mut *tx)
    .await?;

    let updated = sqlx::query(
        "UPDATE inventory.stock_balances
         SET quantity = quantity + $4, updated_at = now()
         WHERE tenant_id = $1 AND driver_id = $2 AND product_id = $3
           AND quantity + $4 >= 0",
    )
    .bind(tenant_id.as_uuid())
    .bind(driver_id)
    .bind(product_id)
    .bind(balance_delta)
    .execute(&mut *tx)
    .await?;

    if updated.rows_affected() == 0 {
        if balance_delta < 0 {
            return Err(PostgresError::from(sqlx::Error::RowNotFound));
        }
        sqlx::query(
            "INSERT INTO inventory.stock_balances (tenant_id, driver_id, product_id, quantity)
             VALUES ($1, $2, $3, $4)",
        )
        .bind(tenant_id.as_uuid())
        .bind(driver_id)
        .bind(product_id)
        .bind(balance_delta)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}
