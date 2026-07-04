use serde_json::{Map, Value};

/// Serializes a JSON value with deterministic object key order and no whitespace.
pub fn to_canonical_json(value: &Value) -> String {
    canonicalize(value).to_string()
}

fn canonicalize(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            let mut ordered = Map::new();
            for key in keys {
                ordered.insert(key.clone(), canonicalize(&map[key]));
            }
            Value::Object(ordered)
        }
        Value::Array(items) => Value::Array(items.iter().map(canonicalize).collect()),
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // Contract: BR-RE-001 — object keys sorted deterministically
    #[test]
    fn given_unordered_object_when_canonicalized_then_keys_sorted() {
        let value = json!({"z":1,"a":2,"m":3});
        assert_eq!(to_canonical_json(&value), r#"{"a":2,"m":3,"z":1}"#);
    }
}
