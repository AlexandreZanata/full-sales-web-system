import { FeaturedItemsSection } from '@/components/catalog/home/FeaturedItemsSection';
import { HeroBannerCarousel } from '@/components/catalog/home/HeroBannerCarousel';
import { HomeCategorySection } from '@/components/catalog/home/HomeCategorySection';
import { OfferBannersSection } from '@/components/catalog/home/OfferBannersSection';
import { PopularItemsSection } from '@/components/catalog/home/PopularItemsSection';

export function CatalogHomePage() {
  return (
    <div data-testid="catalog-home-page">
      <HeroBannerCarousel />
      <HomeCategorySection />
      <FeaturedItemsSection />
      <OfferBannersSection />
      <PopularItemsSection />
    </div>
  );
}
