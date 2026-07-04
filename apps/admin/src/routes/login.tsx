import { createFileRoute, redirect, useNavigate } from '@tanstack/react-router';

import { useAdminAuth } from '@/auth/useAdminAuth';
import { BrandMark } from '@/components/BrandMark';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
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
  const { enterDevShell } = useAdminAuth();
  const navigate = useNavigate();

  function handleDevEnter() {
    enterDevShell();
    void navigate({ to: '/' });
  }

  return (
    <div className="flex min-h-dvh items-center justify-center bg-surface-muted px-4">
      <Card className="w-full max-w-md">
        <div className="mb-6">
          <BrandMark size="lg" className="mb-4" />
          <p className="text-xs font-semibold uppercase tracking-[0.2em] text-muted-foreground">
            {ADMIN_APP_TITLE}
          </p>
          <h1 className="mt-2 text-2xl font-semibold text-foreground">Sign in</h1>
          <p className="mt-1 text-sm text-muted-foreground">
            Authentication form will be wired in Phase 29. Use the dev entry below to preview the
            admin shell.
          </p>
        </div>

        {import.meta.env.DEV ? (
          <Button type="button" className="w-full" onClick={handleDevEnter}>
            Enter admin shell (dev)
          </Button>
        ) : (
          <p className="rounded-md border border-hairline bg-surface-muted px-3 py-2 text-sm text-muted-foreground">
            Login is not available yet.
          </p>
        )}
      </Card>
    </div>
  );
}
