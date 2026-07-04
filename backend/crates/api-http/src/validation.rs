use axum::body::Bytes;
use axum::extract::FromRequest;
use axum::http::Request;
use serde::de::DeserializeOwned;

use crate::error::ApiError;

/// JSON body parser that maps deserialization errors to 400 (API contract).
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(
        req: Request<axum::body::Body>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let bytes = Bytes::from_request(req, state).await.map_err(|_| {
            ApiError::bad_request("INVALID_JSON", "Request body must be valid JSON")
        })?;

        serde_json::from_slice::<T>(&bytes)
            .map(ValidatedJson)
            .map_err(|err| {
                if err.is_data() {
                    ApiError::bad_request("VALIDATION_ERROR", "Invalid request body")
                } else {
                    ApiError::bad_request("INVALID_JSON", "Request body must be valid JSON")
                }
            })
    }
}

/// Serializes a value to JSON bytes for idempotency replay storage.
pub fn to_json_bytes<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, ApiError> {
    serde_json::to_vec(value).map_err(|_| ApiError::internal())
}

impl<T> ValidatedJson<T> {
    #[allow(dead_code)]
    pub fn into_inner(self) -> T {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Sample {
        #[allow(dead_code)]
        name: String,
    }

    #[test]
    fn given_unknown_field_when_deserialize_then_error() {
        let json = r#"{"name":"x","extra":1}"#;
        let result = serde_json::from_str::<Sample>(json);
        assert!(result.is_err());
    }
}
