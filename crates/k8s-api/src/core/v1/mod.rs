//! Core v1 API types
//!
//! This module contains all the core/v1 Kubernetes API types.

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

/// GroupVersion for core/v1
pub const GROUP_VERSION: GroupVersion = GroupVersion {
    group: String::new(),
    version: String::new(),
};

pub fn group_version() -> GroupVersion {
    GroupVersion::new("", "v1")
}
