import {
  Bar,
  BarChart,
  CartesianGrid,
  Cell,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from 'recharts';

import { ChartTooltip } from '@/components/charts/ChartTooltip';
import { chartColors } from '@/lib/chart-colors';

export type HealthUptimeRow = {
  name: string;
  uptime: number;
  status: string;
};

type HealthUptimeChartProps = {
  rows: HealthUptimeRow[];
  uptimeLabel: string;
};

function uptimeBarColor(uptime: number): string {
  if (uptime >= 99) return chartColors.active;
  if (uptime >= 95) return chartColors.pastDue;
  return chartColors.fraud.high;
}

export function HealthUptimeChart({ rows, uptimeLabel }: HealthUptimeChartProps) {
  return (
    <div className="h-64 w-full">
      <ResponsiveContainer width="100%" height="100%">
        <BarChart data={rows} layout="vertical" margin={{ top: 4, right: 12, left: 4, bottom: 4 }}>
          <CartesianGrid strokeDasharray="3 3" stroke={chartColors.grid} horizontal={false} />
          <XAxis
            type="number"
            domain={[0, 100]}
            tickFormatter={(v: number) => `${String(v)}%`}
            tick={{ fontSize: 11, fill: chartColors.muted }}
            axisLine={false}
            tickLine={false}
          />
          <YAxis
            type="category"
            dataKey="name"
            width={88}
            tick={{ fontSize: 11, fill: chartColors.muted }}
            axisLine={false}
            tickLine={false}
          />
          <Tooltip
            content={<ChartTooltip formatter={(value) => `${value.toFixed(1)}% ${uptimeLabel}`} />}
          />
          <Bar dataKey="uptime" radius={[0, 6, 6, 0]} maxBarSize={18}>
            {rows.map((row) => (
              /* eslint-disable-next-line @typescript-eslint/no-deprecated -- per-segment fill */
              <Cell key={row.name} fill={uptimeBarColor(row.uptime)} />
            ))}
          </Bar>
        </BarChart>
      </ResponsiveContainer>
    </div>
  );
}
