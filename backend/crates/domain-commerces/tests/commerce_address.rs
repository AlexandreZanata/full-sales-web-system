//! CommerceAddress domain tests — ENTITY-SPEC-commerce-address validations.

use domain_commerces::{
    AddressType, Cnpj, Commerce, CommerceAddress, CommerceAddressId, CommerceError, CommerceId,
    CreateCommerceAddressInput, CreateCommerceInput, ensure_address_allowed_for_commerce,
    validate_order_delivery_address,
};
use domain_shared::TenantId;

fn sample_commerce(tenant_id: TenantId) -> Commerce {
    Commerce::create(CreateCommerceInput {
        id: CommerceId::generate(),
        cnpj: Cnpj::parse("11222333000181").expect("cnpj"),
        legal_name: "Acme Ltda".into(),
        trade_name: None,
        tenant_id,
    })
}

fn delivery_input(commerce: &Commerce, is_primary: bool) -> CreateCommerceAddressInput {
    CreateCommerceAddressInput {
        id: CommerceAddressId::generate(),
        tenant_id: commerce.tenant_id(),
        commerce_id: commerce.id(),
        address_type: AddressType::Delivery,
        street: "Av Paulista".into(),
        number: "1000".into(),
        district: Some("Bela Vista".into()),
        city: "São Paulo".into(),
        state: "SP".into(),
        postal_code: "01310-100".into(),
        latitude: None,
        longitude: None,
        is_primary,
    }
}

#[test]
fn given_valid_fields_when_create_address_then_ok() {
    let tenant = TenantId::generate();
    let commerce = sample_commerce(tenant);
    let address = CommerceAddress::create(&commerce, delivery_input(&commerce, true), &[])
        .expect("create address");
    assert_eq!(address.postal_code().as_str(), "01310100");
    assert_eq!(address.state().as_str(), "SP");
    assert!(address.is_primary());
}

#[test]
fn given_second_primary_same_type_when_create_then_duplicate_primary() {
    let tenant = TenantId::generate();
    let commerce = sample_commerce(tenant);
    let first =
        CommerceAddress::create(&commerce, delivery_input(&commerce, true), &[]).expect("first");
    let err = CommerceAddress::create(&commerce, delivery_input(&commerce, true), &[first])
        .expect_err("second primary");
    assert_eq!(err, CommerceError::DuplicatePrimaryAddress);
}

#[test]
fn given_billing_address_when_inactive_commerce_then_allowed() {
    let tenant = TenantId::generate();
    let commerce = sample_commerce(tenant).deactivate();
    ensure_address_allowed_for_commerce(&commerce, AddressType::Billing).expect("billing ok");
}

// Contract: BR-CO-004 — inactive commerce cannot add delivery addresses
#[test]
fn given_delivery_address_when_inactive_commerce_then_rejected() {
    let tenant = TenantId::generate();
    let commerce = sample_commerce(tenant).deactivate();
    let err = ensure_address_allowed_for_commerce(&commerce, AddressType::Delivery)
        .expect_err("delivery blocked");
    assert_eq!(err, CommerceError::InactiveCommerceCannotAddDeliveryAddress);

    let create_err = CommerceAddress::create(&commerce, delivery_input(&commerce, false), &[])
        .expect_err("create blocked");
    assert_eq!(
        create_err,
        CommerceError::InactiveCommerceCannotAddDeliveryAddress
    );
}

// Contract: BR-CO-005 — order requires valid delivery address
#[test]
fn given_billing_address_when_validate_order_delivery_then_invalid() {
    let tenant = TenantId::generate();
    let commerce = sample_commerce(tenant);
    let billing = CommerceAddress::create(
        &commerce,
        CreateCommerceAddressInput {
            address_type: AddressType::Billing,
            ..delivery_input(&commerce, true)
        },
        &[],
    )
    .expect("billing");
    let err = validate_order_delivery_address(&commerce, &billing).expect_err("billing invalid");
    assert_eq!(err, CommerceError::InvalidDeliveryAddress);
}

#[test]
fn given_delivery_address_when_validate_order_delivery_then_ok() {
    let tenant = TenantId::generate();
    let commerce = sample_commerce(tenant);
    let delivery =
        CommerceAddress::create(&commerce, delivery_input(&commerce, true), &[]).expect("delivery");
    validate_order_delivery_address(&commerce, &delivery).expect("valid for order");
}

#[test]
fn given_inactive_commerce_when_validate_order_delivery_then_inactive() {
    let tenant = TenantId::generate();
    let commerce = sample_commerce(tenant);
    let delivery =
        CommerceAddress::create(&commerce, delivery_input(&commerce, true), &[]).expect("delivery");
    let inactive = commerce.deactivate();
    let err = validate_order_delivery_address(&inactive, &delivery).expect_err("inactive");
    assert_eq!(err, CommerceError::InactiveCommerce);
}

#[test]
fn given_wrong_commerce_when_validate_order_delivery_then_mismatch() {
    let tenant = TenantId::generate();
    let commerce_a = sample_commerce(tenant);
    let commerce_b = sample_commerce(tenant);
    let delivery = CommerceAddress::create(&commerce_a, delivery_input(&commerce_a, true), &[])
        .expect("delivery");
    let err = validate_order_delivery_address(&commerce_b, &delivery).expect_err("mismatch");
    assert_eq!(err, CommerceError::AddressCommerceMismatch);
}
