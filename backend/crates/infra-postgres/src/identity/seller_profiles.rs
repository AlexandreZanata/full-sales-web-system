use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

pub struct SellerProfileInsert {
    pub user_id: Uuid,
    pub operating_region: Option<String>,
    pub monthly_target_amount: Option<i64>,
}

pub async fn insert_seller_profile(
    pool: &PgPool,
    tenant_id: TenantId,
    profile: SellerProfileInsert,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO identity.seller_profiles
         (user_id, tenant_id, operating_region, monthly_target_amount)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(profile.user_id)
    .bind(tenant_id.as_uuid())
    .bind(profile.operating_region)
    .bind(profile.monthly_target_amount)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}
