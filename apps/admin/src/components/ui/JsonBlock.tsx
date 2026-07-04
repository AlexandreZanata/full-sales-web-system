import { useState } from 'react';

import { useI18n } from '@/lib/i18n/context';
import { cn } from '@/lib/utils';

type JsonBlockProps = {
  value: unknown;
  className?: string;
  defaultOpen?: boolean;
};

function formatJson(value: unknown): string {
  if (value === undefined) {
    return 'null';
  }
  if (typeof value === 'string') {
    return value;
  }
  try {
    return JSON.stringify(value, null, 2);
  } catch {
    return '[unserializable value]';
  }
}

export function JsonBlock({ value, className, defaultOpen = false }: JsonBlockProps) {
  const { t } = useI18n();
  const [open, setOpen] = useState(defaultOpen);
  const formatted = formatJson(value);

  return (
    <div className={cn('rounded-md border border-hairline bg-surface-muted', className)}>
      <button
        type="button"
        className="flex w-full items-center justify-between px-3 py-2 text-left text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground"
        aria-expanded={open}
        onClick={() => {
          setOpen((current) => !current);
        }}
      >
        <span>{t('common.jsonPayload')}</span>
        <span aria-hidden>{open ? '−' : '+'}</span>
      </button>
      {open ? (
        <pre className="max-h-96 overflow-auto border-t border-hairline px-3 py-2 font-mono text-xs text-foreground">
          {formatted}
        </pre>
      ) : null}
    </div>
  );
}
