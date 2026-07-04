/**
 * Contract: docs/GLOSSARY.md ReportType enum — all types listed for i18n status.report.* keys.
 */
import { describe, expect, it } from 'vitest';

import { en } from '@/lib/i18n/locales/en';
import { translateReportType } from '@/lib/i18n/labels';
import { translate } from '@/lib/i18n/translate';
import { REPORT_TYPES } from '@/lib/reports/constants';

const t = (key: Parameters<typeof translate>[1]) => translate(en, key);

describe('REPORT_TYPES — Phase 35 contract', () => {
  it('lists_all_three_report_types_from_api_contract', () => {
    expect(REPORT_TYPES).toEqual(['DailyDriver', 'CommercePeriod', 'Consolidated']);
  });

  it('provides_i18n_labels_for_each_type', () => {
    for (const type of REPORT_TYPES) {
      expect(translateReportType(t, type).length).toBeGreaterThan(0);
    }
  });
});
