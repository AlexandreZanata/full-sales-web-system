-- Module 05-reports: immutable reports enforcement (MIGRATION-SPEC-append-only-hardening)

CREATE OR REPLACE FUNCTION reports.prevent_report_mutation()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'reports are immutable after insert';
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER prevent_report_update_delete
    BEFORE UPDATE OR DELETE ON reports.reports
    FOR EACH ROW EXECUTE FUNCTION reports.prevent_report_mutation();
