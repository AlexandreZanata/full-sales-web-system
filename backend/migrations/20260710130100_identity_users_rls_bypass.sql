-- Phase 1: PlatformAdmin RLS bypass on read (ADR-016).

DROP POLICY tenant_select ON identity.users;

CREATE POLICY tenant_select ON identity.users
    FOR SELECT
    USING (
        tenant_id = nullif(current_setting('app.tenant_id', true), '')::uuid
        OR current_setting('app.bypass_rls', true) = 'true'
    );
