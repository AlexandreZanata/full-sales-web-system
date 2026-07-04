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

export function saleStatusLabel(messages: Messages, status: string): string {
  const key = `status.${status}` as MessageKey;
  const label = translate(messages, key);
  return label === key ? status : label;
}

export function paymentMethodLabel(messages: Messages, method: string): string {
  const key = `paymentMethods.${method}` as MessageKey;
  const label = translate(messages, key);
  return label === key ? method : label;
}

export function deliveryStatusLabel(messages: Messages, status: string): string {
  const key = `deliveryStatus.${status}` as MessageKey;
  const label = translate(messages, key);
  return label === key ? status : label;
}
