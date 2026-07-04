//! RN2 stock reservation lifecycle — contract from BUSINESS-RULES-EXPANSION.

use domain_inventory::{
    CreateStockReservationInput, InventoryError, ProductId, Quantity, ReservationId,
    ReservationStatus, StockReservation,
};
use domain_shared::TenantId;
use uuid::Uuid;

fn reservation_input(
    balance: i32,
    reserved: i32,
    qty: i32,
) -> CreateStockReservationInput {
    CreateStockReservationInput {
        id: ReservationId::generate(),
        tenant_id: TenantId::generate(),
        order_id: Uuid::now_v7(),
        order_item_id: Uuid::now_v7(),
        product_id: ProductId::generate(),
        driver_id: None,
        quantity: Quantity::of(qty).expect("qty"),
        balance_total: balance,
        active_reserved: reserved,
    }
}

#[test]
fn given_available_stock_when_reserve_then_active() {
    let reservation = StockReservation::reserve(reservation_input(100, 20, 30))
        .expect("reserve");
    assert_eq!(reservation.status(), ReservationStatus::Active);
    assert_eq!(reservation.quantity().value(), 30);
}

#[test]
fn given_insufficient_available_when_reserve_then_rejected() {
    let err = StockReservation::reserve(reservation_input(10, 5, 8)).expect_err("oversell");
    assert_eq!(err, InventoryError::InsufficientAvailableStock);
}

#[test]
fn given_active_reservation_when_release_then_released() {
    let reservation = StockReservation::reserve(reservation_input(50, 0, 10))
        .expect("reserve")
        .release()
        .expect("release");
    assert_eq!(reservation.status(), ReservationStatus::Released);
}

#[test]
fn given_active_reservation_when_consume_then_consumed() {
    let reservation = StockReservation::reserve(reservation_input(50, 0, 10))
        .expect("reserve")
        .consume()
        .expect("consume");
    assert_eq!(reservation.status(), ReservationStatus::Consumed);
}

#[test]
fn given_released_reservation_when_consume_then_invalid_transition() {
    let reservation = StockReservation::reserve(reservation_input(50, 0, 10))
        .expect("reserve")
        .release()
        .expect("release");
    let err = reservation.consume().expect_err("must fail");
    assert_eq!(
        err,
        InventoryError::InvalidReservationTransition {
            from: ReservationStatus::Released,
            to: ReservationStatus::Consumed,
        }
    );
}
