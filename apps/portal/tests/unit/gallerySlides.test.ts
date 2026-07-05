import { describe, expect, it } from 'vitest';

import { buildGallerySlides } from '@/lib/catalog/gallerySlides';

describe('buildGallerySlides — Phase 49 contract', () => {
  it('dedupes primary from imageUrls and keeps primary first', () => {
    const primary = 'https://cdn.example/primary.jpg';
    const slides = buildGallerySlides('Widget', primary, [primary, 'https://cdn.example/b.jpg']);

    expect(slides).toHaveLength(2);
    expect(slides[0]?.url).toBe(primary);
    expect(slides[1]?.url).toBe('https://cdn.example/b.jpg');
  });

  it('returns empty array when no images', () => {
    expect(buildGallerySlides('Widget')).toEqual([]);
  });
});
