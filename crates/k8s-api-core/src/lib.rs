//! Core types and utilities for Kubernetes API
//!
//! This crate provides fundamental types used across all Kubernetes API definitions:
//! - Schema types (GroupVersionKind, GroupVersionResource)
//! - Resource types (Quantity, IntOrString)
//! - Runtime interfaces

pub mod resource;
pub mod runtime;
pub mod schema;

pub use resource::{IntOrString, Quantity};
pub use schema::{GroupKind, GroupResource, GroupVersion, GroupVersionKind, GroupVersionResource};
