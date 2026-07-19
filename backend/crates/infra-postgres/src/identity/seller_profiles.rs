use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

pub struct SellerProfileInsert {
    pub user_id: Uuid,
    pub operating_region: Option<String>,
    pub monthly_target_amount: Option<i64>,
    pub public_code: Option<String>,
    pub contact_phone: Option<String>,
    pub share_link_active: bool,
}

pub struct SellerProfileRow {
    pub user_id: Uuid,
    pub operating_region: Option<String>,
    pub monthly_target_amount: Option<i64>,
    pub public_code: Option<String>,
    pub contact_phone: Option<String>,
    pub share_link_active: bool,
}

pub struct PublicSellerRow {
    pub public_code: String,
    pub display_name: String,
    pub contact_phone: Option<String>,
}

type ProfileTuple = (
    Uuid,
    Option<String>,
    Option<i64>,
    Option<String>,
    Option<String>,
    bool,
);

fn map_profile_row(row: ProfileTuple) -> SellerProfileRow {
    SellerProfileRow {
        user_id: row.0,
        operating_region: row.1,
        monthly_target_amount: row.2,
        public_code: row.3,
        contact_phone: row.4,
        share_link_active: row.5,
    }
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
         (user_id, tenant_id, operating_region, monthly_target_amount,
          public_code, contact_phone, share_link_active)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(profile.user_id)
    .bind(tenant_id.as_uuid())
    .bind(profile.operating_region)
    .bind(profile.monthly_target_amount)
    .bind(profile.public_code)
    .bind(profile.contact_phone)
    .bind(profile.share_link_active)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn find_seller_profile_by_user_id(
    pool: &PgPool,
    tenant_id: TenantId,
    user_id: Uuid,
) -> Result<Option<SellerProfileRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<_, ProfileTuple>(
        "SELECT user_id, operating_region, monthly_target_amount,
                public_code, contact_phone, share_link_active
         FROM identity.seller_profiles WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(map_profile_row))
}

pub async fn upsert_seller_profile(
    pool: &PgPool,
    tenant_id: TenantId,
    profile: SellerProfileInsert,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO identity.seller_profiles
         (user_id, tenant_id, operating_region, monthly_target_amount,
          public_code, contact_phone, share_link_active)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         ON CONFLICT (user_id) DO UPDATE SET
           operating_region = EXCLUDED.operating_region,
           monthly_target_amount = EXCLUDED.monthly_target_amount,
           public_code = EXCLUDED.public_code,
           contact_phone = EXCLUDED.contact_phone,
           share_link_active = EXCLUDED.share_link_active",
    )
    .bind(profile.user_id)
    .bind(tenant_id.as_uuid())
    .bind(profile.operating_region)
    .bind(profile.monthly_target_amount)
    .bind(profile.public_code)
    .bind(profile.contact_phone)
    .bind(profile.share_link_active)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

/// Resolves an active seller share link for the public portal.
pub async fn find_public_seller_by_code(
    pool: &PgPool,
    tenant_id: TenantId,
    public_code: &str,
) -> Result<Option<PublicSellerRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<_, (String, String, Option<String>)>(
        "SELECT sp.public_code, u.name, sp.contact_phone
         FROM identity.seller_profiles sp
         JOIN identity.users u ON u.id = sp.user_id
         WHERE lower(sp.public_code) = lower($1)
           AND sp.share_link_active = true
           AND u.active = true
           AND u.role = 'Seller'",
    )
    .bind(public_code)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(
        |(public_code, display_name, contact_phone)| PublicSellerRow {
            public_code,
            display_name,
            contact_phone,
        },
    ))
}
