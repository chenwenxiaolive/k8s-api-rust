//! Node v1 API type definitions

use k8s_apimachinery::apis::meta::v1::{ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

// =============================================================================
// RuntimeClass
// =============================================================================

/// RuntimeClass defines a class of container runtime supported in the cluster.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeClass {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// Handler specifies the underlying runtime and configuration that the CRI implementation will use.
    pub handler: String,
    /// Overhead represents the resource overhead associated with running a pod for a given RuntimeClass.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overhead: Option<Overhead>,
    /// Scheduling holds the scheduling constraints to ensure that pods running with this RuntimeClass are scheduled to nodes that support it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduling: Option<Scheduling>,
}

/// RuntimeClassList is a list of RuntimeClass objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeClassList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<RuntimeClass>,
}

/// Overhead structure represents the resource overhead associated with running a pod.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Overhead {
    /// PodFixed represents the fixed resource overhead associated with running a pod.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub pod_fixed: std::collections::BTreeMap<String, String>,
}

/// Scheduling specifies the scheduling constraints for nodes supporting a RuntimeClass.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scheduling {
    /// NodeSelector lists labels that must be present on nodes that support this RuntimeClass.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub node_selector: std::collections::BTreeMap<String, String>,
    /// Tolerations are appended to pods running with this RuntimeClass.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tolerations: Vec<Toleration>,
}

/// Toleration represents a toleration for a pod.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Toleration {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub key: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub operator: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub effect: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub toleration_seconds: Option<i64>,
}
