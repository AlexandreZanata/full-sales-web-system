-- Phase 1: PlatformAdmin identity (ADR-013) — no RLS on platform tables.

CREATE TABLE identity.platform_users (
    id              UUID PRIMARY KEY DEFAULT uuidv7(),
    email           VARCHAR(320) NOT NULL UNIQUE,
    name            VARCHAR(200) NOT NULL CHECK (char_length(name) >= 2),
    password_hash   VARCHAR(255) NOT NULL,
    mfa_secret      VARCHAR(128),
    mfa_enrolled    BOOLEAN NOT NULL DEFAULT false,
    active          BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_platform_users_active ON identity.platform_users (active) WHERE active = true;

CREATE TABLE identity.impersonation_grants (
    id                  UUID PRIMARY KEY DEFAULT uuidv7(),
    platform_user_id    UUID NOT NULL REFERENCES identity.platform_users (id),
    target_tenant_id    UUID NOT NULL REFERENCES shared.tenants (id),
    target_user_id      UUID REFERENCES identity.users (id),
    reason              VARCHAR(500) NOT NULL CHECK (char_length(reason) >= 3),
    expires_at          TIMESTAMPTZ NOT NULL,
    revoked_at          TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_impersonation_grants_platform_user
    ON identity.impersonation_grants (platform_user_id, created_at DESC);

GRANT SELECT, INSERT, UPDATE ON identity.platform_users TO app_user;
GRANT SELECT, INSERT, UPDATE ON identity.impersonation_grants TO app_user;
