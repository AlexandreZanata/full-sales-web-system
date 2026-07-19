/**
 * Contract: commerce picker search matches trade name, legal name, or CNPJ.
 */
import { describe, expect, it } from 'vitest';

import type { CommerceSummary } from '@/lib/api/types';
import {
  filterCommercesBySearch,
  formatCommerceOption,
} from '@/lib/commerces/filterCommercesBySearch';

function commerce(
  partial: Pick<CommerceSummary, 'id' | 'cnpj' | 'legalName' | 'tradeName'>,
): CommerceSummary {
  return { ...partial, active: true };
}

const catalog = [
  commerce({
    id: '1',
    cnpj: '11222333000181',
    legalName: 'Seed Store Ltda',
    tradeName: 'Seed Store',
  }),
  commerce({
    id: '2',
    cnpj: '22333444000105',
    legalName: 'Beta Market Ltda',
    tradeName: 'Beta Market',
  }),
];

describe('filterCommercesBySearch', () => {
  it('given_trade_name_when_search_then_match', () => {
    expect(filterCommercesBySearch(catalog, 'seed').map((row) => row.id)).toEqual(['1']);
  });

  it('given_cnpj_digits_when_search_then_match', () => {
    expect(filterCommercesBySearch(catalog, '33444000').map((row) => row.id)).toEqual(['2']);
  });

  it('given_empty_query_when_search_then_all', () => {
    expect(filterCommercesBySearch(catalog, '  ')).toHaveLength(2);
  });

  it('formats_commerce_option_with_name_and_cnpj', () => {
    expect(formatCommerceOption(catalog[0]!)).toBe('Seed Store · 11222333000181');
  });
});
