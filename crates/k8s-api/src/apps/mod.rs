//! Apps API group
//!
//! Contains Deployment, StatefulSet, DaemonSet, and ReplicaSet.

pub mod v1;
pub mod v1beta1;
pub mod v1beta2;

/// GroupName for apps API
pub const GROUP_NAME: &str = "apps";
