//! Core traits for Kubernetes resources
//!
//! This module provides the foundational traits that all Kubernetes API types implement.

use std::borrow::Cow;

/// Trait for types that can be deep-copied.
///
/// This is equivalent to Go's `DeepCopyInto` pattern used throughout the Kubernetes codebase.
/// In Rust, most types can simply derive `Clone`, but this trait provides explicit semantics
/// for deep copying complex nested structures.
pub trait DeepCopy: Clone {
    /// Creates a deep copy of this value.
    fn deep_copy(&self) -> Self {
        self.clone()
    }

    /// Copies this value into the target.
    fn deep_copy_into(&self, target: &mut Self) {
        *target = self.deep_copy();
    }
}

// Blanket implementation for all Clone types
impl<T: Clone> DeepCopy for T {}

/// Trait for Kubernetes API resources.
///
/// This trait provides metadata about a Kubernetes resource type, including its
/// API group, version, and kind. It is designed to be compatible with the kube-rs
/// ecosystem.
pub trait Resource {
    /// The API version string (e.g., "v1", "apps/v1")
    const API_VERSION: &'static str;

    /// The API group (e.g., "", "apps", "batch")
    const GROUP: &'static str;

    /// The resource kind (e.g., "Pod", "Deployment")
    const KIND: &'static str;

    /// The API version without group (e.g., "v1", "v1beta1")
    const VERSION: &'static str;

    /// The plural name of the resource (e.g., "pods", "deployments")
    const PLURAL: &'static str;

    /// Returns the API version for this resource.
    fn api_version(&self) -> Cow<'_, str> {
        Cow::Borrowed(Self::API_VERSION)
    }

    /// Returns the kind for this resource.
    fn kind(&self) -> Cow<'_, str> {
        Cow::Borrowed(Self::KIND)
    }
}

/// Marker trait for namespace-scoped resources.
///
/// Resources that implement this trait exist within a namespace.
pub trait NamespacedResource: Resource {}

/// Marker trait for cluster-scoped resources.
///
/// Resources that implement this trait exist at the cluster level
/// and are not namespaced.
pub trait ClusterResource: Resource {}
