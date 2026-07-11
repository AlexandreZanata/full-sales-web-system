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
import { chartColors, fraudSeverityColor } from '@/lib/chart-colors';

export type FraudSeverityRow = {
  severity: string;
  count: number;
};

type FraudSeverityChartProps = {
  rows: FraudSeverityRow[];
  eventsLabel: string;
};

export function FraudSeverityChart({ rows, eventsLabel }: FraudSeverityChartProps) {
  if (!rows.length) {
    return null;
  }

  return (
    <div className="h-44 w-full">
      <ResponsiveContainer width="100%" height="100%">
        <BarChart data={rows} margin={{ top: 4, right: 8, left: 0, bottom: 4 }}>
          <CartesianGrid strokeDasharray="3 3" stroke={chartColors.grid} vertical={false} />
          <XAxis
            dataKey="severity"
            tick={{ fontSize: 11, fill: chartColors.muted }}
            axisLine={false}
            tickLine={false}
          />
          <YAxis
            allowDecimals={false}
            tick={{ fontSize: 11, fill: chartColors.muted }}
            axisLine={false}
            tickLine={false}
          />
          <Tooltip
            content={<ChartTooltip formatter={(value) => `${String(value)} ${eventsLabel}`} />}
          />
          <Bar dataKey="count" radius={[6, 6, 0, 0]} maxBarSize={40}>
            {rows.map((row) => (
              /* eslint-disable-next-line @typescript-eslint/no-deprecated -- per-segment fill */
              <Cell key={row.severity} fill={fraudSeverityColor(row.severity)} />
            ))}
          </Bar>
        </BarChart>
      </ResponsiveContainer>
    </div>
  );
}
