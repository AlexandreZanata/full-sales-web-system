import { HeroBannerCarousel } from '@/components/catalog/home/HeroBannerCarousel';

export function CatalogHomePage() {
  return (
    <div>
      <HeroBannerCarousel />
      <div id="offers" className="scroll-mt-24" aria-hidden />
    </div>
  );
}
