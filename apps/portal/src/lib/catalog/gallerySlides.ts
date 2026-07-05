export type GallerySlide = {
  url: string;
  alt: string;
};

export function buildGallerySlides(
  productName: string,
  primaryUrl?: string,
  imageUrls: string[] = [],
): GallerySlide[] {
  const slides: GallerySlide[] = [];
  const seen = new Set<string>();

  if (primaryUrl) {
    slides.push({ url: primaryUrl, alt: productName });
    seen.add(primaryUrl);
  }

  for (const url of imageUrls) {
    if (!url || seen.has(url)) {
      continue;
    }
    slides.push({ url, alt: productName });
    seen.add(url);
  }

  return slides;
}
