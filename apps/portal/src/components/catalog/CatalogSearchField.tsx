import { Search } from 'lucide-react';
import { useId, type ChangeEventHandler } from 'react';

import { Input } from '@/components/ui/Input';
import { cn } from '@/lib/utils';

type CatalogSearchFieldProps = {
  value: string;
  onChange: ChangeEventHandler<HTMLInputElement>;
  label: string;
  placeholder: string;
};

export function CatalogSearchField({
  value,
  onChange,
  label,
  placeholder,
}: CatalogSearchFieldProps) {
  const inputId = useId();

  return (
    <>
      <div className="md:hidden">
        <Input
          label={label}
          placeholder={placeholder}
          value={value}
          onChange={onChange}
          className="sm:max-w-xs"
        />
      </div>
      <div className="relative hidden md:block md:w-full md:max-w-md">
        <label htmlFor={inputId} className="sr-only">
          {label}
        </label>
        <Search
          className="pointer-events-none absolute left-3.5 top-1/2 size-4 -translate-y-1/2 text-muted-foreground"
          aria-hidden
        />
        <input
          id={inputId}
          type="search"
          value={value}
          placeholder={placeholder}
          onChange={onChange}
          className={cn(
            'h-10 w-full rounded-full border border-hairline bg-background/80 pl-10 pr-4 text-sm text-foreground',
            'placeholder:text-muted-foreground/80',
            'shadow-sm transition-[border-color,box-shadow] focus:border-accent focus:outline-none focus:ring-2 focus:ring-accent/20',
          )}
        />
      </div>
    </>
  );
}
