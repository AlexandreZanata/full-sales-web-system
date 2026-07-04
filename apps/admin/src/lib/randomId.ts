export function randomId(): string {
  if (typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function') {
    return crypto.randomUUID();
  }
  return `id-${String(Date.now())}-${Math.random().toString(36).slice(2, 9)}`;
}
