use domain_shared::TenantId;
use infra_postgres::PgPool;
use infra_postgres::PostgresError;
use infra_postgres::sales::{self, ConfirmSaleItem, NewSaleItem, SaleInsert};
use uuid::Uuid;

use crate::catalog::CatalogSeed;
use crate::commerces::CommercesSeed;
use crate::error::DevSeedResult;
use crate::ids::{order_ids, sale_ids};
use crate::users::UsersSeed;

pub async fn seed_sales(
    app_pool: &PgPool,
    tenant: TenantId,
    users: &UsersSeed,
    commerces: &CommercesSeed,
    catalog: &CatalogSeed,
) -> DevSeedResult<()> {
    let ids = sale_ids();
    let product = catalog.product_ids[0];
    let line = sale_line(product, 2_500_00);

    insert_pending_sale(
        app_pool,
        tenant,
        ids.pending,
        users.driver_a_id,
        commerces.commerce_a_id,
        "Cash",
        &[line],
    )
    .await?;

    insert_pending_sale(
        app_pool,
        tenant,
        ids.cancelled,
        users.driver_b_id,
        commerces.commerce_b_id,
        "Pix",
        &[sale_line(product, 1_200_00)],
    )
    .await?;
    let _ = sales::cancel_sale_status(app_pool, tenant, ids.cancelled).await;

    let confirmed_line = sale_line(product, 2_500_00);
    insert_pending_sale(
        app_pool,
        tenant,
        ids.confirmed,
        users.driver_a_id,
        commerces.commerce_a_id,
        "Credit",
        &[confirmed_line],
    )
    .await?;
    let _ = sales::confirm_sale_with_stock(
        app_pool,
        tenant,
        users.driver_a_id,
        ids.confirmed,
        &[ConfirmSaleItem {
            product_id: product,
            quantity: 1,
        }],
    )
    .await;

    seed_order_linked_sale(
        app_pool,
        tenant,
        users,
        commerces,
        product,
        ids.order_linked,
        order_ids().delivered_path,
    )
    .await?;

    Ok(())
}

fn sale_line(product_id: Uuid, unit_price: i64) -> NewSaleItem {
    NewSaleItem {
        id: Uuid::now_v7(),
        sale_id: Uuid::nil(),
        product_id,
        quantity: 1,
        unit_price_amount: unit_price,
        unit_price_currency: "BRL".into(),
        line_total_amount: unit_price,
    }
}

async fn insert_pending_sale(
    app_pool: &PgPool,
    tenant: TenantId,
    sale_id: Uuid,
    driver_id: Uuid,
    commerce_id: Uuid,
    payment_method: &str,
    lines: &[NewSaleItem],
) -> DevSeedResult<()> {
    if sales::find_sale_by_id(app_pool, tenant, sale_id)
        .await?
        .is_some()
    {
        return Ok(());
    }
    let total: i64 = lines.iter().map(|line| line.line_total_amount).sum();
    let item_id = match sale_id.as_u128() % 3 {
        0 => Uuid::parse_str("01900001-0051-7000-8000-000000000001").expect("si1"),
        1 => Uuid::parse_str("01900001-0051-7000-8000-000000000002").expect("si2"),
        _ => Uuid::parse_str("01900001-0051-7000-8000-000000000003").expect("si3"),
    };
    let items = vec![NewSaleItem {
        id: item_id,
        sale_id,
        product_id: lines[0].product_id,
        quantity: lines[0].quantity,
        unit_price_amount: lines[0].unit_price_amount,
        unit_price_currency: lines[0].unit_price_currency.clone(),
        line_total_amount: lines[0].line_total_amount,
    }];
    sales::insert_sale_with_items(
        app_pool,
        tenant,
        SaleInsert {
            sale_id,
            driver_id,
            commerce_id,
            payment_method: payment_method.into(),
            total_amount: total,
            total_currency: "BRL".into(),
            items,
        },
    )
    .await?;
    Ok(())
}

async fn seed_order_linked_sale(
    app_pool: &PgPool,
    tenant: TenantId,
    users: &UsersSeed,
    commerces: &CommercesSeed,
    product_id: Uuid,
    sale_id: Uuid,
    order_id: Uuid,
) -> DevSeedResult<()> {
    if sales::find_sale_by_id(app_pool, tenant, sale_id)
        .await?
        .is_some()
    {
        return Ok(());
    }
    let item_id = Uuid::parse_str("01900001-0051-7000-8000-000000000099").expect("item");
    let mut tx = app_pool.begin().await.map_err(PostgresError::from)?;
    infra_postgres::rls::apply_tenant_context(&mut tx, tenant).await?;
    sqlx::query(
        "INSERT INTO sales.sales
         (id, tenant_id, driver_id, commerce_id, order_id, payment_method, status,
          total_amount, total_currency, declared_payment_method, declared_payment_received, confirmed_at)
         VALUES ($1, $2, $3, $4, $5, 'Pix', 'Confirmed', 250000, 'BRL', 'Pix', true, now())",
    )
    .bind(sale_id)
    .bind(tenant.as_uuid())
    .bind(users.driver_a_id)
    .bind(commerces.commerce_a_id)
    .bind(order_id)
    .execute(&mut *tx)
    .await
    .map_err(PostgresError::from)?;
    sqlx::query(
        "INSERT INTO sales.sale_items
         (id, tenant_id, sale_id, product_id, quantity, unit_price_amount, unit_price_currency, line_total_amount)
         VALUES ($1, $2, $3, $4, 1, 250000, 'BRL', 250000)",
    )
    .bind(item_id)
    .bind(tenant.as_uuid())
    .bind(sale_id)
    .bind(product_id)
    .execute(&mut *tx)
    .await
    .map_err(PostgresError::from)?;
    tx.commit().await.map_err(PostgresError::from)?;
    Ok(())
}
