//! Kubernetes API version conversion
//!
//! This crate provides version conversion logic for Kubernetes API types.
//!
//! # Overview
//!
//! Kubernetes API types evolve over time, and different versions of the same
//! resource type may have different fields or structures. This crate provides
//! the `Convertible` trait and implementations for converting between versions.
//!
//! # Example
//!
//! ```rust,ignore
//! use k8s_api_conversion::Convertible;
//! use k8s_api::apps::v1beta1::Deployment as DeploymentV1Beta1;
//! use k8s_api::apps::v1::Deployment as DeploymentV1;
//!
//! // Convert from v1beta1 to v1
//! let v1beta1_deployment: DeploymentV1Beta1 = /* ... */;
//! let v1_deployment: DeploymentV1 = v1beta1_deployment.convert_to().unwrap();
//!
//! // Convert back from v1 to v1beta1
//! let converted_back = DeploymentV1Beta1::convert_from(&v1_deployment).unwrap();
//! ```

pub mod admission;
pub mod admissionregistration;
pub mod apidiscovery;
pub mod apps;
pub mod autoscaling;
pub mod batch;
pub mod core;
pub mod authentication;
pub mod authorization;
pub mod certificates;
pub mod coordination;
pub mod discovery;
pub mod events;
pub mod flowcontrol;
pub mod networking;
pub mod node;
pub mod policy;
pub mod rbac;
pub mod scheduling;
pub mod scheme;
pub mod storage;

pub use scheme::{ConversionError, Convertible};
