use domain_identity::Role;
use domain_sales::{PaymentMethod, SaleStatus};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateSaleItemRequest {
    #[serde(rename = "productId")]
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateSaleRequest {
    #[serde(rename = "commerceId")]
    pub commerce_id: Uuid,
    pub items: Vec<CreateSaleItemRequest>,
    #[serde(rename = "paymentMethod")]
    pub payment_method: String,
}

#[derive(Serialize)]
pub struct SaleItemResponse {
    #[serde(rename = "productId")]
    pub product_id: Uuid,
    pub quantity: i32,
    #[serde(rename = "unitPriceAmount")]
    pub unit_price_amount: i64,
    #[serde(rename = "unitPriceCurrency")]
    pub unit_price_currency: String,
    #[serde(rename = "lineTotalAmount")]
    pub line_total_amount: i64,
}

#[derive(Serialize)]
pub struct SaleResponse {
    pub id: Uuid,
    #[serde(rename = "commerceId")]
    pub commerce_id: Uuid,
    #[serde(rename = "driverId")]
    pub driver_id: Uuid,
    pub status: SaleStatus,
    #[serde(rename = "paymentMethod")]
    pub payment_method: PaymentMethod,
    #[serde(rename = "totalAmount")]
    pub total_amount: i64,
    #[serde(rename = "totalCurrency")]
    pub total_currency: String,
    pub items: Vec<SaleItemResponse>,
}

#[derive(Serialize)]
pub struct SaleSummaryResponse {
    pub id: Uuid,
    #[serde(rename = "commerceId")]
    pub commerce_id: Uuid,
    #[serde(rename = "driverId")]
    pub driver_id: Uuid,
    pub status: SaleStatus,
    #[serde(rename = "paymentMethod")]
    pub payment_method: PaymentMethod,
    #[serde(rename = "totalAmount")]
    pub total_amount: i64,
    #[serde(rename = "totalCurrency")]
    pub total_currency: String,
    #[serde(rename = "declaredPaymentMethod")]
    pub declared_payment_method: String,
    #[serde(rename = "declaredPaymentReceived")]
    pub declared_payment_received: bool,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub(crate) fn sale_response_from_dto(dto: &application::sales::SaleDto) -> SaleResponse {
    SaleResponse {
        id: dto.id.as_uuid(),
        commerce_id: dto.commerce_id,
        driver_id: dto.driver_id.as_uuid(),
        status: dto.status,
        payment_method: dto.payment_method,
        total_amount: dto.total.amount_minor(),
        total_currency: dto.total.currency().as_str().to_owned(),
        items: dto
            .items
            .iter()
            .map(|item| SaleItemResponse {
                product_id: item.product_id,
                quantity: item.quantity,
                unit_price_amount: item.unit_price_amount,
                unit_price_currency: item.unit_price_currency.clone(),
                line_total_amount: item.line_total_amount,
            })
            .collect(),
    }
}

pub(crate) fn parse_sale_status(value: &str) -> Result<SaleStatus, ApiError> {
    match value {
        "Pending" => Ok(SaleStatus::Pending),
        "Confirmed" => Ok(SaleStatus::Confirmed),
        "Cancelled" => Ok(SaleStatus::Cancelled),
        _ => Err(ApiError::internal()),
    }
}

pub(crate) fn require_can_record_sale(auth: &AuthUser) -> Result<(), ApiError> {
    match auth.role {
        Role::Admin | Role::Driver | Role::Seller => Ok(()),
        Role::CommerceContact => Err(ApiError::forbidden()),
    }
}

pub(crate) fn map_sales_app_error(err: application::sales::SalesAppError) -> ApiError {
    match err {
        application::sales::SalesAppError::Sale(domain_sales::SaleError::InactiveCommerce) => {
            ApiError::inactive_commerce()
        }
        application::sales::SalesAppError::Sale(domain_sales::SaleError::InactiveProduct) => {
            ApiError::inactive_product()
        }
        application::sales::SalesAppError::Inventory(
            domain_inventory::InventoryError::InvalidQuantity,
        ) => ApiError::bad_request("VALIDATION_ERROR", "Quantity must be a positive integer"),
        application::sales::SalesAppError::ProductNotFound => ApiError::product_not_found(),
        application::sales::SalesAppError::CommerceNotFound => ApiError::commerce_not_found(),
        application::sales::SalesAppError::SaleNotFound => ApiError::sale_not_found(),
        application::sales::SalesAppError::InsufficientStock => ApiError::insufficient_stock(),
        application::sales::SalesAppError::Sale(domain_sales::SaleError::InvalidTransition {
            ..
        }) => ApiError::invalid_transition(),
        application::sales::SalesAppError::Sale(
            domain_sales::SaleError::UnauthorizedPaymentDeclaration,
        ) => ApiError::unauthorized_payment_declaration(),
        _ => ApiError::bad_request("VALIDATION_ERROR", "Invalid sale request"),
    }
}

pub(crate) fn map_products_app_error(err: application::products::ProductsAppError) -> ApiError {
    match err {
        application::products::ProductsAppError::Inventory(_) => {
            ApiError::bad_request("VALIDATION_ERROR", "Invalid product data")
        }
        application::products::ProductsAppError::Domain(_) => ApiError::internal(),
    }
}

pub(crate) fn map_confirm_sale_error(err: infra_postgres::sales::ConfirmSaleError) -> ApiError {
    match err {
        infra_postgres::sales::ConfirmSaleError::InsufficientStock => {
            ApiError::insufficient_stock()
        }
        infra_postgres::sales::ConfirmSaleError::InvalidTransition => {
            ApiError::invalid_transition()
        }
        infra_postgres::sales::ConfirmSaleError::Database(_) => ApiError::internal(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_create_sale_request_schema_when_unknown_field_then_denied() {
        let json = r#"{"commerceId":"550e8400-e29b-41d4-a716-446655440000","items":[],"paymentMethod":"cash","extra":1}"#;
        let result = serde_json::from_str::<CreateSaleRequest>(json);
        assert!(result.is_err());
    }
}
