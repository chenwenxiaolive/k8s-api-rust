//! Kubernetes apimachinery types
//!
//! This crate provides types equivalent to `k8s.io/apimachinery/pkg/apis/meta/v1`,
//! including `TypeMeta`, `ObjectMeta`, and related types.

#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod v1;

pub use v1::*;
