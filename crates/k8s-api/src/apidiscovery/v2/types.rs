//! API Discovery v2 type definitions
//!
//! This module provides types for API discovery (K8s 1.30+).
//! These types are returned from /api and /apis endpoints and contain
//! aggregated lists of API resources that a cluster supports.

use k8s_apimachinery::apis::meta::v1::{ListMeta, ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

pub type DiscoveryFreshness = String;
pub type ResourceScope = String;

// =============================================================================
// APIGroupDiscovery
// =============================================================================

/// APIGroupDiscoveryList is a resource containing a list of APIGroupDiscovery.
/// This is one of the types able to be returned from the /api and /apis endpoint.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIGroupDiscoveryList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// items is the list of groups for discovery. The groups are listed in priority order.
    pub items: Vec<APIGroupDiscovery>,
}

/// APIGroupDiscovery holds information about which resources are being served
/// for all versions of the API Group.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIGroupDiscovery {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// versions are the versions supported in this group.
    /// They are sorted in descending order of preference.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub versions: Vec<APIVersionDiscovery>,
}

/// APIVersionDiscovery holds a list of APIResourceDiscovery types
/// that are served for a particular version within an API Group.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIVersionDiscovery {
    /// version is the name of the version within a group version.
    pub version: String,
    /// resources is a list of APIResourceDiscovery objects for the corresponding group version.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<APIResourceDiscovery>,
    /// freshness marks whether a group version's discovery document is up to date.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub freshness: DiscoveryFreshness,
}

/// APIResourceDiscovery provides information about an API resource for discovery.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIResourceDiscovery {
    /// resource is the plural name of the resource.
    pub resource: String,
    /// responseKind describes the group, version, and kind of the serialization schema.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_kind: Option<GroupVersionKind>,
    /// scope indicates the scope of a resource, either Cluster or Namespaced.
    pub scope: ResourceScope,
    /// singularResource is the singular name of the resource.
    pub singular_resource: String,
    /// verbs is a list of supported API operation types.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verbs: Vec<String>,
    /// shortNames is a list of suggested short names of the resource.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub short_names: Vec<String>,
    /// categories is a list of the grouped resources this resource belongs to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<String>,
    /// subresources is a list of subresources provided by this resource.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subresources: Vec<APISubresourceDiscovery>,
}

/// APISubresourceDiscovery provides information about an API subresource for discovery.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APISubresourceDiscovery {
    /// subresource is the name of the subresource.
    pub subresource: String,
    /// responseKind describes the group, version, and kind of the serialization schema.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_kind: Option<GroupVersionKind>,
    /// acceptedTypes describes the kinds that this endpoint accepts.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accepted_types: Vec<GroupVersionKind>,
    /// verbs is a list of supported API operation types.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verbs: Vec<String>,
}

/// GroupVersionKind unambiguously identifies a kind.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupVersionKind {
    pub group: String,
    pub version: String,
    pub kind: String,
}

// Resource scope constants
pub const SCOPE_CLUSTER: &str = "Cluster";
pub const SCOPE_NAMESPACE: &str = "Namespaced";

// Discovery freshness constants
pub const DISCOVERY_FRESHNESS_CURRENT: &str = "Current";
pub const DISCOVERY_FRESHNESS_STALE: &str = "Stale";
