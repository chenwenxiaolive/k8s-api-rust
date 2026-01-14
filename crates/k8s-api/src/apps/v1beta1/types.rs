//! Apps v1beta1 API type definitions (deprecated)
//!
//! This module provides deprecated beta types for backwards compatibility.

use k8s_apimachinery::apis::meta::v1::{LabelSelector, ListMeta, ObjectMeta, Time, TypeMeta};
use serde::{Deserialize, Serialize};

use crate::core::v1::PodTemplateSpec;

// =============================================================================
// Deployment
// =============================================================================

/// Deployment enables declarative updates for Pods and ReplicaSets.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deployment {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<DeploymentSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DeploymentStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    pub template: PodTemplateSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<DeploymentStrategy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_to: Option<RollbackConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_deadline_seconds: Option<i32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RollbackConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentStrategy {
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "type")]
    pub strategy_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<RollingUpdateDeployment>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RollingUpdateDeployment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<k8s_api_core::IntOrString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_surge: Option<k8s_api_core::IntOrString>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_replicas: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_replicas: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_replicas: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<DeploymentCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i32>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentCondition {
    #[serde(rename = "type")]
    pub condition_type: String,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Time>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    pub items: Vec<Deployment>,
}

// =============================================================================
// StatefulSet
// =============================================================================

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSet {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<StatefulSetSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StatefulSetStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    pub template: PodTemplateSpec,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volume_claim_templates: Vec<crate::core::v1::PersistentVolumeClaim>,
    pub service_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub pod_management_policy: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_strategy: Option<StatefulSetUpdateStrategy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: Option<i32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetUpdateStrategy {
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "type")]
    pub strategy_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<RollingUpdateStatefulSetStrategy>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RollingUpdateStatefulSetStrategy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partition: Option<i32>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    #[serde(default)]
    pub replicas: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_replicas: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_replicas: Option<i32>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub current_revision: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub update_revision: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<StatefulSetCondition>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetCondition {
    #[serde(rename = "type")]
    pub condition_type: String,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Time>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    pub items: Vec<StatefulSet>,
}

// =============================================================================
// ControllerRevision
// =============================================================================

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerRevision {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    pub revision: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerRevisionList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    pub items: Vec<ControllerRevision>,
}

// =============================================================================
// Scale (apps/v1beta1 specific)
// =============================================================================

/// Scale represents a scaling request for a resource.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scale {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ScaleSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ScaleStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScaleSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScaleStatus {
    pub replicas: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_selector: Option<String>,
}
