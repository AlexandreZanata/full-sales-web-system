/** Matches API contract — snake_case pagination envelope */
export type CursorPaginationMeta = {
  next_cursor: string | null;
  has_more: boolean;
  limit: number;
};

export type CursorListResponse<T> = {
  data: T[];
  pagination: CursorPaginationMeta;
};

export type CursorListParams = {
  limit?: number;
  cursor?: string;
};

export async function fetchAllCursorPages<T>(
  fetchPage: (cursor?: string) => Promise<CursorListResponse<T>>,
  pageLimit = 100,
): Promise<T[]> {
  const all: T[] = [];
  let cursor: string | undefined;
  do {
    const page = await fetchPage(cursor);
    all.push(...page.data);
    if (!page.pagination.has_more || !page.pagination.next_cursor) {
      break;
    }
    cursor = page.pagination.next_cursor;
    if (page.data.length < pageLimit) {
      break;
    }
  } while (cursor);
  return all;
}
