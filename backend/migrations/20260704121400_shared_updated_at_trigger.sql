-- Module 00-shared: updated_at trigger function (MIGRATION-SPEC-updated-at-trigger)

CREATE OR REPLACE FUNCTION shared.set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_tenants_updated_at
    BEFORE UPDATE ON shared.tenants
    FOR EACH ROW EXECUTE FUNCTION shared.set_updated_at();

CREATE TRIGGER set_users_updated_at
    BEFORE UPDATE ON identity.users
    FOR EACH ROW EXECUTE FUNCTION shared.set_updated_at();

CREATE TRIGGER set_commerces_updated_at
    BEFORE UPDATE ON commerces.commerces
    FOR EACH ROW EXECUTE FUNCTION shared.set_updated_at();

CREATE TRIGGER set_products_updated_at
    BEFORE UPDATE ON inventory.products
    FOR EACH ROW EXECUTE FUNCTION shared.set_updated_at();
