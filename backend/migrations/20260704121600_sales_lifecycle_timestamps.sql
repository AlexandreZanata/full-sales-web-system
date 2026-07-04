-- Module 04-sales: lifecycle timestamps (MIGRATION-SPEC-003-sales-lifecycle)

ALTER TABLE sales.sales
    ADD COLUMN cancelled_at TIMESTAMPTZ,
    ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT now();

CREATE TRIGGER set_sales_updated_at
    BEFORE UPDATE ON sales.sales
    FOR EACH ROW EXECUTE FUNCTION shared.set_updated_at();
