export type PaginationMeta = {
  page: number;
  totalPages: number;
  total: number;
  pageSize?: number;
};

export type TablePaginationState = Pick<PaginationMeta, 'page' | 'totalPages' | 'total'>;

export function formatTablePaginationSummary(state: TablePaginationState): string {
  return `Page ${String(state.page)} of ${String(state.totalPages)} (${String(state.total)} total)`;
}

export function toTablePagination(meta: PaginationMeta): TablePaginationState {
  return {
    page: meta.page,
    totalPages: meta.totalPages,
    total: meta.total,
  };
}
