-- Module 03-inventory: product catalog columns (MIGRATION-SPEC-006)

ALTER TABLE inventory.products
    ADD COLUMN category VARCHAR(100),
    ADD COLUMN unit_of_measure VARCHAR(10) NOT NULL DEFAULT 'Unit'
        CHECK (unit_of_measure IN ('Unit', 'Kg', 'Box', 'Liter'));
