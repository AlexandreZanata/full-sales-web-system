import { describe, expect, it } from 'vitest';

import { categoryThumbCacheKey } from '@/components/categories/CategoryThumb';

describe('categoryThumbCacheKey', () => {
  it('combines imageFileId and thumbUrl for cache bust identity', () => {
    expect(categoryThumbCacheKey('file-a', '/v1/public/media/file-b/content')).toBe(
      'file-a|/v1/public/media/file-b/content',
    );
  });

  it('returns empty segments when image is missing', () => {
    expect(categoryThumbCacheKey()).toBe('|');
  });
});
