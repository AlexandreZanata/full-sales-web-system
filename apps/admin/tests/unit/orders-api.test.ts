/**
 * Contract: Phase 68D — admin orders list cursor query builder.
 */
import { describe, expect, it } from 'vitest';

import { dateFilterToIso } from '@/lib/api/orders';

describe('orders list API — cursor filter contract', () => {
  it('given_date_when_start_then_midnight_utc', () => {
    expect(dateFilterToIso('2026-07-04', 'start')).toBe('2026-07-04T00:00:00.000Z');
  });

  it('given_date_when_end_then_end_of_day_utc', () => {
    expect(dateFilterToIso('2026-07-04', 'end')).toBe('2026-07-04T23:59:59.999Z');
  });
});
