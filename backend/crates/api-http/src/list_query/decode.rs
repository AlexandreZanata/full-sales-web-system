/// Decodes `application/x-www-form-urlencoded` query pairs (including `%` escapes).
pub fn decode_query_pairs(query: Option<&str>) -> Vec<(String, String)> {
    let Some(raw) = query else {
        return Vec::new();
    };
    if raw.is_empty() {
        return Vec::new();
    }
    raw.split('&')
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let (key, value) = segment.split_once('=').unwrap_or((segment, ""));
            (percent_decode(key), percent_decode(value))
        })
        .collect()
}

fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'+' => {
                out.push(b' ');
                index += 1;
            }
            b'%' if index + 2 < bytes.len() => {
                let hex = &input[index + 1..index + 3];
                if let Ok(byte) = u8::from_str_radix(hex, 16) {
                    out.push(byte);
                    index += 3;
                    continue;
                }
                out.push(bytes[index]);
                index += 1;
            }
            byte => {
                out.push(byte);
                index += 1;
            }
        }
    }
    String::from_utf8(out).unwrap_or_else(|_| input.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_encoded_query_when_decode_then_pairs() {
        let pairs = decode_query_pairs(Some("filter%5Bactive%5D=true&limit=10"));
        assert_eq!(pairs[0].0, "filter[active]");
        assert_eq!(pairs[0].1, "true");
        assert_eq!(pairs[1].0, "limit");
    }
}
