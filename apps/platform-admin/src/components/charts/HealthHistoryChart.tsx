import {
  Area,
  AreaChart,
  CartesianGrid,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from 'recharts';

import { ChartTooltip } from '@/components/charts/ChartTooltip';
import { chartColors } from '@/lib/chart-colors';

export type HealthHistoryRow = {
  time: string;
  uptime: number;
};

type HealthHistoryChartProps = {
  rows: HealthHistoryRow[];
  uptimeLabel: string;
};

export function HealthHistoryChart({ rows, uptimeLabel }: HealthHistoryChartProps) {
  if (!rows.length) {
    return null;
  }

  return (
    <div className="h-56 w-full">
      <ResponsiveContainer width="100%" height="100%">
        <AreaChart data={rows} margin={{ top: 8, right: 12, left: 0, bottom: 4 }}>
          <defs>
            <linearGradient id="healthUptimeFill" x1="0" y1="0" x2="0" y2="1">
              <stop offset="0%" stopColor={chartColors.active} stopOpacity={0.35} />
              <stop offset="100%" stopColor={chartColors.active} stopOpacity={0.02} />
            </linearGradient>
          </defs>
          <CartesianGrid strokeDasharray="3 3" stroke={chartColors.grid} vertical={false} />
          <XAxis
            dataKey="time"
            tick={{ fontSize: 11, fill: chartColors.muted }}
            axisLine={false}
            tickLine={false}
            minTickGap={32}
          />
          <YAxis
            domain={[0, 100]}
            tickFormatter={(v: number) => `${String(v)}%`}
            tick={{ fontSize: 11, fill: chartColors.muted }}
            axisLine={false}
            tickLine={false}
            width={36}
          />
          <Tooltip
            content={<ChartTooltip formatter={(value) => `${value.toFixed(0)}% ${uptimeLabel}`} />}
          />
          <Area
            type="monotone"
            dataKey="uptime"
            stroke={chartColors.active}
            strokeWidth={2}
            fill="url(#healthUptimeFill)"
            dot={false}
            activeDot={{ r: 4, fill: chartColors.active }}
          />
        </AreaChart>
      </ResponsiveContainer>
    </div>
  );
}

function statusToUptime(status: string): number {
  if (status === 'healthy' || status === 'ok') return 100;
  if (status === 'degraded' || status === 'warning') return 50;
  return 0;
}

export function mapHealthHistoryPoints(
  points: Array<{ checkedAt: string; status: string }>,
): HealthHistoryRow[] {
  return points.map((point) => ({
    time: new Date(point.checkedAt).toLocaleTimeString(undefined, {
      hour: '2-digit',
      minute: '2-digit',
    }),
    uptime: statusToUptime(point.status),
  }));
}
