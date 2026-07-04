use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

pub struct AddressInsert {
    pub id: Uuid,
    pub commerce_id: Uuid,
    pub address_type: String,
    pub street: String,
    pub number: String,
    pub district: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub is_primary: bool,
}

pub async fn insert_address(
    pool: &PgPool,
    tenant_id: TenantId,
    row: AddressInsert,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO commerces.commerce_addresses
         (id, tenant_id, commerce_id, address_type, street, number, district, city, state,
          postal_code, latitude, longitude, is_primary)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)",
    )
    .bind(row.id)
    .bind(tenant_id.as_uuid())
    .bind(row.commerce_id)
    .bind(row.address_type)
    .bind(row.street)
    .bind(row.number)
    .bind(row.district)
    .bind(row.city)
    .bind(row.state)
    .bind(row.postal_code)
    .bind(row.latitude)
    .bind(row.longitude)
    .bind(row.is_primary)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub struct AddressRow {
    pub id: Uuid,
    pub commerce_id: Uuid,
    pub address_type: String,
    pub street: String,
    pub number: String,
    pub district: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub is_primary: bool,
}

pub async fn find_address_by_id(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
) -> Result<Option<AddressRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<_, (
        Uuid,
        Uuid,
        String,
        String,
        String,
        Option<String>,
        String,
        String,
        String,
        Option<f64>,
        Option<f64>,
        bool,
    )>(
        "SELECT id, commerce_id, address_type, street, number, district, city, state,
                postal_code, latitude, longitude, is_primary
         FROM commerces.commerce_addresses WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(
        |(
            id,
            commerce_id,
            address_type,
            street,
            number,
            district,
            city,
            state,
            postal_code,
            latitude,
            longitude,
            is_primary,
        )| AddressRow {
            id,
            commerce_id,
            address_type,
            street,
            number,
            district,
            city,
            state,
            postal_code,
            latitude,
            longitude,
            is_primary,
        },
    ))
}

pub async fn list_addresses_by_commerce(
    pool: &PgPool,
    tenant_id: TenantId,
    commerce_id: Uuid,
) -> Result<Vec<AddressRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, (
        Uuid,
        Uuid,
        String,
        String,
        String,
        Option<String>,
        String,
        String,
        String,
        Option<f64>,
        Option<f64>,
        bool,
    )>(
        "SELECT id, commerce_id, address_type, street, number, district, city, state,
                postal_code, latitude, longitude, is_primary
         FROM commerces.commerce_addresses
         WHERE commerce_id = $1
         ORDER BY address_type, is_primary DESC, created_at",
    )
    .bind(commerce_id)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows
        .into_iter()
        .map(
            |(
                id,
                commerce_id,
                address_type,
                street,
                number,
                district,
                city,
                state,
                postal_code,
                latitude,
                longitude,
                is_primary,
            )| AddressRow {
                id,
                commerce_id,
                address_type,
                street,
                number,
                district,
                city,
                state,
                postal_code,
                latitude,
                longitude,
                is_primary,
            },
        )
        .collect())
}

pub async fn update_commerce_logo(
    pool: &PgPool,
    tenant_id: TenantId,
    commerce_id: Uuid,
    logo_file_id: Option<Uuid>,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query("UPDATE commerces.commerces SET logo_file_id = $1 WHERE id = $2")
        .bind(logo_file_id)
        .bind(commerce_id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(())
}
