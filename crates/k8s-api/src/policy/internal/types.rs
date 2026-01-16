//! Internal type definitions for policy.

use k8s_apimachinery::apis::meta::v1::{LabelSelector, ObjectMeta, TypeMeta, Condition};
use serde::{Deserialize, Serialize};

pub type UnhealthyPodEvictionPolicyType = String;

pub const UNHEALTHY_POD_EVICTION_POLICY_ALWAYS_ALLOW: &str = "AlwaysAllow";
pub const UNHEALTHY_POD_EVICTION_POLICY_IF_HEALTHY_BUDGET: &str = "IfHealthyBudget";


/// Eviction evicts a pod from its node subject to certain policies and safety constraints.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Eviction {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// DeleteOptions may be provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_options: Option<serde_json::Value>,
}


/// PodDisruptionBudget is an object to define the max disruption that can be caused to a collection of pods.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodDisruptionBudget {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PodDisruptionBudgetSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PodDisruptionBudgetStatus>,
}


/// PodDisruptionBudgetList is a collection of PodDisruptionBudgets.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodDisruptionBudgetList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<PodDisruptionBudget>,
}


/// PodDisruptionBudgetSpec is a description of a PodDisruptionBudget.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodDisruptionBudgetSpec {
    /// MinAvailable is an eviction is allowed if at least "minAvailable" pods selected by "selector" will still be available after the eviction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_available: Option<serde_json::Value>,
    /// MaxUnavailable is an eviction is allowed if at most "maxUnavailable" pods selected by "selector" are unavailable after the eviction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<serde_json::Value>,
    /// Selector is a label query over a set of resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    /// UnhealthyPodEvictionPolicy defines the criteria for when unhealthy pods should be considered for eviction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_pod_eviction_policy: Option<UnhealthyPodEvictionPolicyType>,
}


/// PodDisruptionBudgetStatus represents information about the status of a PodDisruptionBudget.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodDisruptionBudgetStatus {
    /// ObservedGeneration is the most recent generation observed for this PodDisruptionBudget.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// DisruptedPods contains information about pods whose eviction was processed by the API server.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub disrupted_pods: std::collections::BTreeMap<String, String>,
    /// DisruptionsAllowed is the number of pod disruptions that are currently allowed.
    pub disruptions_allowed: i32,
    /// CurrentHealthy is current number of healthy pods.
    pub current_healthy: i32,
    /// DesiredHealthy is minimum desired number of healthy pods.
    pub desired_healthy: i32,
    /// ExpectedPods is total number of pods counted by this disruption budget.
    pub expected_pods: i32,
    /// Conditions contain conditions for PDB.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,
}
