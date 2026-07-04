-- Module 04-sales: order link + declared payment (MIGRATION-SPEC-005-sales-order-payment)

ALTER TABLE sales.sales
    ADD COLUMN order_id UUID,
    ADD COLUMN declared_payment_method VARCHAR(20) NOT NULL DEFAULT 'NotDeclared'
        CHECK (declared_payment_method IN (
            'Cash', 'Pix', 'Card', 'Boleto', 'Other', 'NotDeclared'
        )),
    ADD COLUMN declared_payment_received BOOLEAN NOT NULL DEFAULT false,
    ADD COLUMN declared_payment_at TIMESTAMPTZ,
    ADD COLUMN declared_payment_by_user_id UUID,
    ADD COLUMN declared_payment_notes TEXT;

ALTER TABLE sales.sales DROP CONSTRAINT sales_payment_method_check;
ALTER TABLE sales.sales ADD CONSTRAINT sales_payment_method_check CHECK (
    payment_method IN ('Cash', 'Pix', 'Credit', 'Debit', 'NotDeclared')
);

CREATE INDEX idx_sales_tenant_order
    ON sales.sales (tenant_id, order_id)
    WHERE order_id IS NOT NULL;
