//! Meta v1 API types
//!
//! Contains the standard metadata types used by all Kubernetes objects.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// TypeMeta describes an individual object in an API response or request
/// with strings representing the type of the object and its API schema version.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeMeta {
    /// Kind is a string value representing the REST resource this object represents.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,

    /// APIVersion defines the versioned schema of this representation of an object.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_version: String,
}

impl TypeMeta {
    pub fn new(api_version: impl Into<String>, kind: impl Into<String>) -> Self {
        Self {
            api_version: api_version.into(),
            kind: kind.into(),
        }
    }
}

/// Time is a wrapper around chrono::DateTime<Utc> for Kubernetes timestamps.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Time(#[serde(with = "chrono::serde::ts_seconds_option")] pub Option<DateTime<Utc>>);

impl Default for Time {
    fn default() -> Self {
        Self(None)
    }
}

impl Time {
    pub fn now() -> Self {
        Self(Some(Utc::now()))
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_none()
    }
}

/// MicroTime is a time with microsecond precision, for Kubernetes resources that need higher precision.
/// In JSON, it is represented as RFC 3339 date-time with microseconds.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MicroTime(pub Option<DateTime<Utc>>);

impl Default for MicroTime {
    fn default() -> Self {
        Self(None)
    }
}

impl MicroTime {
    pub fn now() -> Self {
        Self(Some(Utc::now()))
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_none()
    }
}

/// ListMeta describes metadata that synthetic resources must have.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListMeta {
    /// Deprecated: selfLink is a legacy read-only field.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub self_link: String,

    /// String that identifies the server's internal version of this object.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource_version: String,

    /// Continue token for pagination.
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "continue")]
    pub continue_token: String,

    /// Number of remaining items when using pagination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining_item_count: Option<i64>,
}

/// ObjectMeta is metadata that all persisted resources must have.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectMeta {
    /// Name must be unique within a namespace.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// GenerateName is an optional prefix for generating unique names.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub generate_name: String,

    /// Namespace defines the space within which each name must be unique.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,

    /// Deprecated: selfLink is a legacy read-only field.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub self_link: String,

    /// UID is the unique in time and space value for this object.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub uid: String,

    /// An opaque value that represents the internal version of this object.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource_version: String,

    /// A sequence number representing a specific generation of the desired state.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub generation: i64,

    /// CreationTimestamp is when this object was created.
    #[serde(default, skip_serializing_if = "Time::is_zero")]
    pub creation_timestamp: Time,

    /// DeletionTimestamp is when this resource will be deleted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletion_timestamp: Option<Time>,

    /// Number of seconds allowed for graceful termination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletion_grace_period_seconds: Option<i64>,

    /// Map of string keys and values for organizing objects.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub labels: BTreeMap<String, String>,

    /// Annotations is an unstructured key value map for arbitrary metadata.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub annotations: BTreeMap<String, String>,

    /// List of objects depended by this object.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub owner_references: Vec<OwnerReference>,

    /// Finalizers must be empty before the object is deleted.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub finalizers: Vec<String>,

    /// ManagedFields maps workflow-id and version to managed fields.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub managed_fields: Vec<ManagedFieldsEntry>,
}

fn is_zero(v: &i64) -> bool {
    *v == 0
}

impl ObjectMeta {
    /// Creates a new ObjectMeta with the given name.
    pub fn named(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Creates a new ObjectMeta with the given name and namespace.
    pub fn namespaced(namespace: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            namespace: namespace.into(),
            ..Default::default()
        }
    }
}

/// OwnerReference contains enough information to identify an owning object.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnerReference {
    /// API version of the referent.
    pub api_version: String,

    /// Kind of the referent.
    pub kind: String,

    /// Name of the referent.
    pub name: String,

    /// UID of the referent.
    pub uid: String,

    /// If true, this reference points to the managing controller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controller: Option<bool>,

    /// If true, the owner cannot be deleted until this reference is removed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_owner_deletion: Option<bool>,
}

/// ManagedFieldsEntry is a workflow-id, a FieldSet and the group version of the resource
/// that the fieldset applies to.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedFieldsEntry {
    /// Manager is an identifier of the workflow managing these fields.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub manager: String,

    /// Operation is the type of operation which lead to this ManagedFieldsEntry being created.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub operation: String,

    /// APIVersion defines the version of this resource that this field set applies to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_version: String,

    /// Time is the timestamp of when the ManagedFields entry was added.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Time>,

    /// FieldsType is the discriminator for the different fields format and version.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fields_type: String,

    /// FieldsV1 holds the first JSON version format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields_v1: Option<serde_json::Value>,

    /// Subresource is the name of the subresource used to update that object.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub subresource: String,
}

/// LabelSelector is a label query over a set of resources.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelSelector {
    /// matchLabels is a map of {key,value} pairs.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub match_labels: BTreeMap<String, String>,

    /// matchExpressions is a list of label selector requirements.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_expressions: Vec<LabelSelectorRequirement>,
}

/// LabelSelectorRequirement is a selector that contains values, a key, and an operator.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelSelectorRequirement {
    /// key is the label key that the selector applies to.
    pub key: String,

    /// operator represents a key's relationship to a set of values.
    pub operator: String,

    /// values is an array of string values.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}

/// Status is a return value for calls that don't return other objects.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default)]
    pub metadata: ListMeta,

    /// Status of the operation (Success or Failure).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub status: String,

    /// A human-readable description of the status.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,

    /// A machine-readable description of why this operation is in the status.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,

    /// Extended data associated with the reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<StatusDetails>,

    /// Suggested HTTP return code for this status.
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub code: i32,
}

fn is_zero_i32(v: &i32) -> bool {
    *v == 0
}

/// StatusDetails is a set of additional properties that MAY be set by the server.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusDetails {
    /// The name attribute of the resource associated with the status.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// The group attribute of the resource associated with the status.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,

    /// The kind attribute of the resource associated with the status.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,

    /// UID of the resource.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub uid: String,

    /// The Causes array includes more details associated with the StatusReason failure.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub causes: Vec<StatusCause>,

    /// If specified, the time in seconds before the operation should be retried.
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub retry_after_seconds: i32,
}

/// StatusCause provides more information about a Status failure.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusCause {
    /// A machine-readable description of the cause of the error.
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "reason")]
    pub cause_type: String,

    /// A human-readable description of the cause of the error.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,

    /// The field of the resource that has caused this error.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub field: String,
}

/// Condition contains details for one aspect of the current state of this API Resource.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    /// Type of condition.
    #[serde(rename = "type")]
    pub condition_type: String,

    /// Status of the condition, one of True, False, Unknown.
    pub status: String,

    /// observedGeneration represents the .metadata.generation that the condition was set based upon.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub observed_generation: i64,

    /// lastTransitionTime is the last time the condition transitioned from one status to another.
    pub last_transition_time: Time,

    /// reason contains a programmatic identifier indicating the reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,

    /// message is a human readable message indicating details about the transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_meta_serialize() {
        let meta = ObjectMeta {
            name: "test-pod".to_string(),
            namespace: "default".to_string(),
            labels: [("app".to_string(), "test".to_string())]
                .into_iter()
                .collect(),
            ..Default::default()
        };

        let json = serde_json::to_string_pretty(&meta).unwrap();
        assert!(json.contains("\"name\": \"test-pod\""));
        assert!(json.contains("\"namespace\": \"default\""));
    }

    #[test]
    fn test_type_meta() {
        let tm = TypeMeta::new("v1", "Pod");
        assert_eq!(tm.api_version, "v1");
        assert_eq!(tm.kind, "Pod");
    }
}
