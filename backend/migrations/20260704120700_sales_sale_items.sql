-- Module 04-sales: sale_items (MIGRATION-SPEC-002-sale-items)

CREATE TABLE sales.sale_items (
    id                      UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id               UUID NOT NULL REFERENCES shared.tenants (id),
    sale_id                 UUID NOT NULL REFERENCES sales.sales (id),
    product_id              UUID NOT NULL,
    quantity                INTEGER NOT NULL CHECK (quantity > 0),
    unit_price_amount       BIGINT NOT NULL CHECK (unit_price_amount >= 0),
    unit_price_currency     CHAR(3) NOT NULL,
    line_total_amount       BIGINT NOT NULL CHECK (line_total_amount >= 0),
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_sale_items_sale ON sales.sale_items (tenant_id, sale_id);

ALTER TABLE sales.sale_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE sales.sale_items FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON sales.sale_items
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_insert ON sales.sale_items
    FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_update ON sales.sale_items
    FOR UPDATE
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);
