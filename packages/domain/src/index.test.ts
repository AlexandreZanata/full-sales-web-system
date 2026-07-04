import { describe, expect, it } from 'vitest';

import { DOMAIN_SCAFFOLD_VERSION } from './index.js';

describe('domain scaffold', () => {
  it('given_package_when_loaded_then_has_version', () => {
    expect(DOMAIN_SCAFFOLD_VERSION).toBe('0.1.0');
  });
});
