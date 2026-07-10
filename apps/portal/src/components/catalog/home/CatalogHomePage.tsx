import { FeaturedItemsSection } from '@/components/catalog/home/FeaturedItemsSection';
import { HeroBannerCarousel } from '@/components/catalog/home/HeroBannerCarousel';
import { HomeCategorySection } from '@/components/catalog/home/HomeCategorySection';
import { OfferBannersSection } from '@/components/catalog/home/OfferBannersSection';

export function CatalogHomePage() {
  return (
    <div>
      <HeroBannerCarousel />
      <HomeCategorySection />
      <FeaturedItemsSection />
      <OfferBannersSection />
    </div>
  );
}
