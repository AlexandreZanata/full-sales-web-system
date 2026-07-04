import { describe, expect, it } from 'vitest';

import { getSaleStatusToken } from '@/lib/client-tokens';

describe('getSaleStatusToken', () => {
  it('given_confirmed_when_resolved_then_uses_active_palette', () => {
    const token = getSaleStatusToken('Confirmed');
    expect(token.dot).toBe('bg-status-active');
    expect(token.badge).toContain('text-status-active');
  });

  it('given_unknown_status_when_resolved_then_falls_back_to_neutral', () => {
    const token = getSaleStatusToken('Unknown');
    expect(token.dot).toBe('bg-status-neutral');
  });
});
