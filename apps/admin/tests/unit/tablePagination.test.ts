import { describe, expect, it } from 'vitest';

import { formatTablePaginationSummary, toTablePagination } from '@/lib/tablePagination';

describe('tablePagination', () => {
  it('formats pagination summary for the table footer', () => {
    expect(formatTablePaginationSummary({ page: 2, totalPages: 5, total: 87 })).toBe(
      'Page 2 of 5 (87 total)',
    );
  });

  it('maps API pagination meta', () => {
    expect(toTablePagination({ page: 1, totalPages: 3, total: 42, pageSize: 20 })).toEqual({
      page: 1,
      totalPages: 3,
      total: 42,
    });
  });
});
