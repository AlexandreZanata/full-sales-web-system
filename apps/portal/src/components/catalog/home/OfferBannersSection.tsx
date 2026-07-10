import { Link } from '@tanstack/react-router';

import type { PortalPromotion } from '@/lib/api/portalPromotions';
import { usePromotions } from '@/lib/catalog/usePromotions';
import { useI18n } from '@/lib/i18n/context';

function OfferBannersSkeleton() {
  return (
    <section data-testid="offer-banners" aria-busy="true" className="mb-8">
      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 md:gap-6">
        {Array.from({ length: 2 }, (_, index) => (
          <div
            key={`offer-skeleton-${String(index)}`}
            className="min-h-40 animate-pulse rounded-2xl bg-surface-muted"
          />
        ))}
      </div>
    </section>
  );
}

function OfferBannerCard({
  promotion,
  orderNowLabel,
}: {
  promotion: PortalPromotion;
  orderNowLabel: string;
}) {
  const toneClass =
    promotion.background === 'green' ? 'catalog-offer-card--green' : 'catalog-offer-card--yellow';
  const headlineClass = promotion.background === 'green' ? 'text-emerald-700' : 'text-primary';

  const cta = <span className="catalog-offer-cta">{orderNowLabel}</span>;

  const content = (
    <article className={`catalog-offer-card ${toneClass}`}>
      <div className="catalog-offer-card__content">
        <h3 className={`text-xl font-semibold capitalize sm:text-2xl ${headlineClass}`}>
          {promotion.headline}
        </h3>
        <p className="mt-1 text-sm font-medium text-foreground sm:text-base">
          {promotion.discountText}
        </p>
        {cta}
      </div>
      {promotion.imageUrl ? (
        <img src={promotion.imageUrl} alt="" className="catalog-offer-card__image" loading="lazy" />
      ) : null}
    </article>
  );

  if (promotion.categorySlug) {
    return (
      <Link
        to="/"
        search={{ category: promotion.categorySlug }}
        className="block rounded-2xl focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary"
      >
        {content}
      </Link>
    );
  }

  if (promotion.linkUrl) {
    return (
      <a
        href={promotion.linkUrl}
        className="block rounded-2xl focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary"
      >
        {content}
      </a>
    );
  }

  return content;
}

export function OfferBannersSection() {
  const { t } = useI18n();
  const promotionsQuery = usePromotions(4);
  const promotions = promotionsQuery.data ?? [];

  if (promotionsQuery.isLoading) {
    return <OfferBannersSkeleton />;
  }

  if (promotions.length === 0) {
    return null;
  }

  return (
    <section id="offers" data-testid="offer-banners" className="mb-8 scroll-mt-24">
      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 md:gap-6">
        {promotions.map((promotion) => (
          <OfferBannerCard
            key={promotion.id}
            promotion={promotion}
            orderNowLabel={t('catalog.orderNow')}
          />
        ))}
      </div>
    </section>
  );
}
