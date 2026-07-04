import { Link, createFileRoute } from '@tanstack/react-router';
import { ClipboardList, SlidersHorizontal } from 'lucide-react';
import { useMemo } from 'react';

import { Card } from '@/components/ui/Card';
import { PageHeader } from '@/components/ui/PageHeader';
import { useI18n } from '@/lib/i18n/context';

export const Route = createFileRoute('/_authenticated/inventory/')({
  component: InventoryHubPage,
});

function InventoryHubPage() {
  const { t } = useI18n();

  const links = useMemo(
    () =>
      [
        {
          to: '/inventory/adjustments',
          label: t('inventory.hub.adjustments.title'),
          description: t('inventory.hub.adjustments.description'),
          icon: SlidersHorizontal,
        },
        {
          to: '/inventory/ledger',
          label: t('inventory.hub.ledger.title'),
          description: t('inventory.hub.ledger.description'),
          icon: ClipboardList,
        },
      ] as const,
    [t],
  );

  return (
    <div>
      <PageHeader title={t('inventory.hub.title')} description={t('inventory.hub.description')} />

      <div className="grid gap-4 sm:grid-cols-2">
        {links.map((item) => (
          <Link key={item.to} to={item.to} className="block">
            <Card className="h-full transition hover:border-foreground/20">
              <div className="flex items-start gap-4">
                <item.icon className="mt-1 size-5 shrink-0 text-muted-foreground" />
                <div>
                  <p className="font-medium text-foreground">{item.label}</p>
                  <p className="mt-1 text-sm text-muted-foreground">{item.description}</p>
                </div>
              </div>
            </Card>
          </Link>
        ))}
      </div>
    </div>
  );
}
