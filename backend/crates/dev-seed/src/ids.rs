//! Deterministic UUIDs for idempotent dev seeding.

use domain_shared::TenantId;
use uuid::Uuid;

pub const DEV_TENANT_NAME: &str = "Dev Tenant";
pub const DEV_SIGNING_KEY_ID: &str = "dev-key-1";
pub const DEV_PASSWORD: &str = "secret123";

pub fn tenant_id() -> TenantId {
    TenantId::parse("01900001-0000-7000-8000-000000000001").expect("tenant id")
}

pub fn admin_user_id() -> Uuid {
    parse("01900001-0001-7000-8000-000000000001")
}

pub fn driver_a_id() -> Uuid {
    parse("01900001-0002-7000-8000-000000000001")
}

pub fn driver_b_id() -> Uuid {
    parse("01900001-0002-7000-8000-000000000002")
}

pub fn seller_id() -> Uuid {
    parse("01900001-0003-7000-8000-000000000001")
}

pub fn portal_contact_id() -> Uuid {
    parse("01900001-0004-7000-8000-000000000001")
}

pub fn inactive_driver_id() -> Uuid {
    parse("01900001-0005-7000-8000-000000000001")
}

pub fn commerce_a_id() -> Uuid {
    parse("01900001-0010-7000-8000-000000000001")
}

pub fn commerce_b_id() -> Uuid {
    parse("01900001-0010-7000-8000-000000000002")
}

pub fn commerce_inactive_id() -> Uuid {
    parse("01900001-0010-7000-8000-000000000003")
}

pub fn product_ids() -> [Uuid; 4] {
    [
        parse("01900001-0020-7000-8000-000000000001"),
        parse("01900001-0020-7000-8000-000000000002"),
        parse("01900001-0020-7000-8000-000000000003"),
        parse("01900001-0020-7000-8000-000000000004"),
    ]
}

pub fn category_ids() -> [Uuid; 5] {
    [
        parse("01900001-0015-7000-8000-000000000001"),
        parse("01900001-0015-7000-8000-000000000002"),
        parse("01900001-0015-7000-8000-000000000003"),
        parse("01900001-0015-7000-8000-000000000004"),
        parse("01900001-0015-7000-8000-000000000005"),
    ]
}

pub fn category_image_file_ids() -> [Uuid; 3] {
    [
        parse("01900001-0016-7000-8000-000000000001"),
        parse("01900001-0016-7000-8000-000000000002"),
        parse("01900001-0016-7000-8000-000000000003"),
    ]
}

pub fn portal_banner_ids() -> [Uuid; 2] {
    [
        parse("01900001-0017-7000-8000-000000000001"),
        parse("01900001-0017-7000-8000-000000000002"),
    ]
}

pub fn portal_banner_file_ids() -> [Uuid; 2] {
    [
        parse("01900001-0018-7000-8000-000000000001"),
        parse("01900001-0018-7000-8000-000000000002"),
    ]
}

pub fn portal_promotion_ids() -> [Uuid; 2] {
    [
        parse("01900001-0019-7000-8000-000000000001"),
        parse("01900001-0019-7000-8000-000000000002"),
    ]
}

pub fn order_ids() -> OrderIds {
    OrderIds {
        pending_portal: parse("01900001-0030-7000-8000-000000000001"),
        pending_seller: parse("01900001-0030-7000-8000-000000000002"),
        approved: parse("01900001-0030-7000-8000-000000000003"),
        picking: parse("01900001-0030-7000-8000-000000000004"),
        rejected: parse("01900001-0030-7000-8000-000000000005"),
        cancelled: parse("01900001-0030-7000-8000-000000000006"),
        delivered_path: parse("01900001-0030-7000-8000-000000000007"),
    }
}

pub struct OrderIds {
    pub pending_portal: Uuid,
    pub pending_seller: Uuid,
    pub approved: Uuid,
    pub picking: Uuid,
    pub rejected: Uuid,
    pub cancelled: Uuid,
    pub delivered_path: Uuid,
}

pub fn delivery_ids() -> DeliveryIds {
    DeliveryIds {
        waiting: parse("01900001-0040-7000-8000-000000000001"),
        in_transit: parse("01900001-0040-7000-8000-000000000002"),
        delivered: parse("01900001-0040-7000-8000-000000000003"),
    }
}

pub struct DeliveryIds {
    pub waiting: Uuid,
    pub in_transit: Uuid,
    pub delivered: Uuid,
}

pub fn sale_ids() -> SaleIds {
    SaleIds {
        pending: parse("01900001-0050-7000-8000-000000000001"),
        confirmed: parse("01900001-0050-7000-8000-000000000002"),
        cancelled: parse("01900001-0050-7000-8000-000000000003"),
        order_linked: parse("01900001-0050-7000-8000-000000000004"),
    }
}

pub struct SaleIds {
    pub pending: Uuid,
    pub confirmed: Uuid,
    pub cancelled: Uuid,
    pub order_linked: Uuid,
}

pub fn report_ids() -> [Uuid; 2] {
    [
        parse("01900001-0060-7000-8000-000000000001"),
        parse("01900001-0060-7000-8000-000000000002"),
    ]
}

fn parse(raw: &str) -> Uuid {
    Uuid::parse_str(raw).expect("valid seed uuid")
}
