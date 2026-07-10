import { HeroBannerCarousel } from '@/components/catalog/home/HeroBannerCarousel';
import { HomeCategorySection } from '@/components/catalog/home/HomeCategorySection';

export function CatalogHomePage() {
  return (
    <div>
      <HeroBannerCarousel />
      <HomeCategorySection />
      <div id="offers" className="scroll-mt-24" aria-hidden />
    </div>
  );
}
