import type { QueryClient } from '@tanstack/react-query';

import type { CategorySummary } from '@/lib/api/types';
import type { CursorListResponse } from '@/lib/cursorPagination';

export function patchCategoryInListCaches(
  queryClient: QueryClient,
  updated: CategorySummary,
): void {
  queryClient.setQueriesData<CursorListResponse<CategorySummary>>(
    { queryKey: ['categories'] },
    (current) => {
      if (!current) {
        return current;
      }
      return {
        ...current,
        data: current.data.map((row) => (row.id === updated.id ? { ...row, ...updated } : row)),
      };
    },
  );
}
