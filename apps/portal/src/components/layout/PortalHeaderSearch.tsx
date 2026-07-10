import { Search, X } from 'lucide-react';
import { type SubmitEvent, useState } from 'react';
import { useNavigate } from '@tanstack/react-router';

import { useI18n } from '@/lib/i18n/context';

type PortalHeaderSearchProps = {
  defaultCategorySlug?: string;
};

export function PortalHeaderSearch({ defaultCategorySlug }: PortalHeaderSearchProps) {
  const { t } = useI18n();
  const navigate = useNavigate();
  const [term, setTerm] = useState('');

  const submitSearch = (event: SubmitEvent) => {
    event.preventDefault();
    if (!defaultCategorySlug) {
      return;
    }
    const trimmed = term.trim();
    void navigate({
      to: '/',
      search: { category: defaultCategorySlug, q: trimmed || undefined },
    });
  };

  return (
    <form onSubmit={submitSearch} className="portal-header-search group hidden lg:flex">
      <button
        type="submit"
        className="shrink-0 text-muted-foreground"
        aria-label={t('common.search')}
      >
        <Search className="size-4" aria-hidden />
      </button>
      <input
        type="search"
        value={term}
        placeholder={t('catalog.searchPlaceholder')}
        aria-label={t('common.search')}
        className="portal-header-search-field"
        onChange={(event) => {
          setTerm(event.target.value);
        }}
      />
      {term ? (
        <button
          type="button"
          className="shrink-0 text-muted-foreground hover:text-foreground"
          aria-label={t('common.clearSearch')}
          onClick={() => {
            setTerm('');
          }}
        >
          <X className="size-4" aria-hidden />
        </button>
      ) : null}
    </form>
  );
}
