use domain_shared::TenantId;
use infra_postgres::PgPool;
use infra_postgres::SessionContext;
use infra_postgres::orders::{self, OrderInsert, OrderItemInsert};
use uuid::Uuid;

use crate::catalog::CatalogSeed;
use crate::commerces::CommercesSeed;
use crate::error::DevSeedResult;
use crate::ids::order_ids;
use crate::users::UsersSeed;

pub async fn seed_orders(
    app_pool: &PgPool,
    tenant: TenantId,
    users: &UsersSeed,
    commerces: &CommercesSeed,
    catalog: &CatalogSeed,
) -> DevSeedResult<()> {
    let ids = order_ids();
    let session = admin_session(tenant, users.admin_id);
    let product = catalog.product_ids[0];

    insert_order_with_items(
        app_pool,
        &session,
        ids.pending_portal,
        commerces.commerce_a_id,
        users.portal_contact_id,
        "CommercePortal",
        "PendingApproval",
        commerces.commerce_a_delivery_address,
        2_500_00,
        Uuid::parse_str("01900001-0031-7000-8000-000000000001").expect("item"),
        product,
    )
    .await?;

    insert_order_with_items(
        app_pool,
        &session,
        ids.pending_seller,
        commerces.commerce_b_id,
        users.seller_id,
        "SellerVisit",
        "PendingApproval",
        commerces.commerce_a_delivery_address,
        2_500_00,
        Uuid::parse_str("01900001-0031-7000-8000-000000000002").expect("item"),
        product,
    )
    .await?;

    insert_order_with_items(
        app_pool,
        &session,
        ids.approved,
        commerces.commerce_a_id,
        users.seller_id,
        "SellerVisit",
        "Approved",
        commerces.commerce_a_delivery_address,
        2_500_00,
        Uuid::parse_str("01900001-0031-7000-8000-000000000003").expect("item"),
        product,
    )
    .await?;

    insert_order_with_items(
        app_pool,
        &session,
        ids.picking,
        commerces.commerce_b_id,
        users.admin_id,
        "SellerVisit",
        "Picking",
        commerces.commerce_a_delivery_address,
        2_500_00,
        Uuid::parse_str("01900001-0031-7000-8000-000000000004").expect("item"),
        product,
    )
    .await?;

    insert_order_with_items(
        app_pool,
        &session,
        ids.rejected,
        commerces.commerce_a_id,
        users.portal_contact_id,
        "CommercePortal",
        "PendingApproval",
        commerces.commerce_a_delivery_address,
        2_500_00,
        Uuid::parse_str("01900001-0031-7000-8000-000000000005").expect("item"),
        product,
    )
    .await?;
    if orders::find_order_by_id(app_pool, &session, ids.rejected)
        .await?
        .is_some()
    {
        let _ = orders::reject_order(
            app_pool,
            &session,
            ids.rejected,
            "Out of delivery zone (RN10 seed)",
        )
        .await;
    }

    insert_order_with_items(
        app_pool,
        &session,
        ids.cancelled,
        commerces.commerce_b_id,
        users.seller_id,
        "SellerVisit",
        "PendingApproval",
        commerces.commerce_a_delivery_address,
        2_500_00,
        Uuid::parse_str("01900001-0031-7000-8000-000000000006").expect("item"),
        product,
    )
    .await?;
    let _ = orders::cancel_order_transaction(app_pool, &session, ids.cancelled, false).await;

    insert_order_with_items(
        app_pool,
        &session,
        ids.delivered_path,
        commerces.commerce_a_id,
        users.admin_id,
        "SellerVisit",
        "Delivered",
        commerces.commerce_a_delivery_address,
        2_500_00,
        Uuid::parse_str("01900001-0031-7000-8000-000000000007").expect("item"),
        product,
    )
    .await?;

    Ok(())
}

fn admin_session(tenant: TenantId, admin_id: Uuid) -> SessionContext {
    SessionContext {
        tenant_id: tenant,
        role: "Admin".into(),
        user_id: admin_id,
        commerce_id: None,
    }
}

async fn insert_order_with_items(
    app_pool: &PgPool,
    session: &SessionContext,
    order_id: Uuid,
    commerce_id: Uuid,
    created_by: Uuid,
    source: &str,
    status: &str,
    delivery_address_id: Uuid,
    total_amount: i64,
    item_id: Uuid,
    product_id: Uuid,
) -> DevSeedResult<()> {
    if orders::find_order_by_id(app_pool, session, order_id)
        .await?
        .is_some()
    {
        return Ok(());
    }
    orders::insert_order(
        app_pool,
        session,
        &OrderInsert {
            id: order_id,
            commerce_id,
            created_by_user_id: created_by,
            source: source.into(),
            status: status.into(),
            delivery_address_id,
            notes: None,
            total_amount,
            total_currency: "BRL".into(),
        },
    )
    .await?;
    orders::insert_order_items(
        app_pool,
        session,
        &[OrderItemInsert {
            id: item_id,
            order_id,
            product_id,
            quantity_requested: 1,
            unit_price_amount: 2_500_00,
            unit_price_currency: "BRL".into(),
            line_total_amount: 2_500_00,
        }],
    )
    .await?;
    Ok(())
}
