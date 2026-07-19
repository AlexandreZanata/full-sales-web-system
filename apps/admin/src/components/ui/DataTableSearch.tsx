import { useId } from 'react';

import { useI18n } from '@/lib/i18n/context';

type DataTableSearchProps = {
  value: string;
  placeholder?: string;
  onChange: (value: string) => void;
};

export function DataTableSearch({ value, placeholder, onChange }: DataTableSearchProps) {
  const { t } = useI18n();
  const inputId = useId();

  return (
    <div className="border-b border-hairline bg-surface px-4 py-3">
      <label htmlFor={inputId} className="sr-only">
        {t('common.search')}
      </label>
      <input
        id={inputId}
        type="search"
        autoComplete="off"
        value={value}
        placeholder={placeholder ?? t('common.table.searchPlaceholder')}
        className="h-10 w-full max-w-md rounded-md border border-input bg-surface px-3 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-admin-accent"
        onChange={(event) => {
          onChange(event.target.value);
        }}
      />
    </div>
  );
}
