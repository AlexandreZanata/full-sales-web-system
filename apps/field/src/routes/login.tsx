import { createFileRoute, redirect, useNavigate } from '@tanstack/react-router';
import { useState, type SubmitEvent } from 'react';

import { useFieldAuth } from '@/auth/useFieldAuth';
import { LocaleSwitcher } from '@/components/LocaleSwitcher';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { FieldLoginError } from '@/lib/auth/authErrors';
import { useI18n } from '@/lib/i18n/context';

export const Route = createFileRoute('/login')({
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
  const { login, enterDevShell } = useFieldAuth();
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
      setError(err instanceof FieldLoginError ? err.message : t('common.loadFailed'));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <div className="flex min-h-dvh items-center justify-center px-4">
      <Card className="w-full max-w-md">
        <div className="mb-4 flex justify-end">
          <LocaleSwitcher />
        </div>
        <h1 className="text-2xl font-semibold">{t('auth.signInTitle')}</h1>
        <p className="mt-1 text-sm text-muted-foreground">{t('auth.signInDescription')}</p>
        <form className="mt-6 space-y-4" onSubmit={(event) => void handleSubmit(event)}>
          <Input
            label={t('auth.email')}
            type="email"
            required
            value={email}
            onChange={(e) => {
              setEmail(e.target.value);
            }}
          />
          <Input
            label={t('auth.password')}
            type="password"
            required
            value={password}
            onChange={(e) => {
              setPassword(e.target.value);
            }}
          />
          {error ? <p className="text-sm text-destructive">{error}</p> : null}
          <Button type="submit" className="w-full" disabled={submitting}>
            {submitting ? t('auth.signingIn') : t('auth.signIn')}
          </Button>
        </form>
        {import.meta.env.DEV ? (
          <Button
            variant="secondary"
            className="mt-4 w-full"
            onClick={() => {
              enterDevShell();
              void navigate({ to: '/' });
            }}
          >
            {t('auth.devEnter')}
          </Button>
        ) : null}
      </Card>
    </div>
  );
}
