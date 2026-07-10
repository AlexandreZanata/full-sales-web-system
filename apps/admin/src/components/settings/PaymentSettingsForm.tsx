import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useState, type SubmitEvent } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import {
  connectAsaas,
  disconnectAsaas,
  fetchPaymentBalance,
  type PaymentSettings,
  updatePaymentSettings,
} from '@/lib/api/payments';
import { useI18n } from '@/lib/i18n/context';
import { useToast } from '@/hooks/useToast';

type PaymentSettingsFormProps = {
  settings: PaymentSettings;
};

export function PaymentSettingsForm({ settings }: PaymentSettingsFormProps) {
  const { t } = useI18n();
  const toast = useToast();
  const queryClient = useQueryClient();
  const [apiKey, setApiKey] = useState('');

  const saveSettings = useMutation({
    mutationFn: updatePaymentSettings,
    onSuccess: () => void queryClient.invalidateQueries({ queryKey: ['payments'] }),
  });
  const connect = useMutation({
    mutationFn: () => connectAsaas(apiKey.trim()),
    onSuccess: (result) => {
      toast.success(t('settings.payments.connectedAs').replace('{name}', result.accountName));
      setApiKey('');
      void queryClient.invalidateQueries({ queryKey: ['payments'] });
    },
  });
  const disconnect = useMutation({
    mutationFn: disconnectAsaas,
    onSuccess: () => void queryClient.invalidateQueries({ queryKey: ['payments'] }),
  });

  const patch = (patch: Partial<Pick<PaymentSettings, 'enabled' | 'methods' | 'autoCapture'>>) => {
    saveSettings.mutate({
      enabled: patch.enabled ?? settings.enabled,
      methods: patch.methods ?? settings.methods,
      autoCapture: patch.autoCapture ?? settings.autoCapture,
    });
  };

  return (
    <>
      <Card className="space-y-3 p-4">
        <form
          className="flex flex-wrap items-end gap-2"
          onSubmit={(event: SubmitEvent) => {
            event.preventDefault();
            connect.mutate();
          }}
        >
          <Input
            label={t('settings.payments.apiKey')}
            value={apiKey}
            onChange={(e) => {
              setApiKey(e.target.value);
            }}
          />
          <Button type="submit" disabled={!apiKey.trim() || connect.isPending}>
            {t('settings.payments.connect')}
          </Button>
          {settings.asaas.connected ? (
            <Button
              type="button"
              variant="danger"
              onClick={() => {
                disconnect.mutate();
              }}
            >
              {t('settings.payments.disconnect')}
            </Button>
          ) : null}
        </form>
        {settings.asaas.connected ? (
          <Button
            variant="secondary"
            onClick={() => {
              void queryClient.fetchQuery({
                queryKey: ['payments', 'balance'],
                queryFn: fetchPaymentBalance,
              });
              toast.success('Connection OK');
            }}
          >
            {t('settings.payments.testConnection')}
          </Button>
        ) : null}
      </Card>

      <Card className="space-y-3 p-4">
        <label className="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            checked={settings.enabled}
            onChange={(e) => {
              patch({ enabled: e.target.checked });
            }}
          />
          {t('settings.payments.enableOnline')}
        </label>
        {(['pix', 'credit', 'boleto'] as const).map((method) => {
          const label =
            method === 'pix'
              ? t('settings.payments.methodPix')
              : method === 'credit'
                ? t('settings.payments.methodCredit')
                : t('settings.payments.methodBoleto');
          return (
            <label key={method} className="flex items-center gap-2 text-sm">
              <input
                type="checkbox"
                checked={settings.methods[method]}
                onChange={(e) => {
                  patch({ methods: { ...settings.methods, [method]: e.target.checked } });
                }}
              />
              {label}
            </label>
          );
        })}
        <label className="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            checked={settings.autoCapture}
            onChange={(e) => {
              patch({ autoCapture: e.target.checked });
            }}
          />
          {t('settings.payments.autoCapture')}
        </label>
      </Card>
    </>
  );
}
