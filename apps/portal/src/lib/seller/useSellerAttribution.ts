import { useSyncExternalStore } from 'react';

import {
  clearSellerAttribution,
  readSellerAttribution,
  subscribeSellerAttribution,
  type SellerAttribution,
} from '@/lib/seller/attribution';

/** Session attribution for the current tab (Phase 19). */
export function useSellerAttribution(): {
  attribution: SellerAttribution | null;
  clear: () => void;
} {
  const attribution = useSyncExternalStore(
    subscribeSellerAttribution,
    readSellerAttribution,
    () => null,
  );
  return {
    attribution,
    clear: clearSellerAttribution,
  };
}
