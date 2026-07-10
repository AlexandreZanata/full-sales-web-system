import { FeaturedItemsSection } from '@/components/catalog/home/FeaturedItemsSection';
import { HeroBannerCarousel } from '@/components/catalog/home/HeroBannerCarousel';
import { HomeCategorySection } from '@/components/catalog/home/HomeCategorySection';
import { OfferBannersSection } from '@/components/catalog/home/OfferBannersSection';
import { PopularItemsSection } from '@/components/catalog/home/PopularItemsSection';
import { useI18n } from '@/lib/i18n/context';

export function CatalogHomePage() {
  const { t } = useI18n();

  return (
    <div data-testid="catalog-home-page">
      <h1 className="sr-only">{t('nav.home')}</h1>
      <HeroBannerCarousel />
      <HomeCategorySection />
      <FeaturedItemsSection />
      <OfferBannersSection />
      <PopularItemsSection />
    </div>
  );
}
