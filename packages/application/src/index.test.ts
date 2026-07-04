import { describe, expect, it } from 'vitest';

import { ApplicationError } from './index.js';

// Contract: API-CONTRACT.md error format
describe('ApplicationError', () => {
  it('given_error_when_to_json_then_matches_contract_shape', () => {
    const err = new ApplicationError(
      'NOT_FOUND',
      'Resource not found',
      '550e8400-e29b-41d4-a716-446655440000',
    );

    expect(err.toJSON()).toEqual({
      error: {
        code: 'NOT_FOUND',
        message: 'Resource not found',
        correlationId: '550e8400-e29b-41d4-a716-446655440000',
      },
    });
  });
});
