import type { MessageKey } from '@/lib/i18n/messages';
import type { TablePaginationState } from '@/lib/tablePagination';

type Translate = (key: MessageKey) => string;

export function formatPaginationSummary(t: Translate, state: TablePaginationState): string {
  return t('common.pagination.summary')
    .replace('{page}', String(state.page))
    .replace('{totalPages}', String(state.totalPages))
    .replace('{total}', String(state.total));
}

export function formatMoneyMinor(amountMinor: number, currency: string): string {
  return new Intl.NumberFormat(undefined, {
    style: 'currency',
    currency,
  }).format(amountMinor / 100);
}
