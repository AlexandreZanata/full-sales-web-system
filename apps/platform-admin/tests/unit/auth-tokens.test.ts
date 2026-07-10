/**
 * Contract: Phase 11 — platform storage keys isolated from tenant admin app.
 */
import { afterEach, describe, expect, it } from 'vitest';

import {
  clearTokens,
  getAccessToken,
  getRefreshToken,
  hasSession,
  setTokens,
} from '@/lib/auth/tokens';

describe('platform auth tokens — Phase 11 contract', () => {
  afterEach(() => {
    clearTokens();
  });

  it('persists_tokens_in_sessionStorage', () => {
    setTokens({ accessToken: 'access', refreshToken: 'refresh', expiresIn: 900 });
    expect(hasSession()).toBe(true);
    expect(getAccessToken()).toBe('access');
    expect(getRefreshToken()).toBe('refresh');
  });

  it('uses_platform_specific_storage_keys', () => {
    setTokens({ accessToken: 'a', refreshToken: 'r', expiresIn: 60 });
    expect(sessionStorage.getItem('full_sales_platform_access_token')).toBe('a');
    expect(sessionStorage.getItem('full_sales_admin_access_token')).toBeNull();
  });
});
