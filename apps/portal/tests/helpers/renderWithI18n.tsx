import { render, type RenderOptions } from '@testing-library/react';
import { type ReactElement } from 'react';

import { I18nProvider } from '@/lib/i18n/context';
import type { Locale } from '@/lib/i18n/types';

type RenderWithI18nOptions = Omit<RenderOptions, 'wrapper'> & {
  locale?: Locale;
};

export function renderWithI18n(ui: ReactElement, options?: RenderWithI18nOptions) {
  const { locale = 'pt-BR', ...renderOptions } = options ?? {};
  return render(ui, {
    wrapper: ({ children }) => <I18nProvider initialLocale={locale}>{children}</I18nProvider>,
    ...renderOptions,
  });
}
