import { Link, useRouterState, useSearch } from '@tanstack/react-router';
import { Home, LayoutGrid, ShoppingCart } from 'lucide-react';
import { type ReactNode } from 'react';

import { useCart } from '@/cart/CartProvider';
import { CartFab } from '@/components/CartFab';
import { PortalFooter } from '@/components/layout/PortalFooter';
import { PortalHeader } from '@/components/layout/PortalHeader';
import { resolveDefaultCategorySlug } from '@/lib/catalog/catalogSearch';
import { useCatalogCategories } from '@/lib/catalog/useCatalogCategories';
import { useI18n } from '@/lib/i18n/context';
import { cn } from '@/lib/utils';

type PortalShellProps = {
  children: ReactNode;
};

type MobileNavItem = {
  to: '/' | '/cart';
  labelKey: 'nav.home' | 'nav.menu' | 'nav.cart';
  icon: typeof Home;
  search?: { category: string };
  isActive: boolean;
};

export function PortalShell({ children }: PortalShellProps) {
  const { t } = useI18n();
  const { itemCount } = useCart();
  const pathname = useRouterState({ select: (state) => state.location.pathname });
  const { category } = useSearch({ strict: false });
  const categoriesQuery = useCatalogCategories();
  const defaultCategorySlug = resolveDefaultCategorySlug(categoriesQuery.data ?? []);

  const mobileNavItems: MobileNavItem[] = [
    {
      to: '/',
      labelKey: 'nav.home',
      icon: Home,
      isActive: pathname === '/' && !category,
    },
    {
      to: '/',
      labelKey: 'nav.menu',
      icon: LayoutGrid,
      search: defaultCategorySlug ? { category: defaultCategorySlug } : undefined,
      isActive: Boolean(category) || pathname.startsWith('/products/'),
    },
    {
      to: '/cart',
      labelKey: 'nav.cart',
      icon: ShoppingCart,
      isActive: pathname === '/cart',
    },
  ];

  return (
    <div className="flex min-h-dvh flex-col bg-background">
      <PortalHeader />

      <main className="mx-auto w-full max-w-6xl flex-1 px-4 py-4 pb-24 md:pb-6">{children}</main>

      <PortalFooter />

      <nav
        className="fixed inset-x-0 bottom-0 z-20 border-t border-hairline bg-surface md:hidden"
        aria-label="Mobile"
      >
        <div className="mx-auto grid max-w-6xl grid-cols-3">
          {mobileNavItems.map(({ to, labelKey, icon: Icon, search, isActive }) => (
            <Link
              key={labelKey}
              to={to}
              search={search}
              className={cn(
                'flex flex-col items-center gap-1 py-2 text-xs font-medium',
                isActive ? 'text-primary' : 'text-muted-foreground',
              )}
            >
              <span className="relative">
                <Icon className="size-5" aria-hidden />
                {to === '/cart' && itemCount > 0 ? (
                  <span className="absolute -right-2 -top-1 rounded-full bg-primary px-1 text-[10px] text-primary-foreground">
                    {itemCount}
                  </span>
                ) : null}
              </span>
              {t(labelKey)}
            </Link>
          ))}
        </div>
      </nav>

      <CartFab />
    </div>
  );
}
