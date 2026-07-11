import { Cell, Pie, PieChart, ResponsiveContainer, Tooltip } from 'recharts';

import { ChartTooltip } from '@/components/charts/ChartTooltip';
import { chartColors } from '@/lib/chart-colors';

export type TenantStatusSlice = {
  key: string;
  label: string;
  value: number;
  color: string;
};

type TenantStatusDonutChartProps = {
  slices: TenantStatusSlice[];
  centerLabel: string;
  centerValue: string | number;
};

export function TenantStatusDonutChart({
  slices,
  centerLabel,
  centerValue,
}: TenantStatusDonutChartProps) {
  const data = slices.filter((slice) => slice.value > 0);
  const isEmpty = data.length === 0;
  const chartData = isEmpty
    ? [{ key: 'empty', label: '—', value: 1, color: chartColors.muted }]
    : data;

  return (
    <div className="relative h-56 w-full">
      <ResponsiveContainer width="100%" height="100%">
        <PieChart>
          <Pie
            data={chartData}
            dataKey="value"
            nameKey="label"
            cx="50%"
            cy="50%"
            innerRadius="62%"
            outerRadius="88%"
            paddingAngle={isEmpty ? 0 : 3}
            stroke="none"
          >
            {chartData.map((entry) => (
              /* eslint-disable-next-line @typescript-eslint/no-deprecated */
              <Cell key={entry.key} fill={entry.color} />
            ))}
          </Pie>
          <Tooltip content={<ChartTooltip formatter={(value) => String(value)} />} />
        </PieChart>
      </ResponsiveContainer>
      <div className="pointer-events-none absolute inset-0 flex flex-col items-center justify-center">
        <span className="text-2xl font-semibold tabular-nums text-foreground">{centerValue}</span>
        <span className="text-xs text-muted-foreground">{centerLabel}</span>
      </div>
    </div>
  );
}
