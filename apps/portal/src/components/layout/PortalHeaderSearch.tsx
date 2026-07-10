import { type SubmitEvent, useEffect, useState } from 'react';
import { useNavigate, useSearch } from '@tanstack/react-router';
import { Search, X } from 'lucide-react';

import { useI18n } from '@/lib/i18n/context';

type PortalHeaderSearchProps = {
  defaultCategorySlug?: string;
  activeCategorySlug?: string;
};

export function PortalHeaderSearch({
  defaultCategorySlug,
  activeCategorySlug,
}: PortalHeaderSearchProps) {
  const { t } = useI18n();
  const navigate = useNavigate();
  const { q } = useSearch({ strict: false });
  const [term, setTerm] = useState(q ?? '');

  useEffect(() => {
    setTerm(q ?? '');
  }, [q]);

  const targetCategory = activeCategorySlug ?? defaultCategorySlug;

  const submitSearch = (event: SubmitEvent) => {
    event.preventDefault();
    if (!targetCategory) {
      return;
    }
    const trimmed = term.trim();
    void navigate({
      to: '/',
      search: { category: targetCategory, q: trimmed || undefined },
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
