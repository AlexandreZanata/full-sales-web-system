import { Link } from '@tanstack/react-router';
import { Mail, Phone } from 'lucide-react';
import { type SubmitEvent, useState } from 'react';

import { resolveDefaultCategorySlug } from '@/lib/catalog/catalogSearch';
import { useCatalogCategories } from '@/lib/catalog/useCatalogCategories';
import { useI18n } from '@/lib/i18n/context';
import { useSiteSettings } from '@/lib/settings/useSiteSettings';

export function PortalFooter() {
  const { t } = useI18n();
  const settings = useSiteSettings(true);
  const categoriesQuery = useCatalogCategories();
  const defaultCategorySlug = resolveDefaultCategorySlug(categoriesQuery.data ?? []);
  const brandName = settings.data?.displayName ?? t('auth.portalLabel');
  const logoUrl = settings.data?.logoUrl;
  const phone = settings.data?.salesContactPhone;
  const [email, setEmail] = useState('');
  const year = new Date().getFullYear();

  const handleNewsletterSubmit = (event: SubmitEvent) => {
    event.preventDefault();
    // ponytail: stub until portal newsletter endpoint exists (Phase 71L / settings API).
    setEmail('');
  };

  return (
    <footer className="portal-footer mt-auto" role="contentinfo">
      <div className="mx-auto grid max-w-6xl gap-6 px-4 py-8 sm:grid-cols-2 md:grid-cols-3 md:gap-8">
        <div>
          <Link to="/" className="mb-3 inline-block">
            {logoUrl ? (
              <img src={logoUrl} alt={brandName} className="h-8 w-auto max-w-28 object-contain" />
            ) : (
              <span className="text-lg font-semibold text-primary-foreground">{brandName}</span>
            )}
          </Link>
          <p className="mb-3 text-xs text-primary-foreground/90">{t('footer.newsletterHint')}</p>
          <form
            onSubmit={handleNewsletterSubmit}
            className="mb-2 flex h-9 max-w-xs items-center rounded-lg bg-surface p-1.5"
          >
            <input
              type="email"
              required
              value={email}
              placeholder={t('footer.emailPlaceholder')}
              aria-label={t('footer.emailPlaceholder')}
              className="h-full w-full bg-transparent px-2 text-xs text-foreground outline-none"
              onChange={(event) => {
                setEmail(event.target.value);
              }}
            />
            <button type="submit" className="portal-footer-subscribe-btn">
              {t('footer.subscribe')}
            </button>
          </form>
        </div>

        <div className="sm:mx-auto sm:w-fit">
          <h3 className="mb-3 text-sm font-semibold capitalize">{t('footer.usefulLinks')}</h3>
          <nav className="flex flex-col items-start gap-2 text-xs">
            <Link to="/" className="hover:underline">
              {t('nav.home')}
            </Link>
            <Link
              to="/"
              search={defaultCategorySlug ? { category: defaultCategorySlug } : undefined}
              className="hover:underline"
            >
              {t('nav.menu')}
            </Link>
            <Link to="/" hash="offers" className="hover:underline">
              {t('nav.offers')}
            </Link>
          </nav>
        </div>

        <div>
          <h3 className="mb-3 text-sm font-semibold capitalize">{t('footer.contact')}</h3>
          <ul className="flex flex-col gap-2.5 text-sm">
            <li className="flex items-center gap-2">
              <Mail className="size-4 shrink-0" aria-hidden />
              <span>{brandName}</span>
            </li>
            {phone ? (
              <li className="flex items-center gap-2">
                <Phone className="size-4 shrink-0" aria-hidden />
                <a href={`tel:${phone}`} className="font-medium hover:underline">
                  {phone}
                </a>
              </li>
            ) : null}
          </ul>
        </div>
      </div>
      <div className="border-t border-primary-foreground/20 py-3">
        <p className="text-center text-xs text-primary-foreground/90">
          {t('footer.copyright').replace('{year}', String(year)).replace('{brand}', brandName)}
        </p>
      </div>
    </footer>
  );
}
