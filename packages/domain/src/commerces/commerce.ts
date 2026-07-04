import type { DomainEvent } from '../events/domain-event.js';
import { commerceCreated } from '../events/sale-events.js';
import type { Cnpj } from '../value-objects/cnpj.js';
import type { CommerceId, TenantId } from '../value-objects/ids.js';

export interface CommerceCreateInput {
  readonly id: CommerceId;
  readonly cnpj: Cnpj;
  readonly legalName: string;
  readonly tradeName?: string;
  readonly tenantId: TenantId;
}

/** Registered business client identified by CNPJ. */
export class Commerce {
  private readonly domainEvents: DomainEvent[] = [];

  private constructor(
    readonly id: CommerceId,
    readonly cnpj: Cnpj,
    readonly legalName: string,
    readonly tradeName: string | undefined,
    readonly tenantId: TenantId,
    private _active: boolean,
  ) {}

  static create(input: CommerceCreateInput): Commerce {
    const commerce = new Commerce(
      input.id,
      input.cnpj,
      input.legalName.trim(),
      input.tradeName?.trim(),
      input.tenantId,
      true,
    );
    commerce.raise(commerceCreated(commerce.id, commerce.cnpj.toString()));
    return commerce;
  }

  isActive(): boolean {
    return this._active;
  }

  deactivate(): Commerce {
    return new Commerce(this.id, this.cnpj, this.legalName, this.tradeName, this.tenantId, false);
  }

  pullDomainEvents(): DomainEvent[] {
    return [...this.domainEvents];
  }

  private raise(event: DomainEvent): void {
    this.domainEvents.push(event);
  }
}
