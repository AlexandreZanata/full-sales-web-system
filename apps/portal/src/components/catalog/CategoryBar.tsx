import { useCallback, useMemo, useRef, type KeyboardEvent } from 'react';

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
  ariaLabel?: string;
};

type CategoryTab = {
  key: string;
  label: string;
  isActive: boolean;
  category?: PortalCategory;
  onClick: () => void;
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
  ariaLabel = 'Categories',
}: CategoryBarProps) {
  const tabRefs = useRef<Array<HTMLButtonElement | null>>([]);

  const tabs = useMemo<CategoryTab[]>(() => {
    const items: CategoryTab[] = [];
    if (allLabel && onSelectAll) {
      items.push({
        key: '__all__',
        label: allLabel,
        isActive: !activeSlug,
        onClick: onSelectAll,
      });
    }
    for (const category of categories) {
      items.push({
        key: category.id,
        label: category.name,
        isActive: category.slug === activeSlug,
        category,
        onClick: () => {
          onSelect(category.slug);
        },
      });
    }
    return items;
  }, [activeSlug, allLabel, categories, onSelect, onSelectAll]);

  const focusTab = useCallback((index: number) => {
    tabRefs.current[index]?.focus();
  }, []);

  const handleKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    const buttons = tabRefs.current.filter((tab): tab is HTMLButtonElement => tab !== null);
    if (buttons.length === 0) {
      return;
    }

    const currentIndex = buttons.findIndex((tab) => tab === document.activeElement);
    if (currentIndex === -1) {
      return;
    }

    if (event.key === 'ArrowRight') {
      event.preventDefault();
      focusTab((currentIndex + 1) % buttons.length);
      return;
    }

    if (event.key === 'ArrowLeft') {
      event.preventDefault();
      focusTab((currentIndex - 1 + buttons.length) % buttons.length);
    }
  };

  return (
    <div
      className={
        variant === 'home'
          ? 'catalog-category-row--home'
          : 'flex gap-2 overflow-x-auto pb-1 [-ms-overflow-style:none] [scrollbar-width:none] [&::-webkit-scrollbar]:hidden'
      }
      role="tablist"
      aria-label={ariaLabel}
      onKeyDown={handleKeyDown}
    >
      {tabs.map((tab, index) => (
        <button
          key={tab.key}
          ref={(element) => {
            tabRefs.current[index] = element;
          }}
          type="button"
          role="tab"
          tabIndex={tab.isActive ? 0 : -1}
          aria-selected={tab.isActive}
          aria-current={tab.isActive ? 'true' : undefined}
          className={cn(
            'catalog-category-chip',
            variant === 'home' ? 'catalog-category-chip--home' : 'catalog-category-chip--menu',
            tab.isActive && 'catalog-category-chip--active',
          )}
          onClick={tab.onClick}
        >
          {tab.category ? <CategoryThumb category={tab.category} variant={variant} /> : null}
          <span className="max-w-[5rem] truncate text-xs font-medium capitalize text-foreground">
            {tab.label}
          </span>
        </button>
      ))}
    </div>
  );
}
