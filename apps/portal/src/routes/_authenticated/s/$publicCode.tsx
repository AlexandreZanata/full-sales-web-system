import { createFileRoute, redirect } from '@tanstack/react-router';

import { ApiError } from '@/lib/api/client';
import { fetchPublicSeller } from '@/lib/api/publicSellers';
import { clearSellerAttribution, writeSellerAttribution } from '@/lib/seller/attribution';

type ShareSearch = {
  product?: string;
  category?: string;
};

function parseShareSearch(search: Record<string, unknown>): ShareSearch {
  return {
    product: typeof search.product === 'string' ? search.product : undefined,
    category: typeof search.category === 'string' ? search.category : undefined,
  };
}

export const Route = createFileRoute('/_authenticated/s/$publicCode')({
  validateSearch: parseShareSearch,
  beforeLoad: async ({ params, search }) => {
    try {
      const seller = await fetchPublicSeller(params.publicCode);
      writeSellerAttribution({
        publicCode: seller.publicCode,
        displayName: seller.displayName,
        contactPhone: seller.contactPhone,
      });
    } catch (error) {
      clearSellerAttribution();
      if (!(error instanceof ApiError && error.status === 404)) {
        throw error;
      }
    }

    if (search.product) {
      // eslint-disable-next-line @typescript-eslint/only-throw-error -- TanStack Router redirect
      throw redirect({
        to: '/products/$id',
        params: { id: search.product },
        search: search.category ? { category: search.category } : undefined,
      });
    }

    // eslint-disable-next-line @typescript-eslint/only-throw-error -- TanStack Router redirect
    throw redirect({
      to: '/',
      search: search.category ? { category: search.category } : undefined,
    });
  },
});
