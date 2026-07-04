-- Module 03-inventory: stock_movements reason column (MIGRATION-SPEC-004-stock-movements-reason)

ALTER TABLE inventory.stock_movements
    ADD COLUMN reason VARCHAR(500);
