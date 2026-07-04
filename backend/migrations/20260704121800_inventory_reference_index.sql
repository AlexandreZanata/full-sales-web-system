-- Module 03-inventory: reference_id lookup index (MIGRATION-SPEC-005-stock-movements-reference-index)

CREATE INDEX idx_stock_movements_tenant_reference
    ON inventory.stock_movements (tenant_id, reference_id)
    WHERE reference_id IS NOT NULL;
