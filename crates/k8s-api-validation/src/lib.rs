//! Kubernetes API validation
//!
//! This crate provides validation logic for Kubernetes API types.

pub mod apps;
pub mod autoscaling;
pub mod admissionregistration;
pub mod batch;
pub mod certificates;
pub mod common;
pub mod coordination;
pub mod core;
pub mod discovery;
pub mod flowcontrol;
pub mod networking;
pub mod node;
pub mod policy;
pub mod rbac;
pub mod resource;
pub mod storage;

use thiserror::Error;

/// ValidationError represents an error during validation.
#[derive(Debug, Error, Clone)]
#[error("{field}: {message}")]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub error_type: ErrorType,
}

/// ErrorType categorizes the type of validation error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorType {
    Required,
    Invalid,
    Duplicate,
    TooLong,
    TooShort,
    Forbidden,
    NotSupported,
    NotFound,
    InvalidValue,
    OutOfRange,
}

impl ValidationError {
    pub fn required(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
            error_type: ErrorType::Required,
        }
    }

    pub fn invalid(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
            error_type: ErrorType::Invalid,
        }
    }

    pub fn too_long(field: impl Into<String>, max_len: usize, actual_len: usize) -> Self {
        Self {
            field: field.into(),
            message: format!("must be no more than {} characters, got {}", max_len, actual_len),
            error_type: ErrorType::TooLong,
        }
    }

    pub fn duplicate(field: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: format!("duplicate value: {}", value.into()),
            error_type: ErrorType::Duplicate,
        }
    }

    pub fn forbidden(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
            error_type: ErrorType::Forbidden,
        }
    }

    pub fn not_supported(field: impl Into<String>, value: impl Into<String>, supported: &[&str]) -> Self {
        Self {
            field: field.into(),
            message: format!("unsupported value: {}, must be one of {:?}", value.into(), supported),
            error_type: ErrorType::NotSupported,
        }
    }

    pub fn out_of_range(field: impl Into<String>, min: i64, max: i64, actual: i64) -> Self {
        Self {
            field: field.into(),
            message: format!("must be between {} and {}, got {}", min, max, actual),
            error_type: ErrorType::OutOfRange,
        }
    }
}

/// ValidationResult is a collection of validation errors.
pub type ValidationResult = Vec<ValidationError>;

/// Trait for types that can be validated.
pub trait Validate {
    fn validate(&self) -> ValidationResult;

    fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }
}

/// Helper to build field paths for nested validation.
pub struct FieldPath {
    path: String,
}

impl FieldPath {
    pub fn new(root: impl Into<String>) -> Self {
        Self { path: root.into() }
    }

    pub fn child(&self, name: &str) -> Self {
        Self {
            path: format!("{}.{}", self.path, name),
        }
    }

    pub fn index(&self, idx: usize) -> Self {
        Self {
            path: format!("{}[{}]", self.path, idx),
        }
    }

    pub fn key(&self, key: &str) -> Self {
        Self {
            path: format!("{}[{}]", self.path, key),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.path
    }
}

impl std::fmt::Display for FieldPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}
