use domain_shared::TenantId;
use infra_postgres::PgPool;
use infra_postgres::SessionContext;
use infra_postgres::deliveries::{self, DeliveryInsert};
use infra_postgres::orders;
use uuid::Uuid;

use crate::error::DevSeedResult;
use crate::ids::{delivery_ids, order_ids};
use crate::users::UsersSeed;

pub async fn seed_deliveries(
    app_pool: &PgPool,
    tenant: TenantId,
    users: &UsersSeed,
) -> DevSeedResult<()> {
    let orders = order_ids();
    let deliveries = delivery_ids();
    let admin_session = session(tenant, "Admin", users.admin_id, None);
    let driver_a_session = session(tenant, "Driver", users.driver_a_id, None);
    let driver_b_session = session(tenant, "Driver", users.driver_b_id, None);

    insert_delivery(
        app_pool,
        &admin_session,
        deliveries.waiting,
        orders.picking,
        users.driver_a_id,
        "Waiting",
    )
    .await?;

    insert_delivery(
        app_pool,
        &admin_session,
        deliveries.in_transit,
        orders.approved,
        users.driver_b_id,
        "Waiting",
    )
    .await?;
    if let Some(row) =
        deliveries::find_delivery_by_id(app_pool, &admin_session, deliveries.in_transit).await?
        && row.status == "Waiting"
    {
        deliveries::start_delivery_transit(
            app_pool,
            &driver_b_session,
            &admin_session,
            deliveries.in_transit,
            users.driver_b_id,
            orders.approved,
        )
        .await?;
    }

    insert_delivery(
        app_pool,
        &admin_session,
        deliveries.delivered,
        orders.delivered_path,
        users.driver_a_id,
        "Delivered",
    )
    .await?;

    let _ =
        orders::update_order_status(app_pool, &admin_session, orders.delivered_path, "Delivered")
            .await;

    let _ = driver_a_session;
    Ok(())
}

fn session(
    tenant: TenantId,
    role: &str,
    user_id: Uuid,
    commerce_id: Option<Uuid>,
) -> SessionContext {
    SessionContext {
        tenant_id: tenant,
        role: role.into(),
        user_id,
        commerce_id,
    }
}

async fn insert_delivery(
    app_pool: &PgPool,
    session: &SessionContext,
    delivery_id: Uuid,
    order_id: Uuid,
    driver_id: Uuid,
    status: &str,
) -> DevSeedResult<()> {
    if deliveries::find_delivery_by_id(app_pool, session, delivery_id)
        .await?
        .is_some()
    {
        return Ok(());
    }
    deliveries::insert_delivery(
        app_pool,
        session,
        &DeliveryInsert {
            id: delivery_id,
            order_id,
            driver_id,
            status: status.into(),
        },
    )
    .await?;
    Ok(())
}
