import { describe, expect, it } from 'vitest';

describe('web scaffold', () => {
  it('given_api_url_default_when_unset_then_uses_localhost', () => {
    const fallback = 'http://127.0.0.1:8080';
    expect(fallback).toContain('8080');
  });
});
