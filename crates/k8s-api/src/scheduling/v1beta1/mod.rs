//! Scheduling v1beta1 API type definitions (deprecated)

mod types;
mod internal_conversion;
mod defaults;

pub use types::*;
pub use defaults::*;

pub trait InternalConversion: Sized + serde::Serialize + serde::de::DeserializeOwned {
    type Internal: serde::Serialize + serde::de::DeserializeOwned;

    fn into_internal(&self) -> Result<Self::Internal, serde_json::Error> {
        serde_json::from_value(serde_json::to_value(self)?)
    }

    fn from_internal(internal: &Self::Internal) -> Result<Self, serde_json::Error> {
        serde_json::from_value(serde_json::to_value(internal)?)
    }
}
