//! Internal validation helpers.

use crate::{ValidationError, ValidationResult};

pub fn validate_with<Internal, External>(
    value: &Internal,
    field: &str,
    validate: impl FnOnce(&External) -> ValidationResult,
) -> ValidationResult
where
    Internal: serde::Serialize,
    External: serde::de::DeserializeOwned,
{
    match serde_json::to_value(value).and_then(serde_json::from_value::<External>) {
        Ok(external) => validate(&external),
        Err(err) => vec![ValidationError::invalid(
            field,
            format!("failed to convert internal object: {}", err),
        )],
    }
}
