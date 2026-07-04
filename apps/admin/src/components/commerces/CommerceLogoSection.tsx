import { useState, type SubmitEvent } from 'react';

import { FileUploadField } from '@/components/uploads/FileUploadField';
import { useToast } from '@/hooks/useToast';
import { ApiError } from '@/lib/api/client';
import { updateCommerceLogo } from '@/lib/api/commerces';
import { formatApiErrorMessage } from '@/lib/utils';

type CommerceLogoSectionProps = {
  commerceId: string;
};

export function CommerceLogoSection({ commerceId }: CommerceLogoSectionProps) {
  const toast = useToast();
  const [logoFileId, setLogoFileId] = useState('');
  const [saving, setSaving] = useState(false);

  async function handleLogoChange(fileId: string) {
    setLogoFileId(fileId);
    setSaving(true);
    try {
      await updateCommerceLogo(commerceId, fileId);
      toast.success('Logo updated');
    } catch (error) {
      const message =
        error instanceof ApiError
          ? formatApiErrorMessage(error.message, error.code)
          : 'Unable to update logo';
      toast.error(message);
      setLogoFileId('');
    } finally {
      setSaving(false);
    }
  }

  return (
    <form
      className="space-y-2"
      onSubmit={(event: SubmitEvent<HTMLFormElement>) => {
        event.preventDefault();
      }}
    >
      <FileUploadField
        label="Commerce logo"
        fileId={logoFileId}
        onChange={(fileId) => void handleLogoChange(fileId)}
        entityType="Commerce"
        entityId={commerceId}
      />
      {saving ? <p className="text-xs text-muted-foreground">Saving logo…</p> : null}
    </form>
  );
}
