/** Immutable past-tense domain fact raised by aggregates. */
export interface DomainEvent {
  readonly id: string;
  readonly occurredAt: Date;
  readonly aggregateId: string;
  readonly type: string;
}

export function createEventMeta(
  aggregateId: string,
  occurredAt: Date = new Date(),
): {
  id: string;
  occurredAt: Date;
  aggregateId: string;
} {
  return {
    id: crypto.randomUUID(),
    occurredAt,
    aggregateId,
  };
}
