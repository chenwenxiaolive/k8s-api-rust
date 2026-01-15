//! API Server Internal v1alpha1 type definitions
//!
//! This module provides internal API server types (K8s 1.14+).
//! These are primarily used for internal API server coordination.

use k8s_apimachinery::apis::meta::v1::{ListMeta, ObjectMeta, Time, TypeMeta};
use serde::{Deserialize, Serialize};

pub type ConditionStatus = String;
pub type StorageVersionConditionType = String;

// =============================================================================
// StorageVersion
// =============================================================================

/// StorageVersionList is a collection of storage versions.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// Items holds a list of StorageVersion.
    pub items: Vec<StorageVersion>,
}

/// StorageVersion is an API resource that stores the storage version of a specific resource.
/// The storage version is the version of the resource that the API server is storing.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersion {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// Spec is an empty spec. It is here to comply with Kubernetes API style.
    #[serde(default)]
    pub spec: StorageVersionSpec,
    /// Status is the actual storage versions of the resource.
    #[serde(default)]
    pub status: StorageVersionStatus,
}

/// StorageVersionSpec is an empty spec.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionSpec {}

/// StorageVersionStatus contains the actual storage versions for a specific resource.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionStatus {
    /// storageVersions is a list of storage versions for each API server.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub storage_versions: Vec<ServerStorageVersion>,
    /// commonEncodingVersion is the encoding version used by all API servers.
    /// It is empty if there is no common encoding version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common_encoding_version: Option<String>,
    /// conditions is a list of conditions for this StorageVersion.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<StorageVersionCondition>,
}

/// ServerStorageVersion contains information about storage version reported by a specific server.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerStorageVersion {
    /// apiServerID is the ID of the reporting API server.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_server_id: String,
    /// encodingVersion is the API encoding version.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub encoding_version: String,
    /// decodableVersions is a list of versions that the API server can decode.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub decodable_versions: Vec<String>,
    /// servedVersions is a list of versions that the API server serves.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub served_versions: Vec<String>,
}

/// StorageVersionCondition describes the state of the storage version.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionCondition {
    /// type is the type of condition.
    #[serde(rename = "type", default, skip_serializing_if = "String::is_empty")]
    pub type_: StorageVersionConditionType,
    /// status is the status of the condition (True, False, Unknown).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub status: ConditionStatus,
    /// observedGeneration is the generation that the condition was set based upon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// lastTransitionTime is the last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Time>,
    /// reason is the reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// message is a human readable message indicating details about the transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

// StorageVersion condition types
pub const STORAGE_VERSION_CONDITION_ALL_ENCODED_VERSIONS_EQUAL: &str = "AllEncodingVersionsEqual";

// Condition status constants
pub const CONDITION_TRUE: &str = "True";
pub const CONDITION_FALSE: &str = "False";
pub const CONDITION_UNKNOWN: &str = "Unknown";
