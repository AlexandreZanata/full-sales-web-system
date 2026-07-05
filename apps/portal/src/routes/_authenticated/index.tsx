import { createFileRoute, redirect } from '@tanstack/react-router';

import { CatalogPageContent } from '@/components/catalog/CatalogPageContent';
import { fetchPortalCategories } from '@/lib/api/portal';
import { parseCatalogSearch, resolveDefaultCategorySlug } from '@/lib/catalog/catalogSearch';
import { catalogCategoriesQueryKey } from '@/lib/catalog/useCatalogCategories';

export const Route = createFileRoute('/_authenticated/')({
  validateSearch: parseCatalogSearch,
  beforeLoad: async ({ search, context }) => {
    if (search.category) {
      return;
    }
    const categories = await context.queryClient.ensureQueryData({
      queryKey: catalogCategoriesQueryKey(),
      queryFn: fetchPortalCategories,
    });
    const defaultSlug = resolveDefaultCategorySlug(categories);
    if (defaultSlug) {
      // eslint-disable-next-line @typescript-eslint/only-throw-error
      throw redirect({ to: '/', search: { category: defaultSlug } });
    }
  },
  component: CatalogPage,
});

function CatalogPage() {
  const { category } = Route.useSearch();
  return <CatalogPageContent categoryParam={category} />;
}
