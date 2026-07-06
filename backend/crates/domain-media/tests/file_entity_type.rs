//! Contract: ProductCategory is a valid media entity type (Phase 43).

use std::str::FromStr;

use domain_media::{FileEntityType, MediaError};

#[test]
fn contract_product_category_entity_type_parses() {
    assert_eq!(
        FileEntityType::from_str("ProductCategory").expect("parse"),
        FileEntityType::ProductCategory
    );
    assert_eq!(FileEntityType::ProductCategory.as_str(), "ProductCategory");
}

#[test]
fn contract_unknown_entity_type_rejected() {
    assert!(matches!(
        FileEntityType::from_str("Unknown"),
        Err(MediaError::InvalidEntityType)
    ));
}
