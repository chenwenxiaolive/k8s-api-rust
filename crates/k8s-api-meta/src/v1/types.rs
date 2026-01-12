//! Core meta/v1 types (placeholder)

use serde::{Deserialize, Serialize};

/// TypeMeta describes an individual object in an API response or request.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeMeta {
    /// APIVersion defines the versioned schema of this representation of an object.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_version: String,

    /// Kind is a string value representing the REST resource this object represents.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,
}

/// ObjectMeta is metadata that all persisted resources must have.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectMeta {
    /// Name must be unique within a namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Namespace defines the space within which each name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
