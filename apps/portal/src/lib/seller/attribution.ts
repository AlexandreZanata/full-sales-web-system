export type SellerAttribution = {
  publicCode: string;
  displayName: string;
  contactPhone?: string;
};

const STORAGE_KEY = 'portal.sellerAttribution';

let cachedRaw: string | null | undefined;
let cachedValue: SellerAttribution | null = null;
const listeners = new Set<() => void>();

function notify(): void {
  for (const listener of listeners) {
    listener();
  }
}

function parseAttribution(raw: string): SellerAttribution | null {
  try {
    const parsed = JSON.parse(raw) as SellerAttribution;
    if (!parsed.publicCode || !parsed.displayName) {
      return null;
    }
    return parsed;
  } catch {
    return null;
  }
}

/** Stable snapshot for useSyncExternalStore — same reference until storage changes. */
export function readSellerAttribution(): SellerAttribution | null {
  if (typeof sessionStorage === 'undefined') {
    return null;
  }
  const raw = sessionStorage.getItem(STORAGE_KEY);
  if (raw === cachedRaw) {
    return cachedValue;
  }
  cachedRaw = raw;
  cachedValue = raw ? parseAttribution(raw) : null;
  return cachedValue;
}

export function writeSellerAttribution(value: SellerAttribution): void {
  const raw = JSON.stringify(value);
  sessionStorage.setItem(STORAGE_KEY, raw);
  cachedRaw = raw;
  cachedValue = value;
  notify();
}

export function clearSellerAttribution(): void {
  sessionStorage.removeItem(STORAGE_KEY);
  cachedRaw = null;
  cachedValue = null;
  notify();
}

/** Subscribe to same-tab writes + cross-tab `storage` events. */
export function subscribeSellerAttribution(onStoreChange: () => void): () => void {
  listeners.add(onStoreChange);
  const onStorage = (event: StorageEvent) => {
    if (event.key === STORAGE_KEY || event.key === null) {
      cachedRaw = undefined;
      onStoreChange();
    }
  };
  window.addEventListener('storage', onStorage);
  return () => {
    listeners.delete(onStoreChange);
    window.removeEventListener('storage', onStorage);
  };
}

/** Contact precedence: attributed seller phone → tenant phone. */
export function resolveContactPhone(
  attribution: SellerAttribution | null,
  tenantPhone?: string,
): string | undefined {
  const seller = attribution?.contactPhone?.trim();
  if (seller) {
    return seller;
  }
  const tenant = tenantPhone?.trim();
  return tenant || undefined;
}

/** Test helper — reset module cache between cases. */
export function resetSellerAttributionCacheForTests(): void {
  cachedRaw = undefined;
  cachedValue = null;
  listeners.clear();
}
