//! API Server Internal v1alpha1 type definitions
//!
//! This module provides internal API server types (K8s 1.14+).
//! These are primarily used for internal API server coordination.

use k8s_apimachinery::apis::meta::v1::{Condition, ListMeta, ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

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
    pub conditions: Vec<Condition>,
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

// StorageVersion condition types
pub const STORAGE_VERSION_CONDITION_ALL_ENCODED_VERSIONS_EQUAL: &str = "AllEncodedVersionsEqual";
