//! Batch v1 API types

mod types;

pub use types::*;

use k8s_api_core::schema::GroupVersion;

pub fn group_version() -> GroupVersion {
    GroupVersion::new("batch", "v1")
}
