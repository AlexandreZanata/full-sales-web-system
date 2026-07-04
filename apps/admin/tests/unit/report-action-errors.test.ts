/**
 * Contract: API-CONTRACT.md report error codes → i18n message keys.
 */
import { describe, expect, it } from 'vitest';

import { en } from '@/lib/i18n/locales/en';
import { translate } from '@/lib/i18n/translate';
import { reportActionErrorMessage } from '@/lib/reports/reportActionErrors';

const t = (key: Parameters<typeof translate>[1]) => translate(en, key);

describe('reportActionErrorMessage — Phase 35 contract', () => {
  it('given_signing_key_unavailable_when_map_then_user_message', () => {
    expect(t(reportActionErrorMessage('SIGNING_KEY_UNAVAILABLE'))).toContain('signing');
  });

  it('given_validation_error_when_map_then_user_message', () => {
    expect(t(reportActionErrorMessage('VALIDATION_ERROR'))).toContain('period');
  });
});
