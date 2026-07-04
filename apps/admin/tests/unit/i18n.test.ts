/**
 * Contract: Phase 38 i18n — en and pt-BR share identical key sets; translate resolves nested keys.
 */
import { describe, expect, it } from 'vitest';

import { en } from '@/lib/i18n/locales/en';
import { ptBR } from '@/lib/i18n/locales/pt-BR';
import { translate } from '@/lib/i18n/translate';

function collectLeafKeys(obj: Record<string, unknown>, prefix = ''): string[] {
  const keys: string[] = [];
  for (const [key, value] of Object.entries(obj)) {
    const path = prefix ? `${prefix}.${key}` : key;
    if (typeof value === 'string') {
      keys.push(path);
    } else if (value && typeof value === 'object') {
      keys.push(...collectLeafKeys(value as Record<string, unknown>, path));
    }
  }
  return keys.sort();
}

describe('i18n — Phase 38 contract', () => {
  it('translates_nav_keys_in_english', () => {
    expect(translate(en, 'nav.orders')).toBe('Orders');
  });

  it('translates_nav_keys_in_portuguese', () => {
    expect(translate(ptBR, 'nav.orders')).toBe('Pedidos');
  });

  it('translates_nested_status_keys', () => {
    expect(translate(en, 'status.order.PendingApproval')).toBe('Pending approval');
    expect(translate(ptBR, 'status.order.PendingApproval')).toBe('Aguardando aprovação');
  });

  it('provides_identical_key_sets_in_both_locales', () => {
    const enKeys = collectLeafKeys(en as unknown as Record<string, unknown>);
    const ptKeys = collectLeafKeys(ptBR as unknown as Record<string, unknown>);
    expect(ptKeys).toEqual(enKeys);
    expect(enKeys.length).toBeGreaterThanOrEqual(200);
  });

  it('resolves_every_key_in_both_locales', () => {
    const enKeys = collectLeafKeys(en as unknown as Record<string, unknown>);
    for (const key of enKeys) {
      expect(translate(en, key as Parameters<typeof translate>[1])).not.toBe(key);
      expect(translate(ptBR, key as Parameters<typeof translate>[1])).not.toBe(key);
    }
  });
});
