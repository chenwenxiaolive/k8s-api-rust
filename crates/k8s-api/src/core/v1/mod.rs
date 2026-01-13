//! Core v1 API types
//!
//! This module contains all the core/v1 Kubernetes API types.

mod types;

pub use types::*;

use k8s_api_core::schema::GroupVersion;

/// GroupVersion for core/v1
pub const GROUP_VERSION: GroupVersion = GroupVersion {
    group: String::new(),
    version: String::new(),
};

pub fn group_version() -> GroupVersion {
    GroupVersion::new("", "v1")
}
