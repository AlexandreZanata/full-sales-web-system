import type { DomainEvent } from './domain-event.js';
import { createEventMeta } from './domain-event.js';

export interface SaleCreated extends DomainEvent {
  readonly type: 'SaleCreated';
  readonly driverId: string;
  readonly commerceId: string;
}

export interface SaleConfirmed extends DomainEvent {
  readonly type: 'SaleConfirmed';
  readonly totalMinor: number;
  readonly currency: string;
}

export interface SaleCancelled extends DomainEvent {
  readonly type: 'SaleCancelled';
}

export interface CommerceCreated extends DomainEvent {
  readonly type: 'CommerceCreated';
  readonly cnpj: string;
}

export function saleCreated(
  aggregateId: string,
  driverId: string,
  commerceId: string,
  occurredAt?: Date,
): SaleCreated {
  return {
    ...createEventMeta(aggregateId, occurredAt),
    type: 'SaleCreated',
    driverId,
    commerceId,
  };
}

export function saleConfirmed(
  aggregateId: string,
  totalMinor: number,
  currency: string,
  occurredAt?: Date,
): SaleConfirmed {
  return {
    ...createEventMeta(aggregateId, occurredAt),
    type: 'SaleConfirmed',
    totalMinor,
    currency,
  };
}

export function saleCancelled(aggregateId: string, occurredAt?: Date): SaleCancelled {
  return {
    ...createEventMeta(aggregateId, occurredAt),
    type: 'SaleCancelled',
  };
}

export function commerceCreated(
  aggregateId: string,
  cnpj: string,
  occurredAt?: Date,
): CommerceCreated {
  return {
    ...createEventMeta(aggregateId, occurredAt),
    type: 'CommerceCreated',
    cnpj,
  };
}
