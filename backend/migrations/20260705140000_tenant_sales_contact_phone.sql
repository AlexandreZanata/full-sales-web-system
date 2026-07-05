-- Phase 50: tenant sales contact phone for portal WhatsApp CTA

ALTER TABLE shared.tenants
    ADD COLUMN sales_contact_phone VARCHAR(20) NULL;

ALTER TABLE shared.tenants
    ADD CONSTRAINT tenants_sales_contact_phone_check CHECK (
        sales_contact_phone IS NULL
        OR (
            sales_contact_phone ~ '^[0-9]+$'
            AND char_length(sales_contact_phone) BETWEEN 10 AND 15
        )
    );
