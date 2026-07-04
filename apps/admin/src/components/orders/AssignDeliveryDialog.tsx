import { useState } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Select } from '@/components/ui/Select';
import type { User } from '@/lib/api/types';

type AssignDeliveryDialogProps = {
  open: boolean;
  drivers: User[];
  isLoading?: boolean;
  onCancel: () => void;
  onConfirm: (driverId: string) => void;
};

export function AssignDeliveryDialog({
  open,
  drivers,
  isLoading = false,
  onCancel,
  onConfirm,
}: AssignDeliveryDialogProps) {
  const [driverId, setDriverId] = useState('');
  const [error, setError] = useState('');

  if (!open) return null;

  function handleConfirm() {
    if (!driverId) {
      setError('Select a driver');
      return;
    }
    onConfirm(driverId);
  }

  function handleCancel() {
    setDriverId('');
    setError('');
    onCancel();
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-foreground/40 p-4">
      <Card
        className="flex w-full max-w-md flex-col p-6"
        role="dialog"
        aria-modal="true"
        aria-labelledby="assign-delivery-title"
      >
        <h2 id="assign-delivery-title" className="text-lg font-semibold text-foreground">
          Assign delivery
        </h2>
        <p className="mt-2 text-sm text-muted-foreground">
          Select an active driver to handle this delivery.
        </p>
        <div className="mt-4">
          <Select
            label="Driver"
            value={driverId}
            onChange={(event) => {
              setDriverId(event.target.value);
              if (error) {
                setError('');
              }
            }}
            error={error}
            disabled={isLoading}
          >
            <option value="">Select a driver</option>
            {drivers.map((driver) => (
              <option key={driver.id} value={driver.id}>
                {driver.name}
              </option>
            ))}
          </Select>
        </div>
        <div className="mt-6 flex justify-end gap-2">
          <Button variant="secondary" onClick={handleCancel} disabled={isLoading}>
            Cancel
          </Button>
          <Button onClick={handleConfirm} disabled={isLoading || drivers.length === 0}>
            {isLoading ? 'Assigning…' : 'Assign driver'}
          </Button>
        </div>
      </Card>
    </div>
  );
}
