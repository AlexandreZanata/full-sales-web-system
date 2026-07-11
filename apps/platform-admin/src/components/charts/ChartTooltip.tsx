import type { ReactNode } from 'react';

type ChartTooltipProps = {
  active?: boolean;
  payload?: Array<{ name?: string; value?: number; color?: string }>;
  label?: string;
  formatter?: (value: number, name: string) => ReactNode;
};

export function ChartTooltip({ active, payload, label, formatter }: ChartTooltipProps) {
  if (!active || !payload?.length) {
    return null;
  }

  return (
    <div className="rounded-lg border border-hairline bg-surface px-3 py-2 text-xs shadow-lg">
      {label ? <p className="mb-1 font-medium text-foreground">{label}</p> : null}
      <ul className="space-y-1">
        {payload.map((entry) => (
          <li key={entry.name} className="flex items-center gap-2 text-muted-foreground">
            <span
              className="size-2 rounded-full"
              style={{ backgroundColor: entry.color ?? 'currentColor' }}
            />
            <span className="flex-1">{entry.name}</span>
            <span className="font-medium text-foreground">
              {formatter && entry.value != null && entry.name
                ? formatter(entry.value, entry.name)
                : entry.value}
            </span>
          </li>
        ))}
      </ul>
    </div>
  );
}
