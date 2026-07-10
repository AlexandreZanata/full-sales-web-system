import { ChevronDown, LogOut, Package, User } from 'lucide-react';
import { useEffect, useRef, useState } from 'react';
import { Link } from '@tanstack/react-router';

import { useI18n } from '@/lib/i18n/context';

type PortalAccountMenuProps = {
  email: string;
  onLogout: () => void;
};

export function PortalAccountMenu({ email, onLogout }: PortalAccountMenuProps) {
  const { t } = useI18n();
  const [open, setOpen] = useState(false);
  const rootRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!open) {
      return;
    }
    const handlePointerDown = (event: MouseEvent) => {
      if (!rootRef.current?.contains(event.target as Node)) {
        setOpen(false);
      }
    };
    document.addEventListener('mousedown', handlePointerDown);
    return () => {
      document.removeEventListener('mousedown', handlePointerDown);
    };
  }, [open]);

  return (
    <div ref={rootRef} className="relative">
      <button
        type="button"
        className="portal-login-pill"
        aria-expanded={open}
        aria-haspopup="menu"
        onClick={() => {
          setOpen((current) => !current);
        }}
      >
        <User className="size-5" aria-hidden />
        <span className="hidden sm:inline">{t('nav.account')}</span>
        <ChevronDown className="size-3.5" aria-hidden />
      </button>
      {open ? (
        <div
          role="menu"
          className="absolute right-0 top-full z-30 mt-2 w-56 rounded-xl border border-hairline bg-surface p-2 shadow-lg"
        >
          <p className="truncate px-2 py-1.5 text-xs text-muted-foreground">{email}</p>
          <Link
            to="/orders"
            role="menuitem"
            className="flex items-center gap-2 rounded-lg px-2 py-2 text-sm text-foreground hover:bg-surface-muted"
            onClick={() => {
              setOpen(false);
            }}
          >
            <Package className="size-4" aria-hidden />
            {t('nav.orders')}
          </Link>
          <button
            type="button"
            role="menuitem"
            className="flex w-full items-center gap-2 rounded-lg px-2 py-2 text-sm text-foreground hover:bg-surface-muted"
            onClick={() => {
              setOpen(false);
              onLogout();
            }}
          >
            <LogOut className="size-4" aria-hidden />
            {t('auth.logout')}
          </button>
        </div>
      ) : null}
    </div>
  );
}
