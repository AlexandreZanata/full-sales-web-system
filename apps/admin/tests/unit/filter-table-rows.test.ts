/**
 * Contract: DataTable client search matches any primitive field on the row (case-insensitive).
 */
import { describe, expect, it } from 'vitest';

import { filterTableRows, rowMatchesSearch } from '@/lib/table/filterTableRows';

const rows = [
  { id: '1', name: 'Seed Store', sku: 'SEED-003', active: true },
  { id: '2', name: 'Beta Market', sku: 'SKU-99', active: false },
];

describe('filterTableRows', () => {
  it('given_name_substring_when_filter_then_matching_rows', () => {
    expect(filterTableRows(rows, 'seed').map((row) => row.id)).toEqual(['1']);
  });

  it('given_sku_when_filter_then_matching_rows', () => {
    expect(filterTableRows(rows, 'sku-99').map((row) => row.id)).toEqual(['2']);
  });

  it('given_empty_query_when_filter_then_all_rows', () => {
    expect(filterTableRows(rows, '  ')).toHaveLength(2);
  });

  it('given_custom_getSearchText_when_filter_then_uses_extractor', () => {
    const result = filterTableRows(rows, 'market', (row) => row.name);
    expect(result.map((row) => row.id)).toEqual(['2']);
  });

  it('given_boolean_field_when_match_then_true', () => {
    expect(rowMatchesSearch(rows[0], 'true')).toBe(true);
  });
});
