import { createFileRoute, redirect, useNavigate } from '@tanstack/react-router';
import { useState, type SubmitEvent } from 'react';

import { useAdminAuth } from '@/auth/useAdminAuth';
import { BrandMark } from '@/components/BrandMark';
import { AccessibilityControls } from '@/components/AccessibilityControls';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { useI18n } from '@/lib/i18n/context';
import { AdminLoginError } from '@/lib/auth/authErrors';
import { ADMIN_APP_TITLE } from '@/lib/brand';

export const Route = createFileRoute('/login')({
  beforeLoad: async ({ context }) => {
    const user = await context.auth.ensureSession();
    if (user) {
      // TanStack Router redirect helper — not a standard Error
      // eslint-disable-next-line @typescript-eslint/only-throw-error
      throw redirect({ to: '/' });
    }
  },
  component: LoginPage,
});

function LoginPage() {
  const { login, enterDevShell } = useAdminAuth();
  const { t } = useI18n();
  const navigate = useNavigate();
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);

  async function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    setError(null);
    setSubmitting(true);

    try {
      await login({ email, password });
      void navigate({ to: '/' });
    } catch (err) {
      setError(
        err instanceof AdminLoginError ? err.message : 'Unable to sign in. Please try again.',
      );
    } finally {
      setSubmitting(false);
    }
  }

  function handleDevEnter() {
    enterDevShell();
    void navigate({ to: '/' });
  }

  return (
    <div className="flex min-h-dvh items-center justify-center bg-surface-muted px-4">
      <Card className="w-full max-w-md">
        <div className="mb-6">
          <AccessibilityControls layout="panel" />
        </div>
        <div className="mb-6">
          <BrandMark size="lg" className="mb-4" />
          <p className="text-xs font-semibold uppercase tracking-[0.2em] text-muted-foreground">
            {ADMIN_APP_TITLE}
          </p>
          <h1 className="mt-2 text-2xl font-semibold text-foreground">{t('auth.signInTitle')}</h1>
          <p className="mt-1 text-sm text-muted-foreground">{t('auth.signInDescription')}</p>
        </div>

        <form className="space-y-4" onSubmit={(event) => void handleSubmit(event)}>
          <Input
            label={t('auth.email')}
            name="email"
            type="email"
            autoComplete="username"
            required
            value={email}
            onChange={(event) => {
              setEmail(event.target.value);
            }}
          />
          <Input
            label={t('auth.password')}
            name="password"
            type="password"
            autoComplete="current-password"
            required
            value={password}
            onChange={(event) => {
              setPassword(event.target.value);
            }}
          />

          {error ? (
            <p className="rounded-md border border-destructive/30 bg-destructive/5 px-3 py-2 text-sm text-destructive">
              {error}
            </p>
          ) : null}

          <Button type="submit" className="w-full" disabled={submitting}>
            {submitting ? t('auth.signingIn') : t('auth.signIn')}
          </Button>
        </form>

        {import.meta.env.DEV ? (
          <div className="mt-4 border-t border-hairline pt-4">
            <Button type="button" variant="secondary" className="w-full" onClick={handleDevEnter}>
              {t('auth.devEnter')}
            </Button>
          </div>
        ) : null}
      </Card>
    </div>
  );
}
