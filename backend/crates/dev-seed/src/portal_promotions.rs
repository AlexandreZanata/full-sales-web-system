//! Portal featured promotions seed.

use domain_shared::TenantId;
use infra_postgres::PgPool;
use infra_postgres::portal::promotions::{self, PromotionInsert};

use crate::error::DevSeedResult;
use crate::ids::{admin_user_id, portal_promotion_file_ids, portal_promotion_ids};
use crate::seed_assets::{ensure_media_file, ensure_storage_bytes, read_asset_or_placeholder};

const PROMO_SPECS: [(&str, &str, &str, &str, &str, i32); 2] = [
    (
        "promotions/promo-burger.png",
        "portal/promotions/promo-burger.png",
        "Tasty Burger",
        "30% OFF",
        "snacks",
        0,
    ),
    (
        "promotions/promo-drinks.png",
        "portal/promotions/promo-drinks.png",
        "Fresh Drinks",
        "20% OFF",
        "drinks",
        1,
    ),
];

pub async fn seed_promotions(
    app_pool: &PgPool,
    admin_pool: &PgPool,
    tenant: TenantId,
) -> DevSeedResult<()> {
    let promo_ids = portal_promotion_ids();
    let file_ids = portal_promotion_file_ids();
    let uploader = admin_user_id();

    for (index, (asset, object_key, headline, discount, category, sort_order)) in
        PROMO_SPECS.iter().enumerate()
    {
        let promo_id = promo_ids[index];
        let file_id = file_ids[index];
        ensure_media_file(
            app_pool,
            admin_pool,
            tenant,
            file_id,
            "PortalPromotion",
            promo_id,
            object_key,
            asset,
            uploader,
        )
        .await?;
        if promotions::find_promotion_by_id(app_pool, tenant, promo_id)
            .await?
            .is_some()
        {
            continue;
        }
        promotions::insert_promotion(
            app_pool,
            tenant,
            PromotionInsert {
                id: promo_id,
                headline: (*headline).into(),
                discount_text: (*discount).into(),
                background: "yellow".into(),
                category_slug: Some((*category).into()),
                link_url: None,
                image_file_id: Some(file_id),
                sort_order: *sort_order,
                active: true,
            },
        )
        .await?;
    }
    for (asset, object_key, _, _, _, _) in PROMO_SPECS {
        let (bytes, mime) = read_asset_or_placeholder(asset);
        ensure_storage_bytes(object_key, &bytes, mime).await?;
    }
    Ok(())
}
