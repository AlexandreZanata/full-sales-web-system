import { Link, useRouterState, useSearch } from '@tanstack/react-router';
import { Home, LayoutGrid, ShoppingCart, UserRound } from 'lucide-react';

import { usePortalAuth } from '@/auth/usePortalAuth';
import { useCart } from '@/cart/CartProvider';
import { catalogHomeSearch, resolveDefaultCategorySlug } from '@/lib/catalog/catalogSearch';
import { useCatalogCategories } from '@/lib/catalog/useCatalogCategories';
import { useI18n } from '@/lib/i18n/context';
import { cn } from '@/lib/utils';

type TabDef = {
  key: string;
  labelKey: 'nav.home' | 'nav.menu' | 'nav.cart' | 'nav.account';
  icon: typeof Home;
  to: '/' | '/cart' | '/orders' | '/login';
  search?: { category: string } | typeof catalogHomeSearch;
  isActive: boolean;
  badge?: number;
};

export function PortalMobileTabBar() {
  const { t } = useI18n();
  const { itemCount } = useCart();
  const { user } = usePortalAuth();
  const pathname = useRouterState({ select: (state) => state.location.pathname });
  const { category } = useSearch({ strict: false });
  const categoriesQuery = useCatalogCategories();
  const defaultCategorySlug = resolveDefaultCategorySlug(categoriesQuery.data ?? []);
  const accountTo = user ? '/orders' : '/login';

  const tabs: TabDef[] = [
    {
      key: 'home',
      labelKey: 'nav.home',
      icon: Home,
      to: '/',
      search: catalogHomeSearch,
      isActive: pathname === '/' && !category,
    },
    {
      key: 'menu',
      labelKey: 'nav.menu',
      icon: LayoutGrid,
      to: '/',
      search: defaultCategorySlug ? { category: defaultCategorySlug } : undefined,
      isActive: Boolean(category) || pathname.startsWith('/products/'),
    },
    {
      key: 'cart',
      labelKey: 'nav.cart',
      icon: ShoppingCart,
      to: '/cart',
      isActive: pathname === '/cart',
      badge: itemCount > 0 ? itemCount : undefined,
    },
    {
      key: 'account',
      labelKey: 'nav.account',
      icon: UserRound,
      to: accountTo,
      isActive: pathname.startsWith('/orders') || pathname === '/login',
    },
  ];

  return (
    <nav className="portal-mobile-tabbar" aria-label="Mobile">
      <div className="portal-mobile-tabbar__panel">
        {tabs.map(({ key, labelKey, icon: Icon, to, search, isActive, badge }) => (
          <Link
            key={key}
            to={to}
            search={search}
            className={cn('portal-mobile-tab', isActive && 'portal-mobile-tab--active')}
          >
            <span className="portal-mobile-tab__icon">
              <Icon className="size-5" strokeWidth={isActive ? 2.4 : 2} aria-hidden />
              {badge != null ? (
                <span className="portal-mobile-tab__badge">{badge > 99 ? '99+' : badge}</span>
              ) : null}
            </span>
            <span className="portal-mobile-tab__label">{t(labelKey)}</span>
          </Link>
        ))}
      </div>
    </nav>
  );
}
