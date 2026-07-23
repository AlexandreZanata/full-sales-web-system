import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { Input } from '@/components/ui/Input';
import { JsonBlock } from '@/components/ui/JsonBlock';
import type { CursorListResponse } from '@/lib/cursorPagination';
import type {
  AuditEvent,
  PlatformDomain,
  PlatformUser,
  TenantDetail,
  TenantStats,
} from '@/lib/api/types';
import { formatDateTime } from '@/lib/formatDateTime';
import { formatMoneyMinor } from '@/lib/i18n/labels';
import { useI18n } from '@/lib/i18n/context';

type TenantDetailPanelsProps = {
  tab: string;
  tenantId: string;
  tenant: TenantDetail;
  planId: string;
  onPlanIdChange: (value: string) => void;
  onChangePlan: () => void;
  stats?: TenantStats;
  workforce?: CursorListResponse<PlatformUser>;
  domains?: CursorListResponse<PlatformDomain>;
  audit?: CursorListResponse<AuditEvent>;
};

export function TenantDetailPanels({
  tab,
  tenantId,
  tenant,
  planId,
  onPlanIdChange,
  onChangePlan,
  stats,
  workforce,
  domains,
  audit,
}: TenantDetailPanelsProps) {
  const { t } = useI18n();
  const userColumns: DataTableColumn<PlatformUser>[] = [
    { id: 'name', header: t('common.name'), cell: (row) => row.name },
    { id: 'email', header: t('common.email'), cell: (row) => row.email },
    { id: 'role', header: 'Role', cell: (row) => row.role },
  ];

  if (tab === 'overview') {
    return (
      <Card className="grid gap-2 p-4 text-sm md:grid-cols-2">
        <p>
          <strong>{t('tenants.legalName')}:</strong> {tenant.legalName}
        </p>
        <p>
          <strong>{t('tenants.displayName')}:</strong> {tenant.displayName}
        </p>
        <p>
          <strong>{t('common.status')}:</strong> {tenant.status}
        </p>
        <p>
          <strong>{t('tenants.planId')}:</strong> {tenant.planId ?? '—'}
        </p>
        <p>
          <strong>Users:</strong> {tenant.counts.users}
        </p>
        <p>
          <strong>Orders:</strong> {tenant.counts.orders}
        </p>
      </Card>
    );
  }

  if (tab === 'billing' && stats) {
    return (
      <Card className="space-y-3 p-4 text-sm">
        <p>MRR: {formatMoneyMinor(stats.mrrMinor, stats.mrrCurrency)}</p>
        <p>Orders: {stats.orders}</p>
        <div className="flex flex-wrap items-end gap-2">
          <Input
            label={t('tenants.changePlan')}
            value={planId || tenant.planId || ''}
            onChange={(e) => {
              onPlanIdChange(e.target.value);
            }}
          />
          <Button onClick={onChangePlan} disabled={!planId.trim()}>
            {t('common.save')}
          </Button>
        </div>
      </Card>
    );
  }

  if (tab === 'settings') {
    return <JsonBlock value={tenant.settings} />;
  }

  if (tab === 'users' && workforce) {
    return <DataTable columns={userColumns} rows={workforce.data} getRowKey={(row) => row.id} />;
  }

  if (tab === 'domains' && domains) {
    return (
      <ul className="space-y-2 text-sm">
        {domains.data
          .filter((d) => d.tenantId === tenantId)
          .map((d) => (
            <li key={d.id} className="rounded border border-hairline p-3">
              {d.hostname} — {d.status}
            </li>
          ))}
      </ul>
    );
  }

  if (tab === 'audit' && audit) {
    return (
      <ul className="space-y-2 text-sm">
        {audit.data.map((event) => (
          <li key={event.id} className="rounded border border-hairline p-3">
            {event.action} · {formatDateTime(event.createdAt)}
          </li>
        ))}
      </ul>
    );
  }

  return null;
}
