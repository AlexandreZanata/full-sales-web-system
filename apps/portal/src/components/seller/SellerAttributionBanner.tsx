import { useSellerAttribution } from '@/lib/seller/useSellerAttribution';
import { useI18n } from '@/lib/i18n/context';

/** Subtle chip when the visit came from a seller share link. */
export function SellerAttributionBanner() {
  const { t } = useI18n();
  const { attribution } = useSellerAttribution();
  if (!attribution) {
    return null;
  }
  return (
    <div
      className="border-b border-border bg-muted/40 px-4 py-2 text-center text-sm text-muted-foreground"
      data-testid="seller-attribution-banner"
    >
      {t('sellerAttribution.assistedBy').replace('{name}', attribution.displayName)}
    </div>
  );
}
