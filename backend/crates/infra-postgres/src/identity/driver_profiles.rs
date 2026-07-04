use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

pub struct DriverProfileInsert {
    pub user_id: Uuid,
    pub cnh_number: String,
    pub cnh_category: String,
    pub cnh_photo_file_id: Option<Uuid>,
    pub vehicle_plate: String,
    pub vehicle_model: String,
    pub vehicle_capacity_kg: Option<f64>,
}

pub struct DriverProfileRow {
    pub user_id: Uuid,
    pub cnh_number: String,
    pub cnh_category: String,
    pub cnh_photo_file_id: Option<Uuid>,
    pub vehicle_plate: String,
    pub vehicle_model: String,
}

pub async fn insert_driver_profile(
    pool: &PgPool,
    tenant_id: TenantId,
    profile: DriverProfileInsert,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO identity.driver_profiles
         (user_id, tenant_id, cnh_number, cnh_category, cnh_photo_file_id,
          vehicle_plate, vehicle_model, vehicle_capacity_kg)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(profile.user_id)
    .bind(tenant_id.as_uuid())
    .bind(profile.cnh_number)
    .bind(profile.cnh_category)
    .bind(profile.cnh_photo_file_id)
    .bind(profile.vehicle_plate)
    .bind(profile.vehicle_model)
    .bind(profile.vehicle_capacity_kg)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn find_driver_profile_by_user_id(
    pool: &PgPool,
    tenant_id: TenantId,
    user_id: Uuid,
) -> Result<Option<DriverProfileRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<_, (
        Uuid,
        String,
        String,
        Option<Uuid>,
        String,
        String,
    )>(
        "SELECT user_id, cnh_number, cnh_category, cnh_photo_file_id,
                vehicle_plate, vehicle_model
         FROM identity.driver_profiles WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(
        |(user_id, cnh_number, cnh_category, cnh_photo_file_id, vehicle_plate, vehicle_model)| {
            DriverProfileRow {
                user_id,
                cnh_number,
                cnh_category,
                cnh_photo_file_id,
                vehicle_plate,
                vehicle_model,
            }
        },
    ))
}
