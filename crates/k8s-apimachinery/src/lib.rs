//! Kubernetes API machinery types
//!
//! This crate provides the core metadata types used by all Kubernetes API objects:
//! - TypeMeta: API version and kind
//! - ObjectMeta: Standard object metadata
//! - ListMeta: Metadata for list responses
//! - Status: API response status

pub mod apis;
pub mod types;

pub use apis::meta::v1::{
    Condition, FieldSelectorOperator, FieldSelectorRequirement, LabelSelector, LabelSelectorRequirement,
    ListMeta, ManagedFieldsEntry, ObjectMeta, OwnerReference, Status, StatusCause, StatusDetails,
    Time, TypeMeta,
};
pub use types::{NamespacedName, UID};
