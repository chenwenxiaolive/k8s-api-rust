//! Conversion scheme and traits

use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

/// Error that can occur during type conversion.
#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("field {field} cannot be converted: {message}")]
    FieldConversion { field: String, message: String },

    #[error("unsupported conversion from {from} to {to}")]
    UnsupportedConversion { from: String, to: String },

    #[error("missing required field: {0}")]
    MissingField(String),
}

/// Trait for types that can be converted to another type.
pub trait Convertible<T> {
    /// Convert this type to the target type.
    fn convert_to(&self) -> Result<T, ConversionError>;

    /// Convert from the target type to this type.
    fn convert_from(other: &T) -> Result<Self, ConversionError>
    where
        Self: Sized;
}

pub(crate) fn convert_via_json<T, U>(value: &T) -> Result<U, ConversionError>
where
    T: Serialize,
    U: DeserializeOwned,
{
    let value = serde_json::to_value(value).map_err(|err| ConversionError::FieldConversion {
        field: "json".to_string(),
        message: err.to_string(),
    })?;
    serde_json::from_value(value).map_err(|err| ConversionError::FieldConversion {
        field: "json".to_string(),
        message: err.to_string(),
    })
}
