/** Sale lifecycle states per docs/STATE-MACHINES.md */
export enum SaleStatus {
  Pending = 'Pending',
  Confirmed = 'Confirmed',
  Cancelled = 'Cancelled',
}

export function isTerminalSaleStatus(status: SaleStatus): boolean {
  return status === SaleStatus.Confirmed || status === SaleStatus.Cancelled;
}
