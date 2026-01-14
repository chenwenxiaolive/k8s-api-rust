//! API Registration v1 type definitions
//!
//! This module provides Rust type definitions for APIService, which is used
//! for API aggregation in Kubernetes.

use k8s_apimachinery::apis::meta::v1::{ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

// =============================================================================
// APIService
// =============================================================================

/// APIService represents a server for a particular GroupVersion.
/// Name must be "version.group".
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIService {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// Spec contains information for locating and communicating with a server
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<APIServiceSpec>,
    /// Status contains derived information about an API server
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<APIServiceStatus>,
}

/// APIServiceList is a list of APIService objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIServiceList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    /// Items is the list of APIService
    pub items: Vec<APIService>,
}

/// APIServiceSpec contains information for locating and communicating with a server.
/// Only https is supported, though you are able to disable certificate verification.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIServiceSpec {
    /// Service is a reference to the service for this API server.
    /// It must communicate on port 443.
    /// If the Service is nil, that means the handling for the API groupversion is handled locally on this server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceReference>,
    /// Group is the API group name this server hosts
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,
    /// Version is the API version this server hosts. For example, "v1"
    pub version: String,
    /// InsecureSkipTLSVerify disables TLS certificate verification when communicating with this server.
    /// This is strongly discouraged. You should use the CABundle instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure_skip_tls_verify: Option<bool>,
    /// CABundle is a PEM encoded CA bundle which will be used to validate an API server's serving certificate.
    /// If unspecified, system trust roots on the apiserver are used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ca_bundle: Option<Vec<u8>>,
    /// GroupPriorityMinimum is the priority this group should have at least.
    /// Higher priority means that the group is preferred by clients over lower priority ones.
    pub group_priority_minimum: i32,
    /// VersionPriority controls the ordering of this API version inside of its group.
    /// Must be greater than zero.
    pub version_priority: i32,
}

/// ServiceReference holds a reference to Service.legacy.k8s.io
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceReference {
    /// Namespace is the namespace of the service
    pub namespace: String,
    /// Name is the name of the service
    pub name: String,
    /// Port is the port on the service that hosting the service.
    /// Default to 443 for backward compatibility.
    /// `port` should be a valid port number (1-65535, inclusive).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

// =============================================================================
// APIServiceStatus
// =============================================================================

/// APIServiceStatus contains derived information about an API server
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIServiceStatus {
    /// Conditions represent the current service state of apiService.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<APIServiceCondition>,
}

/// APIServiceCondition describes conditions for an APIService
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIServiceCondition {
    /// Type is the type of the condition.
    #[serde(rename = "type")]
    pub type_: String,
    /// Status is the status of the condition. Can be True, False, Unknown.
    pub status: String,
    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<k8s_apimachinery::apis::meta::v1::Time>,
    /// Unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// Human-readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

// =============================================================================
// Constants
// =============================================================================

/// Available indicates that the service exists and is reachable
pub const CONDITION_AVAILABLE: &str = "Available";

/// Condition statuses
pub const CONDITION_TRUE: &str = "True";
pub const CONDITION_FALSE: &str = "False";
pub const CONDITION_UNKNOWN: &str = "Unknown";
