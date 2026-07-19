import { useQuery } from '@tanstack/react-query';
import { useEffect, useId, useMemo, useState } from 'react';

import { fetchCommercesForPicker } from '@/lib/api/commerces';
import {
  filterCommercesBySearch,
  formatCommerceOption,
} from '@/lib/commerces/filterCommercesBySearch';
import { useDebouncedValue } from '@/lib/hooks/useDebouncedValue';
import { useI18n } from '@/lib/i18n/context';
import { cn } from '@/lib/utils';

const DEBOUNCE_MS = 300;
const MAX_RESULTS = 20;

type CommerceSearchPickerProps = {
  label: string;
  name?: string;
  value: string;
  error?: string;
  disabled?: boolean;
  onChange: (commerceId: string) => void;
};

export function CommerceSearchPicker({
  label,
  name,
  value,
  error,
  disabled,
  onChange,
}: CommerceSearchPickerProps) {
  const { t } = useI18n();
  const inputId = useId();
  const listId = useId();
  const [query, setQuery] = useState('');
  const [open, setOpen] = useState(false);
  const debouncedQuery = useDebouncedValue(query, DEBOUNCE_MS);

  const commerces = useQuery({
    queryKey: ['commerces', 'picker'],
    queryFn: fetchCommercesForPicker,
  });

  const selected = useMemo(
    () => commerces.data?.find((commerce) => commerce.id === value),
    [commerces.data, value],
  );

  useEffect(() => {
    if (!value) {
      setQuery('');
    }
  }, [value]);

  const matches = useMemo(() => {
    if (!open || !commerces.data) {
      return [];
    }
    return filterCommercesBySearch(commerces.data, debouncedQuery).slice(0, MAX_RESULTS);
  }, [commerces.data, debouncedQuery, open]);

  const showList = open && !disabled && debouncedQuery.trim().length > 0;
  const displayValue = open || !selected ? query : formatCommerceOption(selected);

  function selectCommerce(commerceId: string) {
    onChange(commerceId);
    setQuery('');
    setOpen(false);
  }

  function clearSelection() {
    onChange('');
    setQuery('');
    setOpen(false);
  }

  return (
    <div className="relative space-y-1.5">
      <label
        htmlFor={inputId}
        className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground"
      >
        {label}
      </label>
      <div className="flex gap-2">
        <input
          id={inputId}
          name={name}
          role="combobox"
          aria-expanded={showList}
          aria-controls={listId}
          aria-autocomplete="list"
          aria-invalid={Boolean(error)}
          autoComplete="off"
          disabled={disabled || commerces.isLoading}
          placeholder={t('forms.placeholders.searchCommerce')}
          value={displayValue}
          className={cn(
            'h-10 w-full rounded-md border bg-surface px-3 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-admin-accent',
            error ? 'border-destructive focus:ring-destructive' : 'border-input',
          )}
          onFocus={() => {
            setOpen(true);
            if (selected) {
              setQuery('');
            }
          }}
          onBlur={() => {
            window.setTimeout(() => {
              setOpen(false);
            }, 150);
          }}
          onChange={(event) => {
            setOpen(true);
            setQuery(event.target.value);
            if (value) {
              onChange('');
            }
          }}
        />
        {value ? (
          <button
            type="button"
            className="h-10 shrink-0 rounded-md border border-input bg-surface px-3 text-sm text-muted-foreground hover:text-foreground"
            disabled={disabled}
            onClick={clearSelection}
          >
            {t('forms.commerceSearch.clear')}
          </button>
        ) : null}
      </div>
      {showList ? (
        <ul
          id={listId}
          role="listbox"
          className="absolute z-20 mt-1 max-h-60 w-full overflow-auto rounded-md border border-input bg-surface shadow-md"
        >
          {matches.length === 0 ? (
            <li className="px-3 py-2 text-sm text-muted-foreground">
              {t('forms.commerceSearch.noMatches')}
            </li>
          ) : (
            matches.map((commerce) => (
              <li key={commerce.id} role="option" aria-selected={commerce.id === value}>
                <button
                  type="button"
                  className="w-full px-3 py-2 text-left text-sm hover:bg-muted"
                  onMouseDown={(event) => {
                    event.preventDefault();
                  }}
                  onClick={() => {
                    selectCommerce(commerce.id);
                  }}
                >
                  {formatCommerceOption(commerce)}
                </button>
              </li>
            ))
          )}
        </ul>
      ) : null}
      {error ? <p className="text-xs text-destructive">{error}</p> : null}
    </div>
  );
}
