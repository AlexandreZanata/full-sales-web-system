import { Outlet, createFileRoute } from '@tanstack/react-router';

import { SettingsNav } from '@/components/settings/SettingsNav';

export const Route = createFileRoute('/_authenticated/settings')({
  component: SettingsLayout,
});

function SettingsLayout() {
  return (
    <div className="space-y-4">
      <SettingsNav />
      <Outlet />
    </div>
  );
}
