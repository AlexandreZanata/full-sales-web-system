import type { MessageKey, Messages } from '@/lib/i18n/messages';

export function translate(messages: Messages, key: MessageKey): string {
  const parts = key.split('.');
  let current: unknown = messages;
  for (const part of parts) {
    if (current && typeof current === 'object' && part in current) {
      current = (current as Record<string, unknown>)[part];
      continue;
    }
    return key;
  }
  return typeof current === 'string' ? current : key;
}

export function formatMessage(template: string, vars: Record<string, string | number>): string {
  return template.replace(/\{(\w+)\}/g, (_, name: string) => String(vars[name] ?? ''));
}
