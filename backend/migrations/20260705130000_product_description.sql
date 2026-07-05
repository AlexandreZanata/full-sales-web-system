-- Phase 48: optional product description for portal detail page

ALTER TABLE inventory.products
    ADD COLUMN description TEXT NULL
        CHECK (description IS NULL OR char_length(description) <= 2000);
