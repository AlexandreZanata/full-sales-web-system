//! ProductImage primary invariant — ENTITY-SPEC-product-image.

use domain_inventory::{
    CreateProductImageInput, InventoryError, ProductId, ProductImage, ProductImageId,
};
use domain_shared::TenantId;
use uuid::Uuid;

fn image_input(is_primary: bool) -> CreateProductImageInput {
    CreateProductImageInput {
        id: ProductImageId::generate(),
        tenant_id: TenantId::generate(),
        product_id: ProductId::generate(),
        file_id: Uuid::now_v7(),
        sort_order: 0,
        is_primary,
    }
}

#[test]
fn given_first_primary_when_create_then_ok() {
    let input = image_input(true);
    let image = ProductImage::create(input, &[]).expect("create");
    assert!(image.is_primary());
}

#[test]
fn given_second_primary_when_create_then_duplicate() {
    let product_id = ProductId::generate();
    let tenant = TenantId::generate();
    let first = ProductImage::create(
        CreateProductImageInput {
            product_id,
            tenant_id: tenant,
            ..image_input(true)
        },
        &[],
    )
    .expect("first");
    let err = ProductImage::create(
        CreateProductImageInput {
            product_id,
            tenant_id: tenant,
            ..image_input(true)
        },
        &[first],
    )
    .expect_err("duplicate primary");
    assert_eq!(err, InventoryError::DuplicatePrimaryImage);
}

#[test]
fn given_non_primary_when_primary_exists_then_ok() {
    let product_id = ProductId::generate();
    let tenant = TenantId::generate();
    let primary = ProductImage::create(
        CreateProductImageInput {
            product_id,
            tenant_id: tenant,
            ..image_input(true)
        },
        &[],
    )
    .expect("primary");
    ProductImage::create(
        CreateProductImageInput {
            product_id,
            tenant_id: tenant,
            ..image_input(false)
        },
        &[primary],
    )
    .expect("secondary");
}
