//! Core traits and primitive types for Kubernetes API
//!
//! This crate provides foundational traits and types used across all Kubernetes API groups.
//!
//! # Traits
//!
//! - [`DeepCopy`] - Trait for deep copying Kubernetes objects
//! - [`Resource`] - Trait providing API metadata for Kubernetes resources
//! - [`NamespacedResource`] - Marker trait for namespace-scoped resources
//! - [`ClusterResource`] - Marker trait for cluster-scoped resources
//!
//! # Primitive Types
//!
//! - [`IntOrString`] - A value that can be either an integer or a string
//! - [`Quantity`] - A fixed-point representation for resource quantities

#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod primitives;
pub mod quantity;
pub mod traits;

pub use primitives::IntOrString;
pub use quantity::{Format, Quantity, QuantityError, Scale};
pub use traits::{ClusterResource, DeepCopy, NamespacedResource, Resource};
