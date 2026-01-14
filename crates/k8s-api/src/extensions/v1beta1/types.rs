//! Extensions v1beta1 API type definitions (deprecated)

use k8s_api_core::IntOrString;
use k8s_apimachinery::apis::meta::v1::{LabelSelector, ListMeta, ObjectMeta, Time, TypeMeta};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::core::v1::{PodTemplateSpec, TypedLocalObjectReference};

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

/// DeploymentRollback stores the information required to rollback a deployment.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentRollback {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub name: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub updated_annotations: BTreeMap<String, String>,
    pub rollback_to: RollbackConfig,
}

/// RollbackConfig contains the rollback configuration.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RollbackConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}

pub const DEFAULT_DEPLOYMENT_UNIQUE_LABEL_KEY: &str = "pod-template-hash";

/// DeploymentStrategy describes how to replace existing pods with new ones.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentStrategy {
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "type")]
    pub strategy_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<RollingUpdateDeployment>,
}

/// RollingUpdateDeployment specifies parameters for a rolling update.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RollingUpdateDeployment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<IntOrString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_surge: Option<IntOrString>,
}

/// DeploymentStatus is the most recently observed status of the Deployment.
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminating_replicas: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<DeploymentCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i32>,
}

/// DeploymentCondition describes the state of a deployment at a certain point.
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

/// DeploymentList is a list of Deployments.
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
// DaemonSet
// =============================================================================

/// DaemonSetUpdateStrategy indicates how to update DaemonSet pods.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetUpdateStrategy {
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "type")]
    pub strategy_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<RollingUpdateDaemonSet>,
}

/// RollingUpdateDaemonSet specifies parameters for rolling update of a DaemonSet.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RollingUpdateDaemonSet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<IntOrString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_surge: Option<IntOrString>,
}

/// DaemonSetSpec is the specification of a daemon set.
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
    pub template_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: Option<i32>,
}

/// DaemonSetStatus represents the current status of a daemon set.
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

/// DaemonSetCondition describes the state of a DaemonSet at a certain point.
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

pub const DAEMON_SET_TEMPLATE_GENERATION_KEY: &str = "pod-template-generation";
pub const DEFAULT_DAEMON_SET_UNIQUE_LABEL_KEY: &str = "controller-revision-hash";

/// DaemonSetList is a collection of daemon sets.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    pub items: Vec<DaemonSet>,
}

// =============================================================================
// Ingress
// =============================================================================

/// Ingress is a collection of rules that allow inbound connections to reach the endpoints defined by a backend.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ingress {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<IngressSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<IngressStatus>,
}

/// IngressList is a collection of Ingress.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    pub items: Vec<Ingress>,
}

/// IngressSpec describes the Ingress the user wishes to exist.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress_class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend: Option<IngressBackend>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tls: Vec<IngressTLS>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<IngressRule>,
}

/// IngressTLS describes the transport layer security associated with an Ingress.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressTLS {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hosts: Vec<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub secret_name: String,
}

/// IngressStatus describes the current state of the Ingress.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<IngressLoadBalancerStatus>,
}

/// IngressLoadBalancerStatus represents the status of a load-balancer.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressLoadBalancerStatus {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingress: Vec<IngressLoadBalancerIngress>,
}

/// IngressLoadBalancerIngress represents the status of a load-balancer ingress point.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressLoadBalancerIngress {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ip: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hostname: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<IngressPortStatus>,
}

/// IngressPortStatus represents the error condition of a service port.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressPortStatus {
    pub port: i32,
    pub protocol: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// IngressRule represents rules mapping the paths under a host to backends.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressRule {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host: String,
    #[serde(default, flatten)]
    pub ingress_rule_value: IngressRuleValue,
}

/// IngressRuleValue represents a rule to apply against incoming requests.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressRuleValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<HTTPIngressRuleValue>,
}

/// HTTPIngressRuleValue is a list of HTTP selectors pointing to backends.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HTTPIngressRuleValue {
    pub paths: Vec<HTTPIngressPath>,
}

/// HTTPIngressPath associates a path with a backend.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HTTPIngressPath {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_type: Option<String>,
    pub backend: IngressBackend,
}

/// IngressBackend describes all endpoints for a given service and port.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressBackend {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub service_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_port: Option<IntOrString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<TypedLocalObjectReference>,
}

pub const PATH_TYPE_EXACT: &str = "Exact";
pub const PATH_TYPE_PREFIX: &str = "Prefix";
pub const PATH_TYPE_IMPLEMENTATION_SPECIFIC: &str = "ImplementationSpecific";

// =============================================================================
// ReplicaSet
// =============================================================================

/// ReplicaSet ensures that a specified number of pod replicas are running at any given time.
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

/// ReplicaSetList is a collection of ReplicaSets.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaSetList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    pub items: Vec<ReplicaSet>,
}

/// ReplicaSetSpec is the specification of a ReplicaSet.
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

/// ReplicaSetStatus represents the current status of a ReplicaSet.
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

/// ReplicaSetCondition describes the state of a replica set at a certain point.
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

pub const REPLICA_SET_REPLICA_FAILURE: &str = "ReplicaFailure";

// =============================================================================
// NetworkPolicy
// =============================================================================

/// NetworkPolicy describes what network traffic is allowed for a set of Pods.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicy {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<NetworkPolicySpec>,
}

/// NetworkPolicySpec describes a network policy.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicySpec {
    pub pod_selector: LabelSelector,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingress: Vec<NetworkPolicyIngressRule>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub egress: Vec<NetworkPolicyEgressRule>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_types: Vec<String>,
}

/// NetworkPolicyIngressRule matches traffic if ports and from match.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyIngressRule {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<NetworkPolicyPort>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub from: Vec<NetworkPolicyPeer>,
}

/// NetworkPolicyEgressRule describes traffic allowed out of pods.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyEgressRule {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<NetworkPolicyPort>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub to: Vec<NetworkPolicyPeer>,
}

/// NetworkPolicyPort describes a port to allow traffic on.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyPort {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<IntOrString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_port: Option<i32>,
}

/// IPBlock describes a particular CIDR range.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IPBlock {
    pub cidr: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub except: Vec<String>,
}

/// NetworkPolicyPeer describes a peer to allow traffic to/from.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyPeer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_selector: Option<LabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<LabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_block: Option<IPBlock>,
}

/// NetworkPolicyList is a list of NetworkPolicy objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    pub items: Vec<NetworkPolicy>,
}
