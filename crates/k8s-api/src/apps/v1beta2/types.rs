//! Apps v1beta2 API type definitions (deprecated)

use k8s_apimachinery::apis::meta::v1::{LabelSelector, ObjectMeta, Time, TypeMeta};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::core::v1::PodTemplateSpec;

pub const CONTROLLER_REVISION_HASH_LABEL_KEY: &str = "controller-revision-hash";
pub const STATEFUL_SET_REVISION_LABEL: &str = CONTROLLER_REVISION_HASH_LABEL_KEY;
pub const DEPRECATED_ROLLBACK_TO: &str = "deprecated.deployment.rollback.to";
pub const DEPRECATED_TEMPLATE_GENERATION: &str = "deprecated.daemonset.template.generation";
pub const STATEFUL_SET_POD_NAME_LABEL: &str = "statefulset.kubernetes.io/pod-name";
pub const DEFAULT_DEPLOYMENT_UNIQUE_LABEL_KEY: &str = "pod-template-hash";

// =============================================================================
// Scale
// =============================================================================

/// ScaleSpec describes the attributes of a scale subresource.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScaleSpec {
    /// Desired number of instances for the scaled object.
    #[serde(default)]
    pub replicas: i32,
}

/// ScaleStatus represents the current status of a scale subresource.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScaleStatus {
    /// Actual number of observed instances of the scaled object.
    pub replicas: i32,
    /// Selector is a label query over pods that should match the replicas count.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub selector: BTreeMap<String, String>,
    /// TargetSelector is a serialized label selector string.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub target_selector: String,
}

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

/// DeploymentSpec is the specification of the desired behavior of the Deployment.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentSpec {
    /// Number of desired pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,

    /// Label selector for pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,

    /// Template describes the pods that will be created.
    pub template: PodTemplateSpec,

    /// The deployment strategy to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<DeploymentStrategy>,

    /// Minimum number of seconds for which a newly created pod should be ready.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,

    /// The number of old ReplicaSets to retain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: Option<i32>,

    /// Indicates that the deployment is paused.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,

    /// The maximum time in seconds for a deployment to make progress.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_deadline_seconds: Option<i32>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentStrategy {
    /// Type of deployment.
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "type")]
    pub strategy_type: String,

    /// Rolling update config params.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<RollingUpdateDeployment>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RollingUpdateDeployment {
    /// The maximum number of pods that can be unavailable during the update.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<k8s_api_core::IntOrString>,

    /// The maximum number of pods that can be scheduled above the desired number of pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_surge: Option<k8s_api_core::IntOrString>,
}

/// DeploymentStatus is the most recently observed status of the Deployment.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentStatus {
    /// The generation observed by the deployment controller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,

    /// Total number of non-terminated pods targeted by this deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,

    /// Total number of non-terminated pods targeted by this deployment that have the desired template spec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_replicas: Option<i32>,

    /// Total number of ready pods targeted by this deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i32>,

    /// Total number of available pods targeted by this deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_replicas: Option<i32>,

    /// Total number of unavailable pods targeted by this deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_replicas: Option<i32>,

    /// Total number of terminating pods targeted by this deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminating_replicas: Option<i32>,

    /// Represents the latest available observations of a deployment's current state.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<DeploymentCondition>,

    /// Count of hash collisions for the Deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i32>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentCondition {
    /// Type of deployment condition.
    #[serde(rename = "type")]
    pub condition_type: String,

    /// Status of the condition.
    pub status: String,

    /// The last time this condition was updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<Time>,

    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Time>,

    /// The reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,

    /// A human readable message indicating details about the transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

/// DeploymentList is a list of Deployments.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,

    pub items: Vec<Deployment>,
}

// =============================================================================
// StatefulSet
// =============================================================================

/// StatefulSet represents a set of pods with consistent identities.
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

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persistent_volume_claim_retention_policy: Option<StatefulSetPersistentVolumeClaimRetentionPolicy>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ordinals: Option<StatefulSetOrdinals>,
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

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<k8s_api_core::IntOrString>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetPersistentVolumeClaimRetentionPolicy {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub when_deleted: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub when_scaled: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetOrdinals {
    #[serde(default)]
    pub start: i32,
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

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_replicas: Option<i32>,
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

/// StatefulSetList is a collection of StatefulSets.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,

    pub items: Vec<StatefulSet>,
}

// =============================================================================
// DaemonSet
// =============================================================================

/// DaemonSet represents the configuration of a daemon set.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSet {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: ObjectMeta,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<DaemonSetSpec>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DaemonSetStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,

    pub template: PodTemplateSpec,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_strategy: Option<DaemonSetUpdateStrategy>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: Option<i32>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetUpdateStrategy {
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "type")]
    pub strategy_type: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<RollingUpdateDaemonSet>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RollingUpdateDaemonSet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<k8s_api_core::IntOrString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_surge: Option<k8s_api_core::IntOrString>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetStatus {
    pub current_number_scheduled: i32,
    pub number_misscheduled: i32,
    pub desired_number_scheduled: i32,
    pub number_ready: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_number_scheduled: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_available: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_unavailable: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<DaemonSetCondition>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetCondition {
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

/// DaemonSetList is a collection of daemon sets.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,

    pub items: Vec<DaemonSet>,
}

// =============================================================================
// ReplicaSet
// =============================================================================

/// ReplicaSet ensures that a specified number of pod replicas are running.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaSet {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: ObjectMeta,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ReplicaSetSpec>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReplicaSetStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaSetSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<PodTemplateSpec>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaSetStatus {
    pub replicas: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fully_labeled_replicas: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_replicas: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminating_replicas: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<ReplicaSetCondition>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaSetCondition {
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

/// ReplicaSetList is a collection of ReplicaSets.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaSetList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,

    pub items: Vec<ReplicaSet>,
}

// =============================================================================
// ControllerRevision
// =============================================================================

/// ControllerRevision implements an immutable snapshot of state data.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerRevision {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: ObjectMeta,

    /// Data is the serialized representation of the state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,

    /// Revision indicates the revision of the state represented by Data.
    pub revision: i64,
}

/// ControllerRevisionList is a resource containing a list of ControllerRevision objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerRevisionList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,

    pub items: Vec<ControllerRevision>,
}

// =============================================================================
// Tests
// =============================================================================
