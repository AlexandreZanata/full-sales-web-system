import { describe, expect, it } from 'vitest';

import { getOrderStatusToken } from '@/lib/client-tokens';

describe('getOrderStatusToken', () => {
  it('given_pendingApproval_when_resolved_then_uses_warning_palette', () => {
    const token = getOrderStatusToken('PendingApproval');
    expect(token.dot).toBe('bg-status-warning');
    expect(token.badge).toContain('text-status-warning');
  });

  it('given_unknown_status_when_resolved_then_falls_back_to_neutral', () => {
    const token = getOrderStatusToken('Unknown');
    expect(token.dot).toBe('bg-status-neutral');
  });
});
