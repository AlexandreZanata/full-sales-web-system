/** Maps order action API error codes to user-facing messages (API-CONTRACT). */
export function orderActionErrorMessage(code: string): string {
  switch (code) {
    case 'INSUFFICIENT_STOCK':
      return 'Insufficient stock to approve this order.';
    case 'INVALID_ORDER_TRANSITION':
      return 'This action is not allowed for the current order status.';
    case 'REJECTION_REASON_REQUIRED':
      return 'A rejection reason is required.';
    case 'DELIVERY_EXISTS':
      return 'This order already has a delivery assigned.';
    default:
      return 'Unable to complete this action. Please try again.';
  }
}
