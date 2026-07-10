import { Link, useRouterState, useSearch } from '@tanstack/react-router';
import { LogIn, ShoppingBag } from 'lucide-react';

import { usePortalAuth } from '@/auth/usePortalAuth';
import { useCart } from '@/cart/CartProvider';
import { LocaleSwitcher } from '@/components/LocaleSwitcher';
import { PortalAccountMenu } from '@/components/layout/PortalAccountMenu';
import { PortalHeaderSearch } from '@/components/layout/PortalHeaderSearch';
import { resolveDefaultCategorySlug, catalogHomeSearch } from '@/lib/catalog/catalogSearch';
import {
  isPortalHomeActive,
  isPortalMenuActive,
  isPortalOffersActive,
} from '@/lib/catalog/portalHeaderNav';
import { useCatalogCategories } from '@/lib/catalog/useCatalogCategories';
import { useI18n } from '@/lib/i18n/context';
import { formatMoney } from '@/lib/products/formatPrice';
import { useSiteSettings } from '@/lib/settings/useSiteSettings';
import { cn } from '@/lib/utils';

export function PortalHeader() {
  const { t } = useI18n();
  const { logout, user } = usePortalAuth();
  const { totalAmount, currency } = useCart();
  const pathname = useRouterState({ select: (state) => state.location.pathname });
  const hash = useRouterState({ select: (state) => state.location.hash });
  const { category } = useSearch({ strict: false });
  const settings = useSiteSettings(Boolean(user));
  const categoriesQuery = useCatalogCategories();
  const defaultCategorySlug = resolveDefaultCategorySlug(categoriesQuery.data ?? []);
  const brandName = settings.data?.displayName ?? t('auth.portalLabel');
  const logoUrl = settings.data?.logoUrl;
  const cartLabel = formatMoney(totalAmount, currency);

  const navLinkClass = (active: boolean) =>
    cn('portal-nav-link', active && 'portal-nav-link--active');

  return (
    <header className="portal-header">
      <div className="mx-auto flex max-w-6xl flex-col gap-3 px-4 py-3 lg:flex-row lg:items-center lg:justify-between lg:py-0">
        <div className="flex items-center justify-between gap-3 lg:justify-start">
          <Link to="/" search={catalogHomeSearch} className="flex min-w-0 items-center gap-2">
            {logoUrl ? (
              <img src={logoUrl} alt={brandName} className="h-8 w-auto max-w-32 object-contain" />
            ) : (
              <span className="truncate text-lg font-semibold text-foreground">{brandName}</span>
            )}
          </Link>
          <div className="flex items-center gap-2 lg:hidden">
            <Link to="/cart" className="portal-cart-pill" aria-label={t('nav.cart')}>
              <ShoppingBag className="size-4" aria-hidden />
              <span className="tabular-nums">{cartLabel}</span>
            </Link>
            {user ? (
              <PortalAccountMenu email={user.email} onLogout={() => void logout()} />
            ) : (
              <Link to="/login" className="portal-login-pill">
                <LogIn className="size-4" aria-hidden />
                <span>{t('auth.signIn')}</span>
              </Link>
            )}
          </div>
        </div>

        <nav className="hidden items-center justify-center gap-6 lg:flex" aria-label="Main">
          <Link
            to="/"
            search={catalogHomeSearch}
            className={navLinkClass(isPortalHomeActive(pathname, category))}
          >
            {t('nav.home')}
          </Link>
          <Link
            to="/"
            search={defaultCategorySlug ? { category: defaultCategorySlug } : undefined}
            className={navLinkClass(isPortalMenuActive(pathname, category))}
          >
            {t('nav.menu')}
          </Link>
          <Link
            to="/"
            hash="offers"
            search={catalogHomeSearch}
            className={navLinkClass(isPortalOffersActive(hash))}
          >
            {t('nav.offers')}
          </Link>
        </nav>

        <div className="hidden items-center justify-end gap-2 lg:flex">
          <PortalHeaderSearch defaultCategorySlug={defaultCategorySlug} />
          <LocaleSwitcher variant="pill" />
          <Link to="/cart" className="portal-cart-pill">
            <ShoppingBag className="size-4" aria-hidden />
            <span className="tabular-nums">{cartLabel}</span>
          </Link>
          {user ? (
            <PortalAccountMenu email={user.email} onLogout={() => void logout()} />
          ) : (
            <Link to="/login" className="portal-login-pill">
              <LogIn className="size-4" aria-hidden />
              <span>{t('auth.signIn')}</span>
            </Link>
          )}
        </div>
      </div>
    </header>
  );
}
