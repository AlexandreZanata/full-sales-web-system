import { generateUuid, parseUuid } from './uuid-parse.js';

export type TenantId = string & { readonly __brand: 'TenantId' };
export type UserId = string & { readonly __brand: 'UserId' };
export type CommerceId = string & { readonly __brand: 'CommerceId' };
export type ProductId = string & { readonly __brand: 'ProductId' };
export type SaleId = string & { readonly __brand: 'SaleId' };

export function parseTenantId(value: string): TenantId {
  return parseUuid(value, 'tenant id') as TenantId;
}

export function parseUserId(value: string): UserId {
  return parseUuid(value, 'user id') as UserId;
}

export function parseCommerceId(value: string): CommerceId {
  return parseUuid(value, 'commerce id') as CommerceId;
}

export function parseProductId(value: string): ProductId {
  return parseUuid(value, 'product id') as ProductId;
}

export function parseSaleId(value: string): SaleId {
  return parseUuid(value, 'sale id') as SaleId;
}

export function generateSaleId(): SaleId {
  return generateUuid() as SaleId;
}

export function generateCommerceId(): CommerceId {
  return generateUuid() as CommerceId;
}

export function generateProductId(): ProductId {
  return generateUuid() as ProductId;
}
