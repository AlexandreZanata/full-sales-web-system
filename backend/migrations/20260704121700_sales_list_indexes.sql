-- Module 04-sales: list query indexes (MIGRATION-SPEC-004-sales-list-indexes)

CREATE INDEX idx_sales_tenant_driver_created
    ON sales.sales (tenant_id, driver_id, created_at DESC);

CREATE INDEX idx_sales_tenant_commerce_created
    ON sales.sales (tenant_id, commerce_id, created_at DESC);

CREATE INDEX idx_sales_tenant_status_created
    ON sales.sales (tenant_id, status, created_at DESC);
