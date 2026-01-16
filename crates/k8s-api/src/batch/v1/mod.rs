//! Batch v1 API types

mod types;
mod internal_conversion;

pub use types::*;

pub trait InternalConversion: Sized + serde::Serialize + serde::de::DeserializeOwned {
    type Internal: serde::Serialize + serde::de::DeserializeOwned;

    fn into_internal(&self) -> Result<Self::Internal, serde_json::Error> {
        serde_json::from_value(serde_json::to_value(self)?)
    }

    fn from_internal(internal: &Self::Internal) -> Result<Self, serde_json::Error> {
        serde_json::from_value(serde_json::to_value(internal)?)
    }
}

use k8s_api_core::schema::GroupVersion;

pub fn group_version() -> GroupVersion {
    GroupVersion::new("batch", "v1")
}
