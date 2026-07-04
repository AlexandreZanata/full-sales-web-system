import { useState, type SubmitEvent } from 'react';

import { FileUploadField } from '@/components/uploads/FileUploadField';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { useToast } from '@/hooks/useToast';
import { ApiError } from '@/lib/api/client';
import { upsertDriverProfile } from '@/lib/api/users';
import { formatApiErrorMessage } from '@/lib/utils';

type DriverProfileTabProps = {
  userId: string;
};

type DriverFormValues = {
  cnhNumber: string;
  cnhCategory: string;
  vehiclePlate: string;
  vehicleModel: string;
  vehicleCapacityKg: string;
};

const emptyForm: DriverFormValues = {
  cnhNumber: '',
  cnhCategory: '',
  vehiclePlate: '',
  vehicleModel: '',
  vehicleCapacityKg: '',
};

export function DriverProfileTab({ userId }: DriverProfileTabProps) {
  const toast = useToast();
  const [values, setValues] = useState<DriverFormValues>(emptyForm);
  const [cnhPhotoFileId, setCnhPhotoFileId] = useState('');
  const [submitting, setSubmitting] = useState(false);

  async function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    const body = {
      cnhNumber: values.cnhNumber.trim(),
      cnhCategory: values.cnhCategory.trim(),
      vehiclePlate: values.vehiclePlate.trim(),
      vehicleModel: values.vehicleModel.trim(),
      ...(values.vehicleCapacityKg.trim()
        ? { vehicleCapacityKg: Number(values.vehicleCapacityKg) }
        : {}),
      ...(cnhPhotoFileId ? { cnhPhotoFileId } : {}),
    };

    setSubmitting(true);
    try {
      await upsertDriverProfile(userId, body);
      toast.success('Driver profile saved');
    } catch (error) {
      const message =
        error instanceof ApiError
          ? formatApiErrorMessage(error.message, error.code)
          : 'Unable to save driver profile';
      toast.error(message);
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <form className="space-y-4" onSubmit={(event) => void handleSubmit(event)}>
      <div className="grid gap-4 sm:grid-cols-2">
        <Input
          label="CNH number"
          name="cnhNumber"
          required
          value={values.cnhNumber}
          onChange={(event) => {
            setValues((current) => ({ ...current, cnhNumber: event.target.value }));
          }}
        />
        <Input
          label="CNH category"
          name="cnhCategory"
          required
          value={values.cnhCategory}
          onChange={(event) => {
            setValues((current) => ({ ...current, cnhCategory: event.target.value }));
          }}
        />
        <Input
          label="Vehicle plate"
          name="vehiclePlate"
          required
          value={values.vehiclePlate}
          onChange={(event) => {
            setValues((current) => ({ ...current, vehiclePlate: event.target.value }));
          }}
        />
        <Input
          label="Vehicle model"
          name="vehicleModel"
          required
          value={values.vehicleModel}
          onChange={(event) => {
            setValues((current) => ({ ...current, vehicleModel: event.target.value }));
          }}
        />
        <Input
          label="Vehicle capacity (kg)"
          name="vehicleCapacityKg"
          type="number"
          min="0"
          step="0.01"
          value={values.vehicleCapacityKg}
          onChange={(event) => {
            setValues((current) => ({ ...current, vehicleCapacityKg: event.target.value }));
          }}
        />
      </div>

      <FileUploadField
        label="CNH photo"
        fileId={cnhPhotoFileId}
        onChange={setCnhPhotoFileId}
        entityType="User"
        entityId={userId}
      />

      <Button type="submit" disabled={submitting}>
        {submitting ? 'Saving…' : 'Save driver profile'}
      </Button>
    </form>
  );
}
