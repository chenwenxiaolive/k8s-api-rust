//! Kubernetes API validation
//!
//! This crate provides validation logic for Kubernetes API types.

pub mod core;

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
