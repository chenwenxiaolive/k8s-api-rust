//! Kubernetes API types for all API groups
//!
//! This crate provides Rust type definitions for Kubernetes API resources.

#[cfg(feature = "apps")]
pub mod apps;
#[cfg(feature = "batch")]
pub mod batch;
#[cfg(feature = "core")]
pub mod core;
#[cfg(feature = "networking")]
pub mod networking;
#[cfg(feature = "rbac")]
pub mod rbac;
#[cfg(feature = "storage")]
pub mod storage;

// Re-export commonly used types
#[cfg(feature = "core")]
pub use core::v1::{
    ConfigMap, Container, EnvVar, Namespace, Node, PersistentVolume, PersistentVolumeClaim, Pod,
    PodSpec, ResourceRequirements, Secret, Service, ServiceAccount, Volume,
};
