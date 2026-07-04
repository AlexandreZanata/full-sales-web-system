import { describe, expect, it } from 'vitest';

import { Commerce } from '../commerces/commerce.js';
import {
  EmptySaleError,
  InactiveCommerceError,
  InactiveProductError,
  InvalidSaleTransitionError,
} from '../errors/domain-error.js';
import { Product } from '../inventory/product.js';
import { PaymentMethod } from '../shared/payment-method.js';
import { Cnpj } from '../value-objects/cnpj.js';
import { Currency } from '../value-objects/currency.js';
import {
  parseCommerceId,
  parseProductId,
  parseSaleId,
  parseTenantId,
  parseUserId,
} from '../value-objects/ids.js';
import { Money } from '../value-objects/money.js';
import { Quantity } from '../value-objects/quantity.js';
import { Sku } from '../value-objects/sku.js';
import { Sale } from './sale.js';
import { SaleStatus } from './sale-status.js';

const tenantId = parseTenantId('550e8400-e29b-41d4-a716-446655440000');
const driverId = parseUserId('550e8400-e29b-41d4-a716-446655440001');
const commerceId = parseCommerceId('550e8400-e29b-41d4-a716-446655440002');
const productId = parseProductId('550e8400-e29b-41d4-a716-446655440003');
const saleId = parseSaleId('550e8400-e29b-41d4-a716-446655440004');

function activeCommerce(): Commerce {
  return Commerce.create({
    id: commerceId,
    cnpj: Cnpj.parse('11444777000161'),
    legalName: 'Acme Ltda',
    tenantId,
  });
}

function activeProduct(): Product {
  return Product.create({
    id: productId,
    name: 'Widget',
    sku: Sku.parse('WGT-001'),
    unitPrice: Money.of(1_000, Currency.brl()),
    tenantId,
  });
}

function pendingSaleWithItem(): Sale {
  return Sale.create({
    id: saleId,
    driverId,
    commerce: activeCommerce(),
    paymentMethod: PaymentMethod.Cash,
    tenantId,
  }).addItem({ product: activeProduct(), quantity: Quantity.of(2) });
}

describe('Sale', () => {
  it('given_active_commerce_when_create_then_pending_with_sale_created_event', () => {
    const sale = Sale.create({
      id: saleId,
      driverId,
      commerce: activeCommerce(),
      paymentMethod: PaymentMethod.Pix,
      tenantId,
    });
    expect(sale.status).toBe(SaleStatus.Pending);
    const events = sale.pullDomainEvents();
    expect(events).toHaveLength(1);
    expect(events[0]?.type).toBe('SaleCreated');
    expect(events[0]?.aggregateId).toBe(saleId);
  });

  it('given_inactive_commerce_when_create_then_inactive_commerce', () => {
    const inactive = activeCommerce().deactivate();
    expect(() =>
      Sale.create({
        id: saleId,
        driverId,
        commerce: inactive,
        paymentMethod: PaymentMethod.Cash,
        tenantId,
      }),
    ).toThrow(InactiveCommerceError);
  });

  // Contract: docs/STATE-MACHINES.md — Pending → Confirmed via confirm().
  it('given_pending_sale_with_items_when_confirm_then_confirmed_and_event_raised_once', () => {
    const sale = pendingSaleWithItem().confirm();
    expect(sale.status).toBe(SaleStatus.Confirmed);
    expect(sale.total().amountMinor).toBe(2_000);
    const events = sale.pullDomainEvents();
    expect(events).toHaveLength(1);
    expect(events[0]?.type).toBe('SaleConfirmed');
  });

  // Contract: BR-SA-001 — cannot confirm empty sale.
  it('BR-SA-001_given_empty_sale_when_confirm_then_empty_sale', () => {
    const sale = Sale.create({
      id: saleId,
      driverId,
      commerce: activeCommerce(),
      paymentMethod: PaymentMethod.Cash,
      tenantId,
    });
    expect(() => sale.confirm()).toThrow(EmptySaleError);
    expect(sale.status).toBe(SaleStatus.Pending);
  });

  // Contract: BR-SA-003 — cancel pending sale only.
  it('BR-SA-003_given_confirmed_sale_when_cancel_then_invalid_sale_transition', () => {
    const confirmed = pendingSaleWithItem().confirm();
    expect(() => confirmed.cancel()).toThrow(InvalidSaleTransitionError);
  });

  it('given_pending_sale_when_cancel_then_cancelled_with_event', () => {
    const cancelled = pendingSaleWithItem().cancel();
    expect(cancelled.status).toBe(SaleStatus.Cancelled);
    expect(cancelled.pullDomainEvents()[0]?.type).toBe('SaleCancelled');
  });

  // Contract: STATE-MACHINES — Confirmed → Confirmed (re-confirm) fails.
  it('given_confirmed_sale_when_confirm_again_then_invalid_sale_transition', () => {
    const confirmed = pendingSaleWithItem().confirm();
    expect(() => confirmed.confirm()).toThrow(InvalidSaleTransitionError);
  });

  // Contract: STATE-MACHINES — Cancelled → Confirmed fails.
  it('given_cancelled_sale_when_confirm_then_invalid_sale_transition', () => {
    const cancelled = pendingSaleWithItem().cancel();
    expect(() => cancelled.confirm()).toThrow(InvalidSaleTransitionError);
  });

  // Contract: BR-IN-003 — inactive product cannot be sold.
  it('BR-IN-003_given_inactive_product_when_add_item_then_inactive_product', () => {
    const sale = Sale.create({
      id: saleId,
      driverId,
      commerce: activeCommerce(),
      paymentMethod: PaymentMethod.Cash,
      tenantId,
    });
    const inactive = activeProduct().deactivate();
    expect(() => sale.addItem({ product: inactive, quantity: Quantity.of(1) })).toThrow(
      InactiveProductError,
    );
  });

  // Contract: BR-SA-002 — total always from items, never external input.
  it('BR-SA-002_given_items_when_total_then_sum_of_line_totals', () => {
    const sale = pendingSaleWithItem();
    expect(sale.total().amountMinor).toBe(2_000);
  });
});
