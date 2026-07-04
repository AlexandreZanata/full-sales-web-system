import type { Commerce } from '../commerces/commerce.js';
import type { DomainEvent } from '../events/domain-event.js';
import { saleCancelled, saleConfirmed, saleCreated } from '../events/sale-events.js';
import {
  EmptySaleError,
  InactiveCommerceError,
  InactiveProductError,
  InvalidSaleTransitionError,
} from '../errors/domain-error.js';
import type { Product } from '../inventory/product.js';
import { PaymentMethod } from '../shared/payment-method.js';
import { Currency } from '../value-objects/currency.js';
import type { CommerceId, SaleId, TenantId, UserId } from '../value-objects/ids.js';
import { Money } from '../value-objects/money.js';
import { Quantity } from '../value-objects/quantity.js';
import { SaleItem } from './sale-item.js';
import { SaleStatus } from './sale-status.js';

export interface SaleCreateInput {
  readonly id: SaleId;
  readonly driverId: UserId;
  readonly commerce: Commerce;
  readonly paymentMethod: PaymentMethod;
  readonly tenantId: TenantId;
  readonly createdAt?: Date;
}

export interface AddSaleItemInput {
  readonly product: Product;
  readonly quantity: Quantity;
}

/** Commercial transaction aggregate — Pending | Confirmed | Cancelled. */
export class Sale {
  private readonly domainEvents: DomainEvent[] = [];

  private constructor(
    readonly id: SaleId,
    readonly driverId: UserId,
    readonly commerceId: CommerceId,
    readonly paymentMethod: PaymentMethod,
    readonly tenantId: TenantId,
    readonly createdAt: Date,
    private _status: SaleStatus,
    private readonly _items: SaleItem[],
  ) {}

  static create(input: SaleCreateInput): Sale {
    if (!input.commerce.isActive()) {
      throw new InactiveCommerceError();
    }
    const createdAt = input.createdAt ?? new Date();
    const sale = new Sale(
      input.id,
      input.driverId,
      input.commerce.id,
      input.paymentMethod,
      input.tenantId,
      createdAt,
      SaleStatus.Pending,
      [],
    );
    sale.raise(saleCreated(sale.id, sale.driverId, sale.commerceId, createdAt));
    return sale;
  }

  get status(): SaleStatus {
    return this._status;
  }

  get items(): readonly SaleItem[] {
    return this._items;
  }

  total(): Money {
    const currency = Currency.brl();
    return this._items.reduce((sum, item) => sum.add(item.lineTotal), Money.of(0, currency));
  }

  addItem(input: AddSaleItemInput): Sale {
    this.assertCanTransition(SaleStatus.Pending);
    if (!input.product.isActive()) {
      throw new InactiveProductError();
    }
    const item = SaleItem.create(input.product.id, input.quantity, input.product.unitPrice);
    return new Sale(
      this.id,
      this.driverId,
      this.commerceId,
      this.paymentMethod,
      this.tenantId,
      this.createdAt,
      this._status,
      [...this._items, item],
    );
  }

  confirm(): Sale {
    this.assertCanTransition(SaleStatus.Confirmed);
    if (this._items.length === 0) {
      throw new EmptySaleError();
    }
    const total = this.total();
    const confirmed = new Sale(
      this.id,
      this.driverId,
      this.commerceId,
      this.paymentMethod,
      this.tenantId,
      this.createdAt,
      SaleStatus.Confirmed,
      this._items,
    );
    confirmed.raise(saleConfirmed(confirmed.id, total.amountMinor, total.currency.toString()));
    return confirmed;
  }

  cancel(): Sale {
    this.assertCanTransition(SaleStatus.Cancelled);
    const cancelled = new Sale(
      this.id,
      this.driverId,
      this.commerceId,
      this.paymentMethod,
      this.tenantId,
      this.createdAt,
      SaleStatus.Cancelled,
      this._items,
    );
    cancelled.raise(saleCancelled(cancelled.id));
    return cancelled;
  }

  pullDomainEvents(): DomainEvent[] {
    return [...this.domainEvents];
  }

  private assertCanTransition(target: SaleStatus): void {
    if (this._status !== SaleStatus.Pending) {
      throw new InvalidSaleTransitionError(this._status, target);
    }
  }

  private raise(event: DomainEvent): void {
    this.domainEvents.push(event);
  }
}
