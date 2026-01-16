//! Policy v1beta1 API type definitions (deprecated)
//!
//! This module provides deprecated beta types for backwards compatibility.

use k8s_apimachinery::apis::meta::v1::{LabelSelector, ListMeta, ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

pub type UnhealthyPodEvictionPolicyType = String;

// =============================================================================
// PodDisruptionBudget
// =============================================================================

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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodDisruptionBudgetSpec {
    /// An eviction is allowed if at least "minAvailable" pods selected by "selector"
    /// will still be available after the eviction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_available: Option<k8s_api_core::IntOrString>,
    /// Label query over pods whose evictions are managed by the disruption budget.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    /// An eviction is allowed if at most "maxUnavailable" pods selected by "selector"
    /// are unavailable after the eviction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<k8s_api_core::IntOrString>,
    /// UnhealthyPodEvictionPolicy defines the criteria for when unhealthy pods should be considered for eviction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_pod_eviction_policy: Option<UnhealthyPodEvictionPolicyType>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodDisruptionBudgetStatus {
    /// Most recent generation observed when updating this PDB status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// DisruptedPods contains information about pods whose eviction was processed.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub disrupted_pods: std::collections::BTreeMap<String, k8s_apimachinery::apis::meta::v1::Time>,
    /// Number of pod disruptions that are currently allowed.
    pub disruptions_allowed: i32,
    /// Current number of healthy pods.
    pub current_healthy: i32,
    /// Minimum desired number of healthy pods.
    pub desired_healthy: i32,
    /// Total number of pods counted by this disruption budget.
    pub expected_pods: i32,
    /// Conditions contain conditions for PDB.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<k8s_apimachinery::apis::meta::v1::Condition>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodDisruptionBudgetList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    pub items: Vec<PodDisruptionBudget>,
}

// UnhealthyPodEvictionPolicyType constants
pub const UNHEALTHY_POD_EVICTION_POLICY_IF_HEALTHY_BUDGET: &str = "IfHealthyBudget";
pub const UNHEALTHY_POD_EVICTION_POLICY_ALWAYS_ALLOW: &str = "AlwaysAllow";

// =============================================================================
// Eviction
// =============================================================================

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
