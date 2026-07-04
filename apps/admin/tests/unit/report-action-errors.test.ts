/**
 * Contract: API-CONTRACT.md report error codes.
 */
import { describe, expect, it } from 'vitest';

import { reportActionErrorMessage } from '@/lib/reports/reportActionErrors';

describe('reportActionErrorMessage — Phase 35 contract', () => {
  it('given_signing_key_unavailable_when_map_then_user_message', () => {
    expect(reportActionErrorMessage('SIGNING_KEY_UNAVAILABLE')).toContain('signing');
  });

  it('given_validation_error_when_map_then_user_message', () => {
    expect(reportActionErrorMessage('VALIDATION_ERROR')).toContain('period');
  });
});
