-- Product sales totals — incremented on confirmed sales (BR-SA-001).

CREATE TABLE sales.product_sales_totals (
    tenant_id   UUID NOT NULL REFERENCES shared.tenants (id),
    product_id  UUID NOT NULL,
    units_sold  BIGINT NOT NULL DEFAULT 0 CHECK (units_sold >= 0),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (tenant_id, product_id)
);

CREATE INDEX idx_product_sales_totals_tenant_units
    ON sales.product_sales_totals (tenant_id, units_sold DESC);

ALTER TABLE sales.product_sales_totals ENABLE ROW LEVEL SECURITY;
ALTER TABLE sales.product_sales_totals FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON sales.product_sales_totals
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_insert ON sales.product_sales_totals
    FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_update ON sales.product_sales_totals
    FOR UPDATE
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);
