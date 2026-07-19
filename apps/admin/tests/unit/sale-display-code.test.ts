/**
 * Contract: admin sale labels prefer alphanumeric displayCode over UUID prefix.
 */
import { describe, expect, it } from 'vitest';

import { saleDisplayCode } from '@/lib/sales/saleDisplayCode';

describe('saleDisplayCode', () => {
  it('given_displayCode_when_label_then_uses_code', () => {
    expect(
      saleDisplayCode({ id: '019f7be8-aaaa-bbbb-cccc-ddddeeeeffff', displayCode: '00000008' }),
    ).toBe('00000008');
  });

  it('given_missing_displayCode_when_label_then_truncated_uuid', () => {
    expect(saleDisplayCode({ id: '019f7be8-aaaa-bbbb-cccc-ddddeeeeffff' })).toBe('019f7be8…');
  });
});
