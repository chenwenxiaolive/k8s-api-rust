//! Node v1beta1 API type definitions (deprecated)

use k8s_apimachinery::apis::meta::v1::{ListMeta, ObjectMeta, TypeMeta};
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
    /// Handler specifies the underlying runtime and configuration.
    pub handler: String,
    /// Overhead represents the resource overhead associated with running a pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overhead: Option<Overhead>,
    /// Scheduling holds the scheduling constraints.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduling: Option<Scheduling>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Overhead {
    /// PodFixed represents the fixed resource overhead associated with running a pod.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub pod_fixed: std::collections::BTreeMap<String, k8s_api_core::Quantity>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scheduling {
    /// nodeSelector lists labels that must be present on nodes.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub node_selector: std::collections::BTreeMap<String, String>,
    /// tolerations are appended to pods running with this RuntimeClass.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tolerations: Vec<crate::core::v1::Toleration>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeClassList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    pub items: Vec<RuntimeClass>,
}
