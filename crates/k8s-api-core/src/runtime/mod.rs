//! Runtime interfaces and traits for Kubernetes objects

use crate::schema::GroupVersionKind;

/// Object is the base trait for all Kubernetes API objects.
pub trait Object {
    /// Returns the GroupVersionKind for this object type.
    fn gvk(&self) -> GroupVersionKind;

    /// Returns the API version string (e.g., "v1" or "apps/v1").
    fn api_version(&self) -> &str;

    /// Returns the Kind string.
    fn kind(&self) -> &str;
}

/// DeepCopy trait for cloning Kubernetes objects.
pub trait DeepCopy: Clone {
    fn deep_copy(&self) -> Self {
        self.clone()
    }
}

impl<T: Clone> DeepCopy for T {}
