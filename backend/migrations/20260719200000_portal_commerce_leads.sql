-- Portal commerce interest leads (self-serve signup request; admin reviews)

CREATE TABLE commerces.portal_leads (
    id             UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id      UUID NOT NULL REFERENCES shared.tenants (id),
    contact_name   VARCHAR(200) NOT NULL CHECK (char_length(trim(contact_name)) >= 1),
    phone          VARCHAR(15) NOT NULL CHECK (phone ~ '^[0-9]+$' AND char_length(phone) BETWEEN 10 AND 15),
    commerce_name  VARCHAR(200) NOT NULL CHECK (char_length(trim(commerce_name)) >= 1),
    email          VARCHAR(320) NOT NULL CHECK (position('@' in email) > 1),
    status         VARCHAR(20) NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending', 'approved', 'rejected')),
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    reviewed_at    TIMESTAMPTZ,
    reviewed_by    UUID REFERENCES identity.users (id)
);

CREATE INDEX idx_portal_leads_tenant_status_created
    ON commerces.portal_leads (tenant_id, status, created_at DESC);

ALTER TABLE commerces.portal_leads ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerces.portal_leads FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON commerces.portal_leads
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_insert ON commerces.portal_leads
    FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_update ON commerces.portal_leads
    FOR UPDATE
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

GRANT SELECT, INSERT, UPDATE ON commerces.portal_leads TO app_user;
