//! Kubernetes API types for all API groups
//!
//! This crate provides Rust type definitions for Kubernetes API resources.

#[cfg(feature = "admissionregistration")]
pub mod admissionregistration;
#[cfg(feature = "apps")]
pub mod apps;
#[cfg(feature = "autoscaling")]
pub mod autoscaling;
#[cfg(feature = "batch")]
pub mod batch;
#[cfg(feature = "certificates")]
pub mod certificates;
#[cfg(feature = "coordination")]
pub mod coordination;
#[cfg(feature = "core")]
pub mod core;
#[cfg(feature = "discovery")]
pub mod discovery;
#[cfg(feature = "events")]
pub mod events;
#[cfg(feature = "flowcontrol")]
pub mod flowcontrol;
#[cfg(feature = "networking")]
pub mod networking;
#[cfg(feature = "node")]
pub mod node;
#[cfg(feature = "policy")]
pub mod policy;
#[cfg(feature = "rbac")]
pub mod rbac;
#[cfg(feature = "scheduling")]
pub mod scheduling;
#[cfg(feature = "storage")]
pub mod storage;

// Re-export commonly used types
#[cfg(feature = "core")]
pub use core::v1::{
    ConfigMap, Container, EnvVar, Namespace, Node, PersistentVolume, PersistentVolumeClaim, Pod,
    PodSpec, ResourceRequirements, Secret, Service, ServiceAccount, Volume,
};
