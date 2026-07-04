import type { MessageKey, Messages } from '@/lib/i18n/messages';

export function translate(messages: Messages, key: MessageKey): string {
  const [group, field] = key.split('.') as [keyof Messages, string];
  const section = messages[group];
  if (field in section) {
    return section[field as keyof typeof section];
  }
  return key;
}
