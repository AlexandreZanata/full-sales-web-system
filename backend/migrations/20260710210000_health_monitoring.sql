-- Phase 9: system health monitoring (probes, alerts, retention).

CREATE TABLE ops.health_probe_results (
    id          UUID PRIMARY KEY DEFAULT uuidv7(),
    probe_name  VARCHAR(50) NOT NULL,
    status      VARCHAR(20) NOT NULL CHECK (status IN ('up', 'down', 'degraded')),
    latency_ms  INT,
    checked_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    details     JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE INDEX idx_health_probe_results_name_time
    ON ops.health_probe_results (probe_name, checked_at DESC);

CREATE INDEX idx_health_probe_results_checked
    ON ops.health_probe_results (checked_at);

CREATE TABLE ops.ops_alerts (
    id           UUID PRIMARY KEY DEFAULT uuidv7(),
    probe_name   VARCHAR(50) NOT NULL,
    alert_type   VARCHAR(30) NOT NULL DEFAULT 'probe_failure',
    message      TEXT NOT NULL,
    details      JSONB NOT NULL DEFAULT '{}'::jsonb,
    webhook_sent BOOLEAN NOT NULL DEFAULT false,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_ops_alerts_probe_created
    ON ops.ops_alerts (probe_name, created_at DESC);

GRANT SELECT, INSERT, DELETE ON ops.health_probe_results TO app_user;
GRANT SELECT, INSERT ON ops.ops_alerts TO app_user;
