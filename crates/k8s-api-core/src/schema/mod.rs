//! Schema types for Kubernetes API versioning
//!
//! These types represent the group/version/kind and group/version/resource
//! identifiers used throughout the Kubernetes API.

use serde::{Deserialize, Serialize};
use std::fmt;

/// GroupVersionKind identifies a kind within a versioned API group.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GroupVersionKind {
    pub group: String,
    pub version: String,
    pub kind: String,
}

impl GroupVersionKind {
    pub fn new(group: impl Into<String>, version: impl Into<String>, kind: impl Into<String>) -> Self {
        Self {
            group: group.into(),
            version: version.into(),
            kind: kind.into(),
        }
    }

    /// Returns the API version string (e.g., "apps/v1" or "v1" for core)
    pub fn api_version(&self) -> String {
        if self.group.is_empty() {
            self.version.clone()
        } else {
            format!("{}/{}", self.group, self.version)
        }
    }

    pub fn group_version(&self) -> GroupVersion {
        GroupVersion {
            group: self.group.clone(),
            version: self.version.clone(),
        }
    }

    pub fn group_kind(&self) -> GroupKind {
        GroupKind {
            group: self.group.clone(),
            kind: self.kind.clone(),
        }
    }
}

impl fmt::Display for GroupVersionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.group.is_empty() {
            write!(f, "{}, Kind={}", self.version, self.kind)
        } else {
            write!(f, "{}/{}, Kind={}", self.group, self.version, self.kind)
        }
    }
}

/// GroupVersionResource identifies a resource within a versioned API group.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GroupVersionResource {
    pub group: String,
    pub version: String,
    pub resource: String,
}

impl GroupVersionResource {
    pub fn new(group: impl Into<String>, version: impl Into<String>, resource: impl Into<String>) -> Self {
        Self {
            group: group.into(),
            version: version.into(),
            resource: resource.into(),
        }
    }

    pub fn group_version(&self) -> GroupVersion {
        GroupVersion {
            group: self.group.clone(),
            version: self.version.clone(),
        }
    }

    pub fn group_resource(&self) -> GroupResource {
        GroupResource {
            group: self.group.clone(),
            resource: self.resource.clone(),
        }
    }
}

impl fmt::Display for GroupVersionResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.group.is_empty() {
            write!(f, "{}/{}", self.version, self.resource)
        } else {
            write!(f, "{}/{}/{}", self.group, self.version, self.resource)
        }
    }
}

/// GroupVersion identifies a versioned API group.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GroupVersion {
    pub group: String,
    pub version: String,
}

impl GroupVersion {
    pub fn new(group: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            group: group.into(),
            version: version.into(),
        }
    }

    pub fn with_kind(&self, kind: impl Into<String>) -> GroupVersionKind {
        GroupVersionKind {
            group: self.group.clone(),
            version: self.version.clone(),
            kind: kind.into(),
        }
    }

    pub fn with_resource(&self, resource: impl Into<String>) -> GroupVersionResource {
        GroupVersionResource {
            group: self.group.clone(),
            version: self.version.clone(),
            resource: resource.into(),
        }
    }

    /// Returns the API version string
    pub fn api_version(&self) -> String {
        if self.group.is_empty() {
            self.version.clone()
        } else {
            format!("{}/{}", self.group, self.version)
        }
    }
}

impl fmt::Display for GroupVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.api_version())
    }
}

/// GroupKind identifies a kind within an API group (unversioned).
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GroupKind {
    pub group: String,
    pub kind: String,
}

impl GroupKind {
    pub fn new(group: impl Into<String>, kind: impl Into<String>) -> Self {
        Self {
            group: group.into(),
            kind: kind.into(),
        }
    }

    pub fn with_version(&self, version: impl Into<String>) -> GroupVersionKind {
        GroupVersionKind {
            group: self.group.clone(),
            version: version.into(),
            kind: self.kind.clone(),
        }
    }
}

/// GroupResource identifies a resource within an API group (unversioned).
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GroupResource {
    pub group: String,
    pub resource: String,
}

impl GroupResource {
    pub fn new(group: impl Into<String>, resource: impl Into<String>) -> Self {
        Self {
            group: group.into(),
            resource: resource.into(),
        }
    }

    pub fn with_version(&self, version: impl Into<String>) -> GroupVersionResource {
        GroupVersionResource {
            group: self.group.clone(),
            version: version.into(),
            resource: self.resource.clone(),
        }
    }
}
