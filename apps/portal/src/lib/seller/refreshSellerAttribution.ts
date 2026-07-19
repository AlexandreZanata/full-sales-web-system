import { ApiError } from '@/lib/api/client';
import { fetchPublicSeller } from '@/lib/api/publicSellers';
import {
  clearSellerAttribution,
  readSellerAttribution,
  writeSellerAttribution,
  type SellerAttribution,
} from '@/lib/seller/attribution';

function sameAttribution(a: SellerAttribution, b: SellerAttribution): boolean {
  return (
    a.publicCode === b.publicCode &&
    a.displayName === b.displayName &&
    (a.contactPhone ?? '') === (b.contactPhone ?? '')
  );
}

/** Re-fetch public seller so banner/phone stay current after admin edits. */
export async function refreshSellerAttribution(publicCode: string): Promise<void> {
  try {
    const seller = await fetchPublicSeller(publicCode);
    const next: SellerAttribution = {
      publicCode: seller.publicCode,
      displayName: seller.displayName,
      contactPhone: seller.contactPhone,
    };
    const current = readSellerAttribution();
    if (current && sameAttribution(current, next)) {
      return;
    }
    writeSellerAttribution(next);
  } catch (error) {
    if (error instanceof ApiError && error.status === 404) {
      clearSellerAttribution();
      return;
    }
    throw error;
  }
}
