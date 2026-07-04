-- Module 01-identity: user commerce contact delta (MIGRATION-SPEC-002)

ALTER TABLE identity.users
    ADD COLUMN commerce_id UUID,
    ADD COLUMN profile_file_id UUID REFERENCES media.files (id);

ALTER TABLE identity.users DROP CONSTRAINT IF EXISTS users_role_check;
ALTER TABLE identity.users ADD CONSTRAINT users_role_check
    CHECK (role IN ('Admin', 'Driver', 'Seller', 'CommerceContact'));

ALTER TABLE identity.users ADD CONSTRAINT users_commerce_contact_commerce
    CHECK (
        (role = 'CommerceContact' AND commerce_id IS NOT NULL)
        OR (role <> 'CommerceContact' AND commerce_id IS NULL)
    );

CREATE INDEX idx_users_commerce_id ON identity.users (commerce_id)
    WHERE commerce_id IS NOT NULL;
