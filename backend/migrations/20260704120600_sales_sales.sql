-- Module 04-sales: sales (MIGRATION-SPEC-001-sales)

CREATE SCHEMA IF NOT EXISTS sales;

CREATE TABLE sales.sales (
    id              UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id       UUID NOT NULL REFERENCES shared.tenants (id),
    driver_id       UUID NOT NULL,
    commerce_id     UUID NOT NULL,
    status          VARCHAR(20) NOT NULL DEFAULT 'Pending' CHECK (
        status IN ('Pending', 'Confirmed', 'Cancelled')
    ),
    payment_method  VARCHAR(20) NOT NULL CHECK (
        payment_method IN ('Cash', 'Pix', 'Credit', 'Debit')
    ),
    total_amount    BIGINT NOT NULL DEFAULT 0 CHECK (total_amount >= 0),
    total_currency  CHAR(3) NOT NULL DEFAULT 'BRL',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    confirmed_at    TIMESTAMPTZ
);

CREATE INDEX idx_sales_tenant ON sales.sales (tenant_id);
CREATE INDEX idx_sales_tenant_status ON sales.sales (tenant_id, status);

ALTER TABLE sales.sales ENABLE ROW LEVEL SECURITY;
ALTER TABLE sales.sales FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON sales.sales
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_insert ON sales.sales
    FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_update ON sales.sales
    FOR UPDATE
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);
