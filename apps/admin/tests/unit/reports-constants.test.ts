/**
 * Contract: docs/GLOSSARY.md ReportType enum labels.
 */
import { describe, expect, it } from 'vitest';

import { REPORT_TYPE_LABELS, REPORT_TYPES } from '@/lib/reports/constants';

describe('REPORT_TYPES — Phase 35 contract', () => {
  it('lists_all_three_report_types_from_api_contract', () => {
    expect(REPORT_TYPES).toEqual(['DailyDriver', 'CommercePeriod', 'Consolidated']);
  });

  it('provides_human_labels_for_each_type', () => {
    for (const type of REPORT_TYPES) {
      expect(REPORT_TYPE_LABELS[type].length).toBeGreaterThan(0);
    }
  });
});
