// Value objects
export { Currency } from './value-objects/currency.js';
export { Money } from './value-objects/money.js';
export { Quantity } from './value-objects/quantity.js';
export { Cnpj } from './value-objects/cnpj.js';
export { Sku } from './value-objects/sku.js';
export { Email } from './value-objects/email.js';
export { FullName } from './value-objects/full-name.js';
export {
  parseTenantId,
  parseUserId,
  parseCommerceId,
  parseProductId,
  parseSaleId,
  generateSaleId,
  generateCommerceId,
  generateProductId,
  generateTenantId,
  generateUserId,
} from './value-objects/ids.js';
export type { TenantId, UserId, CommerceId, ProductId, SaleId } from './value-objects/ids.js';

// Shared enums
export { PaymentMethod, parsePaymentMethod } from './shared/payment-method.js';

// Aggregates
export { Commerce } from './commerces/commerce.js';
export type { CommerceCreateInput } from './commerces/commerce.js';
export { Product } from './inventory/product.js';
export type { ProductCreateInput } from './inventory/product.js';
export { Sale } from './sales/sale.js';
export type { SaleCreateInput, AddSaleItemInput } from './sales/sale.js';
export { SaleItem } from './sales/sale-item.js';
export { SaleStatus } from './sales/sale-status.js';

// Domain events
export type { DomainEvent } from './events/domain-event.js';
export type {
  SaleCreated,
  SaleConfirmed,
  SaleCancelled,
  CommerceCreated,
} from './events/sale-events.js';

// Errors
export {
  DomainError,
  NegativeMoneyAmountError,
  InvalidCurrencyError,
  CurrencyMismatchError,
  MoneyOverflowError,
  InvalidQuantityError,
  InvalidUuidError,
  InvalidCnpjError,
  InvalidSkuError,
  InvalidEmailError,
  InvalidFullNameError,
  InactiveProductError,
  InactiveCommerceError,
  EmptySaleError,
  InvalidSaleTransitionError,
} from './errors/domain-error.js';
