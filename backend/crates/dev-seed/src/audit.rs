use domain_shared::TenantId;
use infra_postgres::PgPool;
use infra_postgres::audit::{self, NewAuditEvent};
use uuid::Uuid;

use crate::catalog::CatalogSeed;
use crate::commerces::CommercesSeed;
use crate::error::DevSeedResult;
use crate::ids::{order_ids, report_ids, sale_ids};
use crate::users::UsersSeed;

pub async fn seed_audit(
    app_pool: &PgPool,
    tenant: TenantId,
    users: &UsersSeed,
    commerces: &CommercesSeed,
    catalog: &CatalogSeed,
) -> DevSeedResult<()> {
    if infra_postgres::audit::count_audit_events(app_pool, tenant).await? >= 10 {
        return Ok(());
    }

    let events = [
        event(
            "01900001-0070-7000-8000-000000000001",
            users.admin_id,
            "user.created",
            "User",
            users.driver_a_id,
            None,
        ),
        event(
            "01900001-0070-7000-8000-000000000002",
            users.admin_id,
            "commerce.created",
            "Commerce",
            commerces.commerce_a_id,
            None,
        ),
        event(
            "01900001-0070-7000-8000-000000000003",
            users.admin_id,
            "order.approved",
            "Order",
            order_ids().approved,
            None,
        ),
        event(
            "01900001-0070-7000-8000-000000000004",
            users.admin_id,
            "order.rejected",
            "Order",
            order_ids().rejected,
            Some(serde_json::json!({"reason": "Out of delivery zone (RN10 seed)"})),
        ),
        event(
            "01900001-0070-7000-8000-000000000005",
            users.admin_id,
            "sale.confirmed",
            "Sale",
            sale_ids().confirmed,
            None,
        ),
        event(
            "01900001-0070-7000-8000-000000000006",
            users.admin_id,
            "inventory.adjusted",
            "Product",
            catalog.product_ids[0],
            Some(serde_json::json!({"quantity": 20, "movementType": "Inbound"})),
        ),
        event(
            "01900001-0070-7000-8000-000000000007",
            users.admin_id,
            "report.generated",
            "Report",
            report_ids()[0],
            None,
        ),
        event(
            "01900001-0070-7000-8000-000000000008",
            users.driver_a_id,
            "sale.declared_payment_changed",
            "Sale",
            sale_ids().order_linked,
            Some(serde_json::json!({
                "previousMethod": "NotDeclared",
                "previousReceived": false,
                "newMethod": "Pix",
                "newReceived": true
            })),
        ),
        event(
            "01900001-0070-7000-8000-000000000009",
            users.seller_id,
            "order.created",
            "Order",
            order_ids().pending_seller,
            None,
        ),
        event(
            "01900001-0070-7000-8000-000000000010",
            users.portal_contact_id,
            "order.created",
            "Order",
            order_ids().pending_portal,
            None,
        ),
    ];

    for spec in events {
        insert_event_if_missing(app_pool, tenant, spec).await?;
    }
    Ok(())
}

struct AuditSpec {
    id: Uuid,
    actor_id: Uuid,
    action: String,
    resource_type: String,
    resource_id: Uuid,
    metadata: Option<serde_json::Value>,
}

fn event(
    id: &str,
    actor_id: Uuid,
    action: &str,
    resource_type: &str,
    resource_id: Uuid,
    metadata: Option<serde_json::Value>,
) -> AuditSpec {
    AuditSpec {
        id: Uuid::parse_str(id).expect("audit id"),
        actor_id,
        action: action.into(),
        resource_type: resource_type.into(),
        resource_id,
        metadata,
    }
}

async fn insert_event_if_missing(
    app_pool: &PgPool,
    tenant: TenantId,
    spec: AuditSpec,
) -> DevSeedResult<()> {
    let existing = audit::list_audit_event_ids(app_pool, tenant).await?;
    if existing.contains(&spec.id) {
        return Ok(());
    }
    audit::insert_audit_event(
        app_pool,
        tenant,
        NewAuditEvent {
            id: spec.id,
            actor_id: spec.actor_id,
            action: spec.action,
            resource_type: spec.resource_type,
            resource_id: spec.resource_id,
            metadata: spec.metadata,
            correlation_id: None,
        },
    )
    .await?;
    Ok(())
}
