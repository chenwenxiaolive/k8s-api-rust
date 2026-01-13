//! Core API group
//!
//! The core API group (also called "legacy" group) contains fundamental Kubernetes resources
//! like Pods, Services, ConfigMaps, and Secrets.

pub mod v1;

/// GroupName is the group name for the core API.
pub const GROUP_NAME: &str = "";
