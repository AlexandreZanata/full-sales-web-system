import { describe, expect, it } from 'vitest';

import { productCardDescription, stripHtml } from '@/lib/catalog/stripHtml';

describe('stripHtml', () => {
  it('given_html_when_stripped_then_returns_plain_text', () => {
    expect(stripHtml('<p>Hello <strong>world</strong></p>')).toBe('Hello world');
  });

  it('given_description_with_html_when_normalized_then_returns_plain', () => {
    expect(productCardDescription('<p>Fresh <em>cola</em></p>')).toBe('Fresh cola');
  });

  it('given_empty_description_when_normalized_then_returns_undefined', () => {
    expect(productCardDescription('   ')).toBeUndefined();
    expect(productCardDescription('<p></p>')).toBeUndefined();
  });
});
