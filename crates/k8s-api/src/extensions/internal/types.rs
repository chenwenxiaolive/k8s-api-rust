//! Internal type definitions for extensions.

use k8s_api_core::IntOrString;
use k8s_apimachinery::apis::meta::v1::{LabelSelector, ListMeta, ObjectMeta, Time, TypeMeta};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::core::v1::{ConditionStatus, PodTemplateSpec, Protocol, TypedLocalObjectReference};

pub type DaemonSetConditionType = String;
pub type DaemonSetUpdateStrategyType = String;
pub type DeploymentConditionType = String;
pub type DeploymentStrategyType = String;
pub type PathType = String;
pub type PolicyType = String;
pub type ReplicaSetConditionType = String;

pub const DAEMON_SET_TEMPLATE_GENERATION_KEY: &str = "pod-template-generation";
pub const DAEMON_SET_UPDATE_STRATEGY_TYPE_ON_DELETE: &str = "OnDelete";
pub const DAEMON_SET_UPDATE_STRATEGY_TYPE_ROLLING_UPDATE: &str = "RollingUpdate";
pub const DEFAULT_DAEMON_SET_UNIQUE_LABEL_KEY: &str = "controller-revision-hash";
pub const DEFAULT_DEPLOYMENT_UNIQUE_LABEL_KEY: &str = "pod-template-hash";
pub const DEPLOYMENT_CONDITION_AVAILABLE: &str = "Available";
pub const DEPLOYMENT_CONDITION_PROGRESSING: &str = "Progressing";
pub const DEPLOYMENT_CONDITION_REPLICA_FAILURE: &str = "ReplicaFailure";
pub const DEPLOYMENT_STRATEGY_TYPE_RECREATE: &str = "Recreate";
pub const DEPLOYMENT_STRATEGY_TYPE_ROLLING_UPDATE: &str = "RollingUpdate";
pub const PATH_TYPE_EXACT: &str = "Exact";
pub const PATH_TYPE_IMPLEMENTATION_SPECIFIC: &str = "ImplementationSpecific";
pub const PATH_TYPE_PREFIX: &str = "Prefix";
pub const POLICY_TYPE_EGRESS: &str = "Egress";
pub const POLICY_TYPE_INGRESS: &str = "Ingress";
pub const REPLICA_SET_REPLICA_FAILURE: &str = "ReplicaFailure";


/// DaemonSet represents the configuration of a daemon set.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSet {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<DaemonSetSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DaemonSetStatus>,
}


/// DaemonSetCondition describes the state of a DaemonSet at a certain point.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetCondition {
    #[serde(rename = "type")]
    pub condition_type: DaemonSetConditionType,
    pub status: ConditionStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Time>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub message: String,
}


/// DaemonSetList is a collection of daemon sets.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ListMeta,
    pub items: Vec<DaemonSet>,
}


/// DaemonSetSpec is the specification of a daemon set.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    pub template: PodTemplateSpec,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_strategy: Option<DaemonSetUpdateStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_generation: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_number_scheduled: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_available: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_unavailable: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<DaemonSetCondition>,
}


/// DaemonSetUpdateStrategy indicates how to update DaemonSet pods.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetUpdateStrategy {
    #[serde(skip_serializing_if = "String::is_empty", rename = "type")]
    pub strategy_type: DaemonSetUpdateStrategyType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<RollingUpdateDaemonSet>,
}


/// Deployment enables declarative updates for Pods and ReplicaSets.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deployment {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<DeploymentSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DeploymentStatus>,
}


/// DeploymentCondition describes the state of a deployment at a certain point.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentCondition {
    #[serde(rename = "type")]
    pub condition_type: DeploymentConditionType,
    pub status: ConditionStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<Time>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Time>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub message: String,
}


/// DeploymentList is a list of Deployments.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ListMeta,
    pub items: Vec<Deployment>,
}


/// DeploymentRollback stores the information required to rollback a deployment.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentRollback {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub name: String,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub updated_annotations: BTreeMap<String, String>,
    pub rollback_to: RollbackConfig,
}


/// DeploymentSpec is the specification of the desired behavior of the Deployment.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    pub template: PodTemplateSpec,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<DeploymentStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_to: Option<RollbackConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_deadline_seconds: Option<i32>,
}


/// DeploymentStatus is the most recently observed status of the Deployment.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable_replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminating_replicas: Option<i32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<DeploymentCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i32>,
}


/// DeploymentStrategy describes how to replace existing pods with new ones.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentStrategy {
    #[serde(skip_serializing_if = "String::is_empty", rename = "type")]
    pub strategy_type: DeploymentStrategyType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<RollingUpdateDeployment>,
}


/// HTTPIngressPath associates a path with a backend.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HTTPIngressPath {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_type: Option<PathType>,
    pub backend: IngressBackend,
}


/// HTTPIngressRuleValue is a list of HTTP selectors pointing to backends.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HTTPIngressRuleValue {
    pub paths: Vec<HTTPIngressPath>,
}


/// IPBlock describes a particular CIDR range.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IPBlock {
    pub cidr: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub except: Vec<String>,
}


/// Ingress is a collection of rules that allow inbound connections to reach the endpoints defined by a backend.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ingress {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<IngressSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<IngressStatus>,
}


/// IngressBackend describes all endpoints for a given service and port.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressBackend {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub service_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_port: Option<IntOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<TypedLocalObjectReference>,
}


/// IngressList is a collection of Ingress.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ListMeta,
    pub items: Vec<Ingress>,
}


/// IngressLoadBalancerIngress represents the status of a load-balancer ingress point.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressLoadBalancerIngress {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub ip: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub hostname: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<IngressPortStatus>,
}


/// IngressLoadBalancerStatus represents the status of a load-balancer.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressLoadBalancerStatus {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ingress: Vec<IngressLoadBalancerIngress>,
}


/// IngressPortStatus represents the error condition of a service port.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressPortStatus {
    pub port: i32,
    pub protocol: Protocol,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}


/// IngressRule represents rules mapping the paths under a host to backends.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressRule {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub host: String,
    #[serde(flatten)]
    pub ingress_rule_value: IngressRuleValue,
}


/// IngressRuleValue represents a rule to apply against incoming requests.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressRuleValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HTTPIngressRuleValue>,
}


/// IngressSpec describes the Ingress the user wishes to exist.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_class_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend: Option<IngressBackend>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tls: Vec<IngressTLS>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<IngressRule>,
}


/// IngressStatus describes the current state of the Ingress.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<IngressLoadBalancerStatus>,
}


/// IngressTLS describes the transport layer security associated with an Ingress.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressTLS {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub hosts: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub secret_name: String,
}


/// NetworkPolicy describes what network traffic is allowed for a set of Pods.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicy {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<NetworkPolicySpec>,
}


/// NetworkPolicyEgressRule describes traffic allowed out of pods.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyEgressRule {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<NetworkPolicyPort>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub to: Vec<NetworkPolicyPeer>,
}


/// NetworkPolicyIngressRule matches traffic if ports and from match.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyIngressRule {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<NetworkPolicyPort>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub from: Vec<NetworkPolicyPeer>,
}


/// NetworkPolicyList is a list of NetworkPolicy objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ListMeta,
    pub items: Vec<NetworkPolicy>,
}


/// NetworkPolicyPeer describes a peer to allow traffic to/from.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyPeer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_selector: Option<LabelSelector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<LabelSelector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_block: Option<IPBlock>,
}


/// NetworkPolicyPort describes a port to allow traffic on.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyPort {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<IntOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_port: Option<i32>,
}


/// NetworkPolicySpec describes a network policy.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicySpec {
    pub pod_selector: LabelSelector,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ingress: Vec<NetworkPolicyIngressRule>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub egress: Vec<NetworkPolicyEgressRule>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub policy_types: Vec<PolicyType>,
}


/// ReplicaSet ensures that a specified number of pod replicas are running at any given time.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaSet {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<ReplicaSetSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ReplicaSetStatus>,
}


/// ReplicaSetCondition describes the state of a replica set at a certain point.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaSetCondition {
    #[serde(rename = "type")]
    pub condition_type: ReplicaSetConditionType,
    pub status: ConditionStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Time>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub message: String,
}


/// ReplicaSetList is a collection of ReplicaSets.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaSetList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ListMeta,
    pub items: Vec<ReplicaSet>,
}


/// ReplicaSetSpec is the specification of a ReplicaSet.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaSetSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<PodTemplateSpec>,
}


/// ReplicaSetStatus represents the current status of a ReplicaSet.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaSetStatus {
    pub replicas: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fully_labeled_replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminating_replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<ReplicaSetCondition>,
}


/// RollbackConfig contains the rollback configuration.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RollbackConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}


/// RollingUpdateDaemonSet specifies parameters for rolling update of a DaemonSet.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RollingUpdateDaemonSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<IntOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_surge: Option<IntOrString>,
}


/// RollingUpdateDeployment specifies parameters for a rolling update.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RollingUpdateDeployment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<IntOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_surge: Option<IntOrString>,
}


/// Scale represents a scaling request for a resource.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scale {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<ScaleSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ScaleStatus>,
}


/// ScaleSpec describes the attributes of a scale subresource.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScaleSpec {
    /// Desired number of instances for the scaled object.
    pub replicas: i32,
}


/// ScaleStatus represents the current status of a scale subresource.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScaleStatus {
    /// Actual number of observed instances of the scaled object.
    pub replicas: i32,
    /// Selector is a label query over pods that should match the replicas count.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub selector: BTreeMap<String, String>,
    /// TargetSelector is a serialized label selector string.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub target_selector: String,
}
