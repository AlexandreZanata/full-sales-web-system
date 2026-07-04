import { BrandMark } from '@/components/BrandMark';
import { useI18n } from '@/lib/i18n/context';
import { adminTokens } from '@/lib/admin-tokens';
import { BRAND_NAME } from '@/lib/brand';
import type { MessageKey } from '@/lib/i18n/messages';
import { useSiteSettings } from '@/lib/settings/useSiteSettings';
import { cn } from '@/lib/utils';

type SiteBrandProps = {
  className?: string;
  subtitleKey?: MessageKey;
  fallbackSubtitle?: string;
};

export function SiteBrand({ className, subtitleKey, fallbackSubtitle }: SiteBrandProps) {
  const { t } = useI18n();
  const settings = useSiteSettings();
  const displayName = settings.data?.displayName;
  const logoUrl = settings.data?.logoUrl;
  const subtitle = subtitleKey ? t(subtitleKey) : (fallbackSubtitle ?? '');

  return (
    <div className={cn(adminTokens.shellBrandBar, className)}>
      <div className="flex items-center gap-3">
        {logoUrl ? (
          <img
            src={logoUrl}
            alt=""
            className="size-10 shrink-0 rounded-md border border-hairline object-cover"
          />
        ) : (
          <BrandMark size="md" />
        )}
        <div className="min-w-0">
          <p className="text-xs font-semibold uppercase tracking-[0.2em] text-muted-foreground">
            {displayName ?? BRAND_NAME}
          </p>
          {subtitle ? (
            <p className="mt-1 truncate text-lg font-semibold leading-tight text-foreground">
              {subtitle}
            </p>
          ) : null}
        </div>
      </div>
    </div>
  );
}
