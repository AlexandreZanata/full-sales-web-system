import { describe, expect, it } from 'vitest';

import { localeFromBrowserLanguage } from '@/lib/i18n/storage';

describe('localeFromBrowserLanguage', () => {
  it('maps_english_and_portuguese_prefixes', () => {
    expect(localeFromBrowserLanguage('en')).toBe('en');
    expect(localeFromBrowserLanguage('en-US')).toBe('en');
    expect(localeFromBrowserLanguage('pt')).toBe('pt-BR');
    expect(localeFromBrowserLanguage('pt-BR')).toBe('pt-BR');
    expect(localeFromBrowserLanguage('pt-PT')).toBe('pt-BR');
  });

  it('defaults_unrecognized_to_portuguese_never_english', () => {
    expect(localeFromBrowserLanguage('es-ES')).toBe('pt-BR');
    expect(localeFromBrowserLanguage('fr')).toBe('pt-BR');
    expect(localeFromBrowserLanguage('')).toBe('pt-BR');
    expect(localeFromBrowserLanguage(undefined)).toBe('pt-BR');
    expect(localeFromBrowserLanguage(null)).toBe('pt-BR');
  });
});
