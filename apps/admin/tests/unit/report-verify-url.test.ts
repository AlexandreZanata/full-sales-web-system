/**
 * Contract: ADR-007 public verify URL shape.
 */
import { describe, expect, it } from 'vitest';

import { buildReportVerifyUrl } from '@/lib/reports/verifyUrl';

describe('buildReportVerifyUrl — Phase 35 contract', () => {
  it('given_report_id_when_build_then_public_verify_path', () => {
    const url = buildReportVerifyUrl('550e8400-e29b-41d4-a716-446655440000');
    expect(url).toBe('/v1/reports/550e8400-e29b-41d4-a716-446655440000/verify');
  });
});
