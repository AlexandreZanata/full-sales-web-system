import type { PortalCategory } from '@/lib/api/types';
import { cn } from '@/lib/utils';

export type CategoryBarVariant = 'menu' | 'home';

type CategoryBarProps = {
  categories: PortalCategory[];
  activeSlug?: string;
  onSelect: (slug: string) => void;
  variant?: CategoryBarVariant;
  allLabel?: string;
  onSelectAll?: () => void;
};

function CategoryThumb({
  category,
  variant,
}: {
  category: PortalCategory;
  variant: CategoryBarVariant;
}) {
  if (category.thumbUrl) {
    return (
      <img
        src={category.thumbUrl}
        alt=""
        className={cn(
          'catalog-category-thumb object-cover',
          variant === 'home' && 'catalog-category-thumb--home',
        )}
      />
    );
  }

  const initial = category.name.trim().charAt(0).toUpperCase();

  return (
    <div
      className={cn('catalog-category-thumb', variant === 'home' && 'catalog-category-thumb--home')}
    >
      {initial}
    </div>
  );
}

export function CategoryBar({
  categories,
  activeSlug,
  onSelect,
  variant = 'menu',
  allLabel,
  onSelectAll,
}: CategoryBarProps) {
  return (
    <div
      className="flex gap-2 overflow-x-auto pb-1 [-ms-overflow-style:none] [scrollbar-width:none] [&::-webkit-scrollbar]:hidden"
      role="tablist"
      aria-label="Categories"
    >
      {allLabel && onSelectAll ? (
        <button
          type="button"
          role="tab"
          aria-selected={!activeSlug}
          aria-current={!activeSlug ? 'true' : undefined}
          className={cn(
            'catalog-category-chip',
            variant === 'home' ? 'catalog-category-chip--home' : 'catalog-category-chip--menu',
            !activeSlug && 'catalog-category-chip--active',
          )}
          onClick={onSelectAll}
        >
          <span className="text-xs font-medium text-foreground">{allLabel}</span>
        </button>
      ) : null}
      {categories.map((category) => {
        const isActive = category.slug === activeSlug;
        return (
          <button
            key={category.id}
            type="button"
            role="tab"
            aria-selected={isActive}
            aria-current={isActive ? 'true' : undefined}
            className={cn(
              'catalog-category-chip',
              variant === 'home' ? 'catalog-category-chip--home' : 'catalog-category-chip--menu',
              isActive && 'catalog-category-chip--active',
            )}
            onClick={() => {
              onSelect(category.slug);
            }}
          >
            <CategoryThumb category={category} variant={variant} />
            <span className="max-w-[5rem] truncate text-xs font-medium text-foreground">
              {category.name}
            </span>
          </button>
        );
      })}
    </div>
  );
}
