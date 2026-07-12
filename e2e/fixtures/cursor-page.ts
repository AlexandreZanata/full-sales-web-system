/** Cursor list envelope — matches API-CONTRACT / field client CursorListResponse. */
export function cursorPage<T>(
  data: T[],
  limit = 20,
): { data: T[]; pagination: { next_cursor: null; has_more: false; limit: number } } {
  return {
    data,
    pagination: { next_cursor: null, has_more: false, limit },
  };
}
