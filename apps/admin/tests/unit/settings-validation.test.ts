/**
 * Contract: Phase 41 — settings form validation.
 */
import { describe, expect, it } from 'vitest';

import { hasSiteIdentityErrors, validateSiteIdentityForm } from '@/lib/settings/validation';

describe('validateSiteIdentityForm — Phase 41 contract', () => {
  it('given_empty_display_name_when_validate_then_name_required', () => {
    const errors = validateSiteIdentityForm({ displayName: '   ' });
    expect(errors.displayName).toBe('forms.validation.nameRequired');
  });

  it('given_long_display_name_when_validate_then_too_long', () => {
    const errors = validateSiteIdentityForm({ displayName: 'x'.repeat(201) });
    expect(errors.displayName).toBe('settings.validation.displayNameTooLong');
  });

  it('given_valid_display_name_when_validate_then_no_errors', () => {
    const errors = validateSiteIdentityForm({ displayName: 'Dev Sales Platform' });
    expect(hasSiteIdentityErrors(errors)).toBe(false);
  });
});
