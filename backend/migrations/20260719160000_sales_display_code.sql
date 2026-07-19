-- Human-readable sequential sale code for sellers (visual only; UUID remains PK).

CREATE TABLE IF NOT EXISTS sales.sale_display_sequences (
    tenant_id   UUID PRIMARY KEY REFERENCES shared.tenants (id),
    next_value  BIGINT NOT NULL DEFAULT 1 CHECK (next_value >= 1)
);

ALTER TABLE sales.sale_display_sequences ENABLE ROW LEVEL SECURITY;
ALTER TABLE sales.sale_display_sequences FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_all ON sales.sale_display_sequences
    FOR ALL
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE OR REPLACE FUNCTION sales.format_display_code(n BIGINT)
RETURNS VARCHAR(8)
LANGUAGE plpgsql
IMMUTABLE
AS $$
DECLARE
    alphabet CONSTANT TEXT := '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ';
    result TEXT := '';
    v BIGINT := n;
    rem INT;
BEGIN
    IF n IS NULL OR n < 1 THEN
        RAISE EXCEPTION 'display code sequence must be >= 1';
    END IF;
    WHILE v > 0 LOOP
        rem := (v % 36)::INT;
        result := substr(alphabet, rem + 1, 1) || result;
        v := v / 36;
    END LOOP;
    RETURN lpad(result, 8, '0');
END;
$$;

ALTER TABLE sales.sales
    ADD COLUMN IF NOT EXISTS display_code VARCHAR(8);

DO $$
DECLARE
    r RECORD;
    seq BIGINT;
BEGIN
    FOR r IN
        SELECT id, tenant_id
        FROM sales.sales
        WHERE display_code IS NULL
        ORDER BY tenant_id, created_at ASC, id ASC
    LOOP
        PERFORM set_config('app.tenant_id', r.tenant_id::text, true);

        INSERT INTO sales.sale_display_sequences AS s (tenant_id, next_value)
        VALUES (r.tenant_id, 1)
        ON CONFLICT (tenant_id) DO UPDATE
            SET next_value = s.next_value + 1
        RETURNING s.next_value INTO seq;

        UPDATE sales.sales
        SET display_code = sales.format_display_code(seq)
        WHERE id = r.id;
    END LOOP;
END $$;

ALTER TABLE sales.sales
    ALTER COLUMN display_code SET NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS uq_sales_tenant_display_code
    ON sales.sales (tenant_id, display_code);
