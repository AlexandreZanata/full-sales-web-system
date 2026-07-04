import { InvalidUuidError } from '../errors/domain-error.js';

const UUID_PATTERN = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;

export function parseUuid(value: string, label: string): string {
  if (!UUID_PATTERN.test(value)) {
    throw new InvalidUuidError(label);
  }
  return value.toLowerCase();
}

export function generateUuid(): string {
  return crypto.randomUUID();
}
