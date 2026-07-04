import type { MessageKey, Messages } from '@/lib/i18n/messages';

export function translate(messages: Messages, key: MessageKey): string {
  const parts = key.split('.');
  let current: unknown = messages;

  for (const part of parts) {
    if (current && typeof current === 'object' && part in current) {
      current = (current as Record<string, unknown>)[part];
      continue;
    }
    if (import.meta.env.DEV) {
      console.warn(`Missing i18n key: ${key}`);
    }
    return key;
  }

  return typeof current === 'string' ? current : key;
}
