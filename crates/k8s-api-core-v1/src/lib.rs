//! Kubernetes core/v1 API types
//!
//! This crate provides types equivalent to `k8s.io/api/core/v1`,
//! including Pod, Service, ConfigMap, Secret, and related types.
//!
//! # Organization
//!
//! The types are organized into modules by category:
//!
//! - [`common`] - Common reference types (ObjectReference, LocalObjectReference)
//! - [`resource`] - Resource-related types (ResourceRequirements, ResourceList)
//! - [`container`] - Container types (Container, ContainerPort, EnvVar, Probe)
//! - [`pod`] - Pod types (Pod, PodSpec, PodStatus)
//!
//! # Re-exports
//!
//! This crate re-exports commonly used types from [`k8s_api_core`] and [`k8s_api_meta`]
//! for convenience.

#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod common;
pub mod container;
pub mod pod;
pub mod resource;

// Re-export common types from dependencies
pub use k8s_api_core::{IntOrString, Quantity};
pub use k8s_api_meta::{
    Duration, LabelSelector, LabelSelectorRequirement, MicroTime, ObjectMeta, OwnerReference, Time,
    TypeMeta,
};

// Re-export types from this crate
pub use common::*;
pub use container::*;
pub use pod::*;
pub use resource::*;
