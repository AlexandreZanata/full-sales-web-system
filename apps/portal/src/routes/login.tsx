import { createFileRoute, Link, redirect, useNavigate } from '@tanstack/react-router';
import { useState, type SubmitEvent } from 'react';

import { usePortalAuth } from '@/auth/usePortalAuth';
import { LocaleSwitcher } from '@/components/LocaleSwitcher';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { PortalLoginError } from '@/lib/auth/authErrors';
import { resolvePostLoginRedirect } from '@/lib/auth/postLoginRedirect';
import { useI18n } from '@/lib/i18n/context';

type LoginSearch = {
  redirect?: string;
};

export const Route = createFileRoute('/login')({
  validateSearch: (search: Record<string, unknown>): LoginSearch => ({
    redirect: typeof search.redirect === 'string' ? search.redirect : undefined,
  }),
  beforeLoad: async ({ context }) => {
    const user = await context.auth.ensureSession();
    if (user) {
      // eslint-disable-next-line @typescript-eslint/only-throw-error
      throw redirect({ to: '/' });
    }
  },
  component: LoginPage,
});

function LoginPage() {
  const { login, enterDevShell } = usePortalAuth();
  const { t } = useI18n();
  const navigate = useNavigate();
  const { redirect: redirectTo } = Route.useSearch();
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
      void navigate({ to: resolvePostLoginRedirect(redirectTo) });
    } catch (err) {
      setError(
        err instanceof PortalLoginError ? err.message : 'Unable to sign in. Please try again.',
      );
    } finally {
      setSubmitting(false);
    }
  }

  function handleDevEnter() {
    enterDevShell();
    void navigate({ to: resolvePostLoginRedirect(redirectTo) });
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

        <p className="mt-4 text-center text-sm text-muted-foreground">
          <Link to="/register" className="font-medium text-primary underline">
            {t('auth.registerLink')}
          </Link>
        </p>

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
