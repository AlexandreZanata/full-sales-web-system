use domain_shared::TenantId;
use infra_postgres::PgPool;
use infra_postgres::identity::{self, DriverProfileInsert, SellerProfileInsert};
use infra_postgres::media::{self, FileInsert};
use uuid::Uuid;

use crate::error::DevSeedResult;
use crate::foundation::ensure_user;
use crate::ids::{
    admin_user_id, commerce_a_id, driver_a_id, driver_b_id, inactive_driver_id, portal_contact_id,
    seller_id,
};
use crate::media_bytes::{DEV_MEDIA_BUCKET, minimal_webp_bytes};

pub struct UsersSeed {
    pub admin_id: Uuid,
    pub driver_a_id: Uuid,
    pub driver_b_id: Uuid,
    pub seller_id: Uuid,
    pub portal_contact_id: Uuid,
}

pub async fn seed_users(
    admin_pool: &PgPool,
    app_pool: &PgPool,
    tenant: TenantId,
) -> DevSeedResult<UsersSeed> {
    let cnh_file_id = Uuid::parse_str("01900001-0002-7000-8000-000000000099").expect("cnh file");
    seed_driver_cnh_file(app_pool, tenant, driver_a_id(), cnh_file_id).await?;

    ensure_user(
        app_pool,
        admin_pool,
        tenant,
        admin_user_id(),
        "admin@test.com",
        "Dev Admin",
        "Admin",
        true,
        None,
        None,
    )
    .await?;
    ensure_user(
        app_pool,
        admin_pool,
        tenant,
        driver_a_id(),
        "driver-a@test.com",
        "Driver A",
        "Driver",
        true,
        None,
        Some(cnh_file_id),
    )
    .await?;
    ensure_user(
        app_pool,
        admin_pool,
        tenant,
        driver_b_id(),
        "driver-b@test.com",
        "Driver B",
        "Driver",
        true,
        None,
        None,
    )
    .await?;
    ensure_user(
        app_pool,
        admin_pool,
        tenant,
        seller_id(),
        "seller@test.com",
        "Dev Seller",
        "Seller",
        true,
        None,
        None,
    )
    .await?;
    ensure_user(
        app_pool,
        admin_pool,
        tenant,
        portal_contact_id(),
        "portal@seed-store.com",
        "Portal Contact",
        "CommerceContact",
        true,
        Some(commerce_a_id()),
        None,
    )
    .await?;
    ensure_user(
        app_pool,
        admin_pool,
        tenant,
        inactive_driver_id(),
        "inactive-driver@test.com",
        "Inactive Driver",
        "Driver",
        false,
        None,
        None,
    )
    .await?;

    seed_profiles(app_pool, tenant, cnh_file_id).await?;

    Ok(UsersSeed {
        admin_id: admin_user_id(),
        driver_a_id: driver_a_id(),
        driver_b_id: driver_b_id(),
        seller_id: seller_id(),
        portal_contact_id: portal_contact_id(),
    })
}

async fn seed_profiles(
    app_pool: &PgPool,
    tenant: TenantId,
    cnh_file_id: Uuid,
) -> DevSeedResult<()> {
    if identity::find_driver_profile_by_user_id(app_pool, tenant, driver_a_id())
        .await?
        .is_none()
    {
        identity::upsert_driver_profile(
            app_pool,
            tenant,
            DriverProfileInsert {
                user_id: driver_a_id(),
                cnh_number: "12345678901".into(),
                cnh_category: "B".into(),
                cnh_photo_file_id: Some(cnh_file_id),
                vehicle_plate: "ABC1D23".into(),
                vehicle_model: "Fiat Fiorino".into(),
                vehicle_capacity_kg: Some(800.0),
            },
        )
        .await?;
    }
    if identity::find_driver_profile_by_user_id(app_pool, tenant, driver_b_id())
        .await?
        .is_none()
    {
        identity::upsert_driver_profile(
            app_pool,
            tenant,
            DriverProfileInsert {
                user_id: driver_b_id(),
                cnh_number: "98765432109".into(),
                cnh_category: "C".into(),
                cnh_photo_file_id: None,
                vehicle_plate: "XYZ9K88".into(),
                vehicle_model: "VW Delivery".into(),
                vehicle_capacity_kg: Some(1200.0),
            },
        )
        .await?;
    }
    identity::upsert_seller_profile(
        app_pool,
        tenant,
        SellerProfileInsert {
            user_id: seller_id(),
            operating_region: Some("Greater São Paulo".into()),
            monthly_target_amount: Some(50_000_00),
            public_code: Some("dev-seller".into()),
            contact_phone: Some("11987654321".into()),
            share_link_active: true,
        },
    )
    .await?;
    Ok(())
}

async fn seed_driver_cnh_file(
    app_pool: &PgPool,
    tenant: TenantId,
    driver_id: Uuid,
    file_id: Uuid,
) -> DevSeedResult<()> {
    if media::find_file_by_id(app_pool, tenant, file_id)
        .await?
        .is_some()
    {
        return Ok(());
    }
    let bytes = minimal_webp_bytes();
    media::insert_file(
        app_pool,
        tenant,
        FileInsert {
            id: file_id,
            entity_type: "User".into(),
            entity_id: driver_id,
            bucket: DEV_MEDIA_BUCKET.into(),
            object_key: "drivers/cnh-a.webp".into(),
            mime_type: "image/webp".into(),
            size_bytes: bytes.len() as i64,
            sha256: "dev-seed-cnh-sha256".into(),
            uploaded_by_user_id: admin_user_id(),
        },
    )
    .await?;
    Ok(())
}
