import { createFileRoute } from '@tanstack/react-router';

import { CatalogHomePage } from '@/components/catalog/home/CatalogHomePage';
import { CatalogPageContent } from '@/components/catalog/CatalogPageContent';
import { parseCatalogSearch } from '@/lib/catalog/catalogSearch';

export const Route = createFileRoute('/_authenticated/')({
  validateSearch: parseCatalogSearch,
  component: CatalogPage,
});

function CatalogPage() {
  const { category, q } = Route.useSearch();

  if (!category) {
    return <CatalogHomePage />;
  }

  return <CatalogPageContent categoryParam={category} initialSearch={q} />;
}
