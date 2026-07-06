import { describe, expect, it } from 'vitest';

import {
  resolveMediaContentUrl,
  resolveCatalogImagePreviewUrl,
  resolveCategoryThumbUrl,
  resolveProductImagePreviewUrl,
} from '@/lib/api/uploads';

describe('resolveMediaContentUrl', () => {
  it('maps memory presigned URLs to authenticated content route', () => {
    expect(
      resolveMediaContentUrl(
        '01900001-0021-7000-8000-000000000001',
        'memory://dev-media/products/widget.webp?ttl=900',
      ),
    ).toBe('/v1/media/01900001-0021-7000-8000-000000000001/content');
  });

  it('passes through http presigned URLs unchanged', () => {
    const url = 'https://cdn.example.com/widget.webp?sig=abc';
    expect(resolveMediaContentUrl('01900001-0021-7000-8000-000000000001', url)).toBe(url);
  });
});

describe('resolveProductImagePreviewUrl', () => {
  it('uses public catalog media route for img tags', () => {
    expect(resolveProductImagePreviewUrl('01900001-0021-7000-8000-000000000001')).toBe(
      '/v1/public/media/01900001-0021-7000-8000-000000000001/content',
    );
  });
});

describe('resolveCatalogImagePreviewUrl', () => {
  it('uses public catalog media route for category img tags', () => {
    expect(resolveCatalogImagePreviewUrl('01900001-0016-7000-8000-000000000001')).toBe(
      '/v1/public/media/01900001-0016-7000-8000-000000000001/content',
    );
  });
});

describe('resolveCategoryThumbUrl', () => {
  it('prefers API thumbUrl over imageFileId', () => {
    expect(resolveCategoryThumbUrl('file-a', '/v1/public/media/file-b/content')).toBe(
      '/v1/public/media/file-b/content',
    );
  });

  it('falls back to public route from imageFileId', () => {
    expect(resolveCategoryThumbUrl('01900001-0016-7000-8000-000000000001')).toBe(
      '/v1/public/media/01900001-0016-7000-8000-000000000001/content',
    );
  });

  it('returns undefined when no image is linked', () => {
    expect(resolveCategoryThumbUrl()).toBeUndefined();
  });
});
