/** Maps sale action API error codes to user-facing messages (API-CONTRACT). */
export function saleActionErrorMessage(code: string): string {
  switch (code) {
    case 'INSUFFICIENT_STOCK':
      return 'Insufficient stock to confirm this sale.';
    case 'INVALID_TRANSITION':
      return 'This action is not allowed for the current sale status.';
    case 'INACTIVE_COMMERCE':
      return 'The selected commerce is inactive.';
    case 'INACTIVE_PRODUCT':
      return 'One or more products are inactive.';
    case 'PRODUCT_NOT_FOUND':
      return 'A selected product was not found.';
    case 'COMMERCE_NOT_FOUND':
      return 'The selected commerce was not found.';
    default:
      return 'Unable to complete this action. Please try again.';
  }
}
