import { createFileRoute } from '@tanstack/react-router';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useState, type SubmitEvent } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { EmptyState } from '@/components/ui/EmptyState';
import { Input } from '@/components/ui/Input';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import {
  createDomain,
  deleteDomain,
  fetchDomains,
  setPrimaryDomain,
  verifyDomain,
} from '@/lib/api/domains';
import { useI18n } from '@/lib/i18n/context';
import { useToast } from '@/hooks/useToast';

export const Route = createFileRoute('/_authenticated/settings/domains')({
  component: DomainsSettingsPage,
});

function DomainsSettingsPage() {
  const { t } = useI18n();
  const toast = useToast();
  const queryClient = useQueryClient();
  const [hostname, setHostname] = useState('');
  const [dnsHint, setDnsHint] = useState<string | null>(null);

  const domains = useQuery({ queryKey: ['domains'], queryFn: fetchDomains });

  const addDomain = useMutation({
    mutationFn: () => createDomain(hostname.trim()),
    onSuccess: (created) => {
      setDnsHint(
        t('settings.domains.dnsInstructions')
          .replace('{record}', created.txtRecord)
          .replace('{value}', created.txtValue),
      );
      setHostname('');
      void queryClient.invalidateQueries({ queryKey: ['domains'] });
    },
  });

  const verify = useMutation({
    mutationFn: verifyDomain,
    onSuccess: (result) => {
      toast.success(result.status);
      void queryClient.invalidateQueries({ queryKey: ['domains'] });
    },
  });

  const makePrimary = useMutation({
    mutationFn: setPrimaryDomain,
    onSuccess: () => void queryClient.invalidateQueries({ queryKey: ['domains'] }),
  });

  const remove = useMutation({
    mutationFn: deleteDomain,
    onSuccess: () => void queryClient.invalidateQueries({ queryKey: ['domains'] }),
  });

  if (domains.isLoading) {
    return <LoadingSpinner />;
  }

  return (
    <div className="space-y-4">
      <PageHeader
        title={t('settings.domains.title')}
        description={t('settings.domains.description')}
      />
      <Card className="space-y-3 p-4">
        <form
          className="flex flex-wrap items-end gap-2"
          onSubmit={(event: SubmitEvent) => {
            event.preventDefault();
            addDomain.mutate();
          }}
        >
          <Input
            label={t('settings.domains.hostname')}
            value={hostname}
            onChange={(e) => {
              setHostname(e.target.value);
            }}
            placeholder="shop.example.com"
          />
          <Button type="submit" disabled={!hostname.trim() || addDomain.isPending}>
            {t('settings.domains.add')}
          </Button>
        </form>
        {dnsHint ? <p className="text-xs text-muted-foreground">{dnsHint}</p> : null}
      </Card>

      {domains.data?.data.length ? (
        <ul className="space-y-3">
          {domains.data.data.map((domain) => (
            <li key={domain.id} className="rounded-md border border-hairline p-4 text-sm">
              <div className="flex flex-wrap items-center justify-between gap-2">
                <div>
                  <p className="font-medium">{domain.hostname}</p>
                  <p className="text-muted-foreground">
                    {domain.status}
                    {domain.isPrimary ? ' · primary' : ''}
                  </p>
                </div>
                <div className="flex flex-wrap gap-2">
                  <Button
                    variant="secondary"
                    className="min-h-8"
                    onClick={() => {
                      verify.mutate(domain.id);
                    }}
                  >
                    {t('settings.domains.verify')}
                  </Button>
                  <Button
                    variant="secondary"
                    className="min-h-8"
                    onClick={() => {
                      makePrimary.mutate(domain.id);
                    }}
                  >
                    {t('settings.domains.setPrimary')}
                  </Button>
                  <Button
                    variant="danger"
                    className="min-h-8"
                    onClick={() => {
                      remove.mutate(domain.id);
                    }}
                  >
                    {t('common.remove')}
                  </Button>
                </div>
              </div>
            </li>
          ))}
        </ul>
      ) : (
        <EmptyState title={t('settings.domains.noDomains')} />
      )}
    </div>
  );
}
