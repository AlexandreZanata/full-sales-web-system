/**
 * Contract: Phase 36 i18n — en and pt-BR share the same message keys.
 */
import { describe, expect, it } from 'vitest';

import { en } from '@/lib/i18n/locales/en';
import { ptBR } from '@/lib/i18n/locales/pt-BR';
import { translate } from '@/lib/i18n/translate';

describe('i18n — Phase 36 contract', () => {
  it('translates_nav_keys_in_english', () => {
    expect(translate(en, 'nav.orders')).toBe('Orders');
  });

  it('translates_nav_keys_in_portuguese', () => {
    expect(translate(ptBR, 'nav.orders')).toBe('Pedidos');
  });

  it('provides_matching_key_groups_in_both_locales', () => {
    expect(Object.keys(ptBR.nav)).toEqual(Object.keys(en.nav));
    expect(Object.keys(ptBR.auth)).toEqual(Object.keys(en.auth));
    expect(Object.keys(ptBR.common)).toEqual(Object.keys(en.common));
  });
});
