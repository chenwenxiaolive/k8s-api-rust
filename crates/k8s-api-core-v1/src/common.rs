//! Common reference types used across core/v1 API.
//!
//! This module contains reference types that are used to point to other Kubernetes objects.

use serde::{Deserialize, Serialize};

/// ObjectReference contains enough information to let you inspect or modify the referred object.
///
/// New uses of this type are discouraged because of difficulty describing its usage when embedded in APIs.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectReference {
    /// Kind of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Namespace of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// Name of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// UID of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Specific resourceVersion to which this reference is made, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<String>,

    /// If referring to a piece of an object instead of an entire object, this string
    /// should contain a valid JSON/Go field access statement, such as
    /// desiredState.manifest.containers[2].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_path: Option<String>,
}

/// LocalObjectReference contains enough information to let you locate the
/// referenced object inside the same namespace.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalObjectReference {
    /// Name of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl LocalObjectReference {
    /// Creates a new LocalObjectReference with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
        }
    }
}

impl From<&str> for LocalObjectReference {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

impl From<String> for LocalObjectReference {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}

/// TypedLocalObjectReference contains enough information to let you locate the
/// typed referenced object inside the same namespace.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypedLocalObjectReference {
    /// APIGroup is the group for the resource being referenced.
    /// If APIGroup is not specified, the specified Kind must be in the core API group.
    /// For any other third-party types, APIGroup is required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_group: Option<String>,

    /// Kind is the type of resource being referenced.
    pub kind: String,

    /// Name is the name of resource being referenced.
    pub name: String,
}

/// SecretReference represents a Secret Reference.
/// It has enough information to retrieve secret in any namespace.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretReference {
    /// Name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// ConfigMapKeySelector selects a key of a ConfigMap.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapKeySelector {
    /// The ConfigMap to select from.
    #[serde(flatten)]
    pub local_object_reference: LocalObjectReference,

    /// The key to select.
    pub key: String,

    /// Specify whether the ConfigMap or its key must be defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// SecretKeySelector selects a key of a Secret.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretKeySelector {
    /// The name of the secret in the pod's namespace to select from.
    #[serde(flatten)]
    pub local_object_reference: LocalObjectReference,

    /// The key of the secret to select from. Must be a valid secret key.
    pub key: String,

    /// Specify whether the Secret or its key must be defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_reference_serialize() {
        let obj_ref = ObjectReference {
            kind: Some("Pod".to_string()),
            namespace: Some("default".to_string()),
            name: Some("my-pod".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&obj_ref).unwrap();
        assert!(json.contains("\"kind\":\"Pod\""));
        assert!(json.contains("\"namespace\":\"default\""));
        assert!(json.contains("\"name\":\"my-pod\""));
    }

    #[test]
    fn test_local_object_reference() {
        let local_ref = LocalObjectReference::new("my-secret");
        let json = serde_json::to_string(&local_ref).unwrap();
        assert_eq!(json, "{\"name\":\"my-secret\"}");
    }

    #[test]
    fn test_typed_local_object_reference() {
        let typed_ref = TypedLocalObjectReference {
            api_group: Some("storage.k8s.io".to_string()),
            kind: "StorageClass".to_string(),
            name: "standard".to_string(),
        };
        let json = serde_json::to_string(&typed_ref).unwrap();
        let parsed: TypedLocalObjectReference = serde_json::from_str(&json).unwrap();
        assert_eq!(typed_ref, parsed);
    }

    #[test]
    fn test_config_map_key_selector() {
        let selector = ConfigMapKeySelector {
            local_object_reference: LocalObjectReference::new("my-config"),
            key: "database_url".to_string(),
            optional: Some(false),
        };
        let json = serde_json::to_string(&selector).unwrap();
        assert!(json.contains("\"key\":\"database_url\""));
    }
}
