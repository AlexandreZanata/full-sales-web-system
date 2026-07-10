import { Autoplay, Pagination } from 'swiper/modules';
import { Swiper, SwiperSlide } from 'swiper/react';

import { useHeroBanners } from '@/lib/catalog/useHeroBanners';

import 'swiper/css';
import 'swiper/css/pagination';

function HeroBannerSkeleton() {
  return (
    <section data-testid="hero-banner" aria-busy="true" aria-label="Loading banner">
      <div className="mb-5 mt-5 aspect-[3/1] animate-pulse rounded-2xl bg-surface-muted sm:mt-8" />
    </section>
  );
}

export function HeroBannerCarousel() {
  const bannersQuery = useHeroBanners();

  if (bannersQuery.isLoading) {
    return <HeroBannerSkeleton />;
  }

  const banners = bannersQuery.data ?? [];
  if (banners.length === 0) {
    return null;
  }

  return (
    <section data-testid="hero-banner" className="mb-5 mt-5 sm:mt-8">
      <Swiper
        className="hero-banner-swiper rounded-2xl"
        modules={[Autoplay, Pagination]}
        slidesPerView={1}
        loop={banners.length > 1}
        speed={1000}
        autoplay={{ delay: 5000, disableOnInteraction: false }}
        pagination={{ clickable: true }}
      >
        {banners.map((banner) => {
          const image = (
            <img
              src={banner.imageUrl}
              alt={banner.altText ?? ''}
              className="aspect-[3/1] w-full rounded-2xl object-cover"
              loading="eager"
            />
          );

          return (
            <SwiperSlide key={banner.id}>
              {banner.linkUrl ? (
                <a href={banner.linkUrl} className="block">
                  {image}
                </a>
              ) : (
                image
              )}
            </SwiperSlide>
          );
        })}
      </Swiper>
    </section>
  );
}
