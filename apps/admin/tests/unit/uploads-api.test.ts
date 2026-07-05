import { describe, expect, it } from 'vitest';

import { resolveMediaContentUrl, resolveProductImagePreviewUrl } from '@/lib/api/uploads';

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
