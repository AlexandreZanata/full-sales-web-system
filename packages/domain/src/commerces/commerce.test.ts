import { describe, expect, it } from 'vitest';

import { Commerce } from './commerce.js';
import { Cnpj } from '../value-objects/cnpj.js';
import { parseCommerceId, parseTenantId } from '../value-objects/ids.js';

const tenantId = parseTenantId('550e8400-e29b-41d4-a716-446655440000');
const commerceId = parseCommerceId('550e8400-e29b-41d4-a716-446655440002');

describe('Commerce', () => {
  it('given_valid_input_when_create_then_active_with_commerce_created_event', () => {
    const commerce = Commerce.create({
      id: commerceId,
      cnpj: Cnpj.parse('11444777000161'),
      legalName: 'Acme Comercio Ltda',
      tradeName: 'Acme Store',
      tenantId,
    });
    expect(commerce.isActive()).toBe(true);
    const events = commerce.pullDomainEvents();
    expect(events).toHaveLength(1);
    expect(events[0]?.type).toBe('CommerceCreated');
  });

  it('given_active_commerce_when_deactivate_then_inactive', () => {
    const commerce = Commerce.create({
      id: commerceId,
      cnpj: Cnpj.parse('11444777000161'),
      legalName: 'Acme Comercio Ltda',
      tenantId,
    });
    expect(commerce.deactivate().isActive()).toBe(false);
  });
});
