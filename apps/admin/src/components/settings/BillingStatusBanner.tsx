import { Link } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { AlertTriangle } from 'lucide-react';

import { Button } from '@/components/ui/Button';
import { fetchSubscription } from '@/lib/api/billing';
import { shouldShowBillingBanner } from '@/lib/billing/helpers';
import { useI18n } from '@/lib/i18n/context';

export function BillingStatusBanner() {
  const { t } = useI18n();
  const subscription = useQuery({
    queryKey: ['billing', 'subscription'],
    queryFn: fetchSubscription,
    retry: false,
  });

  if (!subscription.data || !shouldShowBillingBanner(subscription.data)) {
    return null;
  }

  const suspended = subscription.data.tenantStatus === 'Suspended';
  const message = suspended ? t('settings.banner.suspended') : t('settings.banner.pastDue');

  return (
    <div className="mb-4 flex flex-wrap items-center gap-3 rounded-md border border-status-warning/40 bg-status-warning/10 px-4 py-3 text-sm">
      <AlertTriangle className="size-4 shrink-0 text-status-warning" aria-hidden />
      <p className="flex-1">{message}</p>
      <Link to="/settings/billing">
        <Button variant="secondary" className="min-h-9">
          {t('settings.banner.updatePayment')}
        </Button>
      </Link>
    </div>
  );
}
