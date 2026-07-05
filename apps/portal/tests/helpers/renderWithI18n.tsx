import { render, type RenderOptions } from '@testing-library/react';
import { type ReactElement } from 'react';

import { I18nProvider } from '@/lib/i18n/context';

export function renderWithI18n(ui: ReactElement, options?: Omit<RenderOptions, 'wrapper'>) {
  return render(ui, {
    wrapper: ({ children }) => <I18nProvider>{children}</I18nProvider>,
    ...options,
  });
}
