import { SaleStatus } from '@full-sales/domain';

/** Ensures application layer depends on domain package only (scaffold check). */
export function applicationDomainRef(): typeof SaleStatus.Pending {
  return SaleStatus.Pending;
}
