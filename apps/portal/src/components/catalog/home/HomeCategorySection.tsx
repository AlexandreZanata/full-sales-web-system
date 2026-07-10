import { Link, useNavigate } from '@tanstack/react-router';

import { CategoryBar } from '@/components/catalog/CategoryBar';
import { resolveDefaultCategorySlug } from '@/lib/catalog/catalogSearch';
import { useCatalogCategories } from '@/lib/catalog/useCatalogCategories';
import { useI18n } from '@/lib/i18n/context';

function HomeCategorySkeleton() {
  return (
    <section data-testid="home-categories" aria-busy="true">
      <div className="mb-4 flex items-center justify-between">
        <div className="h-8 w-40 animate-pulse rounded-lg bg-surface-muted" />
        <div className="h-8 w-20 animate-pulse rounded-3xl bg-surface-muted" />
      </div>
      <div className="flex gap-4 overflow-hidden">
        {Array.from({ length: 4 }, (_, index) => (
          <div
            key={`home-category-skeleton-${String(index)}`}
            className="h-28 w-32 shrink-0 animate-pulse rounded-2xl bg-surface-muted"
          />
        ))}
      </div>
    </section>
  );
}

export function HomeCategorySection() {
  const { t } = useI18n();
  const navigate = useNavigate();
  const categoriesQuery = useCatalogCategories();
  const categories = categoriesQuery.data ?? [];
  const defaultSlug = resolveDefaultCategorySlug(categories);

  if (categoriesQuery.isLoading) {
    return <HomeCategorySkeleton />;
  }

  if (categories.length === 0) {
    return null;
  }

  return (
    <section data-testid="home-categories" className="mb-8">
      <div className="mb-4 flex items-center justify-between gap-4">
        <h2 className="text-2xl font-semibold capitalize text-foreground">
          {t('catalog.ourMenu')}
        </h2>
        {defaultSlug ? (
          <Link to="/" search={{ category: defaultSlug }} className="catalog-view-all-pill">
            {t('catalog.viewAll')}
          </Link>
        ) : null}
      </div>
      <CategoryBar
        variant="home"
        categories={categories}
        ariaLabel={t('catalog.categories')}
        onSelect={(slug) => {
          void navigate({ to: '/', search: { category: slug } });
        }}
      />
    </section>
  );
}
