-- Raise media upload cap from 5 MiB to 15 MiB (align with domain-media MAX_FILE_SIZE_BYTES).

ALTER TABLE media.files
    DROP CONSTRAINT IF EXISTS files_size_bytes_check;

ALTER TABLE media.files
    ADD CONSTRAINT files_size_bytes_check CHECK (
        size_bytes > 0 AND size_bytes <= 15728640
    );
