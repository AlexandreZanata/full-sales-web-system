use domain_shared::TenantId;
use infra_postgres::PgPool;
use infra_postgres::commerces::{self, addresses};
use infra_postgres::media::{self, FileInsert};
use uuid::Uuid;

use crate::error::DevSeedResult;
use crate::ids::{admin_user_id, commerce_a_id, commerce_b_id, commerce_inactive_id};
use crate::media_bytes::{DEV_MEDIA_BUCKET, minimal_webp_bytes};

pub struct CommercesSeed {
    pub commerce_a_id: Uuid,
    pub commerce_b_id: Uuid,
    pub commerce_a_delivery_address: Uuid,
    #[allow(dead_code)]
    pub commerce_a_billing_address: Uuid,
}

pub async fn seed_commerces(app_pool: &PgPool, tenant: TenantId) -> DevSeedResult<CommercesSeed> {
    seed_commerce_row(
        app_pool,
        tenant,
        commerce_a_id(),
        "11222333000181",
        "Seed Store Ltda",
        "Seed Store",
    )
    .await?;
    seed_commerce_row(
        app_pool,
        tenant,
        commerce_b_id(),
        "22333444000105",
        "Beta Market Ltda",
        "Beta Market",
    )
    .await?;
    seed_commerce_row(
        app_pool,
        tenant,
        commerce_inactive_id(),
        "33444555000162",
        "Closed Shop Ltda",
        "Closed Shop",
    )
    .await?;
    commerces::deactivate_commerce(app_pool, tenant, commerce_inactive_id()).await?;

    let logo_file_id = Uuid::parse_str("01900001-0010-7000-8000-000000000099").expect("logo");
    seed_logo_file(app_pool, tenant, commerce_a_id(), logo_file_id).await?;
    addresses::update_commerce_logo(app_pool, tenant, commerce_a_id(), Some(logo_file_id)).await?;

    let billing = Uuid::parse_str("01900001-0011-7000-8000-000000000001").expect("billing");
    let delivery_a = Uuid::parse_str("01900001-0011-7000-8000-000000000002").expect("delivery a");
    let delivery_b = Uuid::parse_str("01900001-0011-7000-8000-000000000003").expect("delivery b");

    seed_address_if_missing(
        app_pool,
        tenant,
        billing,
        commerce_a_id(),
        "Billing",
        "Av. Paulista",
        "1000",
        false,
    )
    .await?;
    seed_address_if_missing(
        app_pool,
        tenant,
        delivery_a,
        commerce_a_id(),
        "Delivery",
        "Rua Augusta",
        "200",
        true,
    )
    .await?;
    seed_address_if_missing(
        app_pool,
        tenant,
        delivery_b,
        commerce_b_id(),
        "Delivery",
        "Rua Oscar Freire",
        "50",
        true,
    )
    .await?;

    Ok(CommercesSeed {
        commerce_a_id: commerce_a_id(),
        commerce_b_id: commerce_b_id(),
        commerce_a_delivery_address: delivery_a,
        commerce_a_billing_address: billing,
    })
}

async fn seed_commerce_row(
    app_pool: &PgPool,
    tenant: TenantId,
    id: Uuid,
    cnpj: &str,
    legal_name: &str,
    trade_name: &str,
) -> DevSeedResult<()> {
    if commerces::find_commerce_by_id(app_pool, tenant, id)
        .await?
        .is_some()
    {
        return Ok(());
    }
    commerces::insert_commerce(
        app_pool,
        tenant,
        id,
        cnpj,
        legal_name,
        trade_name,
        serde_json::json!({"city": "São Paulo", "state": "SP"}),
    )
    .await?;
    Ok(())
}

async fn seed_address_if_missing(
    app_pool: &PgPool,
    tenant: TenantId,
    id: Uuid,
    commerce_id: Uuid,
    address_type: &str,
    street: &str,
    number: &str,
    is_primary: bool,
) -> DevSeedResult<()> {
    if addresses::find_address_by_id(app_pool, tenant, id)
        .await?
        .is_some()
    {
        return Ok(());
    }
    addresses::insert_address(
        app_pool,
        tenant,
        addresses::AddressInsert {
            id,
            commerce_id,
            address_type: address_type.into(),
            street: street.into(),
            number: number.into(),
            district: Some("Centro".into()),
            city: "São Paulo".into(),
            state: "SP".into(),
            postal_code: "01310100".into(),
            latitude: None,
            longitude: None,
            is_primary,
        },
    )
    .await?;
    Ok(())
}

async fn seed_logo_file(
    app_pool: &PgPool,
    tenant: TenantId,
    commerce_id: Uuid,
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
            entity_type: "Commerce".into(),
            entity_id: commerce_id,
            bucket: DEV_MEDIA_BUCKET.into(),
            object_key: "commerces/seed-store-logo.webp".into(),
            mime_type: "image/webp".into(),
            size_bytes: bytes.len() as i64,
            sha256: "dev-seed-logo-sha256".into(),
            uploaded_by_user_id: admin_user_id(),
        },
    )
    .await?;
    Ok(())
}
