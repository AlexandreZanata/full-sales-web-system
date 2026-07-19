use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::PostgresError;

/// Formats a 1-based sequence as an 8-char uppercase base36 code (e.g. 1 → 00000001).
pub fn format_display_code(sequence: u64) -> String {
    assert!(sequence >= 1, "display code sequence must be >= 1");
    const ALPHABET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut n = sequence;
    let mut buf = [b'0'; 8];
    let mut i = 8;
    while n > 0 {
        i -= 1;
        buf[i] = ALPHABET[(n % 36) as usize];
        n /= 36;
    }
    String::from_utf8(buf.to_vec()).expect("ascii alphabet")
}

/// Allocates the next per-tenant display code inside an open transaction (RLS context applied).
pub async fn allocate_display_code(
    tx: &mut Transaction<'_, Postgres>,
    tenant_id: Uuid,
) -> Result<String, PostgresError> {
    let seq: i64 = sqlx::query_scalar(
        r#"
        INSERT INTO sales.sale_display_sequences AS s (tenant_id, next_value)
        VALUES ($1, 1)
        ON CONFLICT (tenant_id) DO UPDATE
            SET next_value = s.next_value + 1
        RETURNING s.next_value
        "#,
    )
    .bind(tenant_id)
    .fetch_one(&mut **tx)
    .await?;
    Ok(format_display_code(seq as u64))
}

#[cfg(test)]
mod tests {
    use super::format_display_code;

    #[test]
    fn format_display_code_pads_base36() {
        assert_eq!(format_display_code(1), "00000001");
        assert_eq!(format_display_code(10), "0000000A");
        assert_eq!(format_display_code(36), "00000010");
    }
}
