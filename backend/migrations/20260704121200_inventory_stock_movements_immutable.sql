-- Module 03-inventory: append-only enforcement (MIGRATION-SPEC-append-only-hardening)

REVOKE UPDATE, DELETE ON inventory.stock_movements FROM app_user;

CREATE OR REPLACE FUNCTION inventory.prevent_stock_movement_mutation()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'stock_movements are append-only';
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER prevent_stock_movement_update_delete
    BEFORE UPDATE OR DELETE ON inventory.stock_movements
    FOR EACH ROW EXECUTE FUNCTION inventory.prevent_stock_movement_mutation();
