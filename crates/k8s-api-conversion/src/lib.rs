//! Kubernetes API version conversion
//!
//! This crate provides version conversion logic for Kubernetes API types.

pub mod core;
pub mod scheme;

pub use scheme::{ConversionError, Convertible};
