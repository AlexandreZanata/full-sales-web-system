-- Phase 69: seller commerce registration workflow

ALTER TABLE identity.users
    ADD COLUMN can_review_commerce BOOLEAN NOT NULL DEFAULT false;

ALTER TABLE commerces.commerces
    ADD COLUMN registration_status VARCHAR(20) NOT NULL DEFAULT 'Active'
        CHECK (registration_status IN ('Active', 'PendingReview', 'Rejected')),
    ADD COLUMN submitted_by_user_id UUID REFERENCES identity.users (id),
    ADD COLUMN reviewed_by_user_id UUID REFERENCES identity.users (id),
    ADD COLUMN rejection_reason TEXT,
    ADD COLUMN lookup_snapshot JSONB,
    ADD COLUMN registration_mode VARCHAR(20)
        CHECK (registration_mode IS NULL OR registration_mode IN ('cnpj_lookup', 'manual'));

CREATE INDEX idx_commerces_tenant_registration_status
    ON commerces.commerces (tenant_id, registration_status);

CREATE INDEX idx_commerces_submitted_by
    ON commerces.commerces (tenant_id, submitted_by_user_id)
    WHERE submitted_by_user_id IS NOT NULL;
