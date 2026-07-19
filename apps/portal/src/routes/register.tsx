import { createFileRoute, Link, redirect } from '@tanstack/react-router';
import { useState, type SubmitEvent } from 'react';

import { LocaleSwitcher } from '@/components/LocaleSwitcher';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { ApiError } from '@/lib/api/client';
import { submitCommerceLead } from '@/lib/api/commerceLeads';
import { useI18n } from '@/lib/i18n/context';

export const Route = createFileRoute('/register')({
  beforeLoad: async ({ context }) => {
    const user = await context.auth.ensureSession();
    if (user) {
      // eslint-disable-next-line @typescript-eslint/only-throw-error
      throw redirect({ to: '/' });
    }
  },
  component: RegisterPage,
});

function RegisterPage() {
  const { t } = useI18n();
  const [contactName, setContactName] = useState('');
  const [phone, setPhone] = useState('');
  const [commerceName, setCommerceName] = useState('');
  const [email, setEmail] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [done, setDone] = useState(false);
  const [submitting, setSubmitting] = useState(false);

  async function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    setError(null);
    setSubmitting(true);
    try {
      await submitCommerceLead({ contactName, phone, commerceName, email });
      setDone(true);
    } catch (err) {
      setError(err instanceof ApiError ? err.message : t('auth.registerFailed'));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <div className="flex min-h-dvh items-center justify-center bg-surface-muted px-4">
      <Card className="w-full max-w-md">
        <div className="mb-4 flex justify-end">
          <LocaleSwitcher />
        </div>
        <div className="mb-6">
          <p className="text-xs font-semibold uppercase tracking-[0.2em] text-muted-foreground">
            {t('auth.portalLabel')}
          </p>
          <h1 className="mt-2 text-2xl font-semibold text-foreground">{t('auth.registerTitle')}</h1>
          <p className="mt-1 text-sm text-muted-foreground">{t('auth.registerDescription')}</p>
        </div>

        {done ? (
          <div className="space-y-4">
            <p className="rounded-md border border-emerald-600/30 bg-emerald-600/5 px-3 py-2 text-sm text-foreground">
              {t('auth.registerSuccess')}
            </p>
            <Link to="/login" className="text-sm font-medium text-primary underline">
              {t('auth.backToSignIn')}
            </Link>
          </div>
        ) : (
          <form className="space-y-4" onSubmit={(event) => void handleSubmit(event)}>
            <Input
              label={t('auth.contactName')}
              name="contactName"
              required
              value={contactName}
              onChange={(event) => setContactName(event.target.value)}
            />
            <Input
              label={t('auth.commerceName')}
              name="commerceName"
              required
              value={commerceName}
              onChange={(event) => setCommerceName(event.target.value)}
            />
            <Input
              label={t('auth.phone')}
              name="phone"
              type="tel"
              required
              value={phone}
              onChange={(event) => setPhone(event.target.value)}
            />
            <Input
              label={t('auth.email')}
              name="email"
              type="email"
              autoComplete="email"
              required
              value={email}
              onChange={(event) => setEmail(event.target.value)}
            />
            {error ? (
              <p className="rounded-md border border-destructive/30 bg-destructive/5 px-3 py-2 text-sm text-destructive">
                {error}
              </p>
            ) : null}
            <Button type="submit" className="w-full" disabled={submitting}>
              {submitting ? t('auth.registering') : t('auth.registerSubmit')}
            </Button>
            <p className="text-center text-sm text-muted-foreground">
              <Link to="/login" className="font-medium text-primary underline">
                {t('auth.backToSignIn')}
              </Link>
            </p>
          </form>
        )}
      </Card>
    </div>
  );
}
