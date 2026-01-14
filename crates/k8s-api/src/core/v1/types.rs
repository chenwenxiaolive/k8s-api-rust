//! Core v1 API type definitions
//!
//! This module contains the Rust definitions for Kubernetes core/v1 API types.

use k8s_api_core::resource::{IntOrString, Quantity};
use k8s_apimachinery::apis::meta::v1::{LabelSelector, ObjectMeta, Time, TypeMeta};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

// =============================================================================
// Constants
// =============================================================================

/// NamespaceDefault is the default namespace
pub const NAMESPACE_DEFAULT: &str = "default";
/// NamespaceSystem is the system namespace
pub const NAMESPACE_SYSTEM: &str = "kube-system";
/// NamespacePublic is the public namespace
pub const NAMESPACE_PUBLIC: &str = "kube-public";
/// NamespaceNodeLease is the node lease namespace
pub const NAMESPACE_NODE_LEASE: &str = "kube-node-lease";

// =============================================================================
// Pod
// =============================================================================

/// Pod is a collection of containers that can run on a host.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pod {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    #[serde(default)]
    pub metadata: ObjectMeta,

    /// Specification of the desired behavior of the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PodSpec>,

    /// Most recently observed status of the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PodStatus>,
}

impl Pod {
    pub const KIND: &'static str = "Pod";
    pub const API_VERSION: &'static str = "v1";

    pub fn new(name: impl Into<String>) -> Self {
        Self {
            type_meta: TypeMeta::new(Self::API_VERSION, Self::KIND),
            metadata: ObjectMeta::named(name),
            ..Default::default()
        }
    }
}

/// PodList is a list of Pods.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,

    pub items: Vec<Pod>,
}

/// PodSpec is a description of a pod.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodSpec {
    /// List of containers belonging to the pod.
    #[serde(default)]
    pub containers: Vec<Container>,

    /// List of initialization containers belonging to the pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub init_containers: Vec<Container>,

    /// List of ephemeral containers run in this pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ephemeral_containers: Vec<EphemeralContainer>,

    /// List of volumes that can be mounted by containers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes: Vec<Volume>,

    /// Restart policy for all containers. One of Always, OnFailure, Never.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub restart_policy: String,

    /// Optional duration in seconds for graceful termination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termination_grace_period_seconds: Option<i64>,

    /// Optional duration in seconds the pod may be active before termination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_deadline_seconds: Option<i64>,

    /// Set DNS policy for the pod.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub dns_policy: String,

    /// NodeSelector is a selector which must be true for the pod to fit on a node.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub node_selector: BTreeMap<String, String>,

    /// ServiceAccountName is the name of the ServiceAccount to use.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub service_account_name: String,

    /// Deprecated: Use ServiceAccountName instead.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub service_account: String,

    /// AutomountServiceAccountToken indicates whether to mount the service account token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automount_service_account_token: Option<bool>,

    /// NodeName is a request to schedule this pod onto a specific node.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub node_name: String,

    /// Host networking requested for this pod.
    #[serde(default, skip_serializing_if = "is_false")]
    pub host_network: bool,

    /// Use the host's pid namespace.
    #[serde(default, skip_serializing_if = "is_false")]
    pub host_p_i_d: bool,

    /// Use the host's ipc namespace.
    #[serde(default, skip_serializing_if = "is_false")]
    pub host_i_p_c: bool,

    /// SecurityContext holds pod-level security attributes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_context: Option<PodSecurityContext>,

    /// ImagePullSecrets is a list of references to secrets for pulling images.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image_pull_secrets: Vec<LocalObjectReference>,

    /// Hostname for the pod.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hostname: String,

    /// Subdomain for the pod.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub subdomain: String,

    /// Affinity scheduling rules for the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affinity: Option<Affinity>,

    /// SchedulerName is the name of the scheduler.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub scheduler_name: String,

    /// Tolerations are attached to the pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tolerations: Vec<Toleration>,

    /// HostAliases is an optional list of hosts and IPs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub host_aliases: Vec<HostAlias>,

    /// PriorityClassName is the pod's priority class.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub priority_class_name: String,

    /// Priority value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,

    /// DNS config for the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<PodDNSConfig>,

    /// ReadinessGates specifies additional readiness gates.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub readiness_gates: Vec<PodReadinessGate>,

    /// RuntimeClassName refers to a RuntimeClass object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_class_name: Option<String>,

    /// EnableServiceLinks indicates whether service information should be injected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_service_links: Option<bool>,

    /// PreemptionPolicy is the Policy for preempting pods with lower priority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preemption_policy: Option<String>,

    /// Overhead represents the resource overhead.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub overhead: BTreeMap<String, Quantity>,

    /// TopologySpreadConstraints describes how pods should spread across topology domains.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topology_spread_constraints: Vec<TopologySpreadConstraint>,

    /// OS specifies the target OS for the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<PodOS>,
}

fn is_false(b: &bool) -> bool {
    !*b
}

/// PodStatus represents information about the status of a pod.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodStatus {
    /// Current condition of the pod.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub phase: String,

    /// Current service state of pod conditions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<PodCondition>,

    /// A human readable message indicating details about the status.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,

    /// A brief CamelCase message indicating details about the status.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,

    /// nominatedNodeName is set only when this pod preempts other pods.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub nominated_node_name: String,

    /// IP address of the host to which the pod is assigned.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host_i_p: String,

    /// IP address allocated to the pod.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub pod_i_p: String,

    /// podIPs holds the IP addresses allocated to the pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pod_i_ps: Vec<PodIP>,

    /// RFC 3339 date and time at which the object was acknowledged by the Kubelet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Time>,

    /// Status of init containers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub init_container_statuses: Vec<ContainerStatus>,

    /// Status of containers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub container_statuses: Vec<ContainerStatus>,

    /// Status of ephemeral containers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ephemeral_container_statuses: Vec<ContainerStatus>,

    /// The Quality of Service (QOS) classification assigned to the pod.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub qos_class: String,
}

/// PodCondition contains details for the current condition of this pod.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodCondition {
    /// Type is the type of the condition.
    #[serde(rename = "type")]
    pub condition_type: String,

    /// Status is the status of the condition.
    pub status: String,

    /// Last time we probed the condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_probe_time: Option<Time>,

    /// Last time the condition transitioned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Time>,

    /// Unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,

    /// Human-readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

/// PodIP represents a single IP address allocated to the pod.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodIP {
    /// IP is the IP address assigned to the pod.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ip: String,
}

/// PodTemplate describes a template for creating copies of a predefined pod.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplate {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: ObjectMeta,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<PodTemplateSpec>,
}

impl PodTemplate {
    pub const KIND: &'static str = "PodTemplate";
    pub const API_VERSION: &'static str = "v1";
}

/// PodTemplateSpec describes the data a pod should have when created from a template.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplateSpec {
    /// Standard object's metadata.
    #[serde(default)]
    pub metadata: ObjectMeta,

    /// Specification of the desired behavior of the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PodSpec>,
}

/// PodTemplateList is a list of PodTemplates.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplateList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,

    pub items: Vec<PodTemplate>,
}

// =============================================================================
// Container
// =============================================================================

/// Container represents a single container in a pod.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Container {
    /// Name of the container.
    pub name: String,

    /// Container image name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub image: String,

    /// Entrypoint array.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,

    /// Arguments to the entrypoint.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,

    /// Container's working directory.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub working_dir: String,

    /// List of ports to expose from the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<ContainerPort>,

    /// List of sources to populate environment variables.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env_from: Vec<EnvFromSource>,

    /// List of environment variables to set.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<EnvVar>,

    /// Compute Resources required by this container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceRequirements>,

    /// Pod volumes to mount into the container's filesystem.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volume_mounts: Vec<VolumeMount>,

    /// volumeDevices is the list of block devices to be used by the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volume_devices: Vec<VolumeDevice>,

    /// Periodic probe of container liveness.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub liveness_probe: Option<Probe>,

    /// Periodic probe of container service readiness.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readiness_probe: Option<Probe>,

    /// StartupProbe indicates that the Pod has successfully initialized.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub startup_probe: Option<Probe>,

    /// Lifecycle callbacks for container lifecycle events.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,

    /// Message describing the current termination message path.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub termination_message_path: String,

    /// Policy for the termination message.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub termination_message_policy: String,

    /// Image pull policy.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub image_pull_policy: String,

    /// SecurityContext holds security configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_context: Option<SecurityContext>,

    /// Whether this container should allocate a buffer for stdin.
    #[serde(default, skip_serializing_if = "is_false")]
    pub stdin: bool,

    /// Whether the container runtime should close the stdin channel.
    #[serde(default, skip_serializing_if = "is_false")]
    pub stdin_once: bool,

    /// Whether this container should allocate a TTY.
    #[serde(default, skip_serializing_if = "is_false")]
    pub tty: bool,
}

impl Container {
    pub fn new(name: impl Into<String>, image: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            image: image.into(),
            ..Default::default()
        }
    }
}

/// EphemeralContainer is a temporary container that may be added to an existing pod.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EphemeralContainer {
    /// Name of the ephemeral container.
    pub name: String,

    /// Container image name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub image: String,

    /// Entrypoint array.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,

    /// Arguments to the entrypoint.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,

    /// Target container name for this ephemeral container.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub target_container_name: String,

    /// List of environment variables to set.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<EnvVar>,

    /// Pod volumes to mount into the container's filesystem.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volume_mounts: Vec<VolumeMount>,

    /// Whether this container should allocate a TTY.
    #[serde(default, skip_serializing_if = "is_false")]
    pub tty: bool,

    /// Whether this container should allocate a buffer for stdin.
    #[serde(default, skip_serializing_if = "is_false")]
    pub stdin: bool,
}

/// ContainerPort represents a network port in a container.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerPort {
    /// Name for this port.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// Number of port to expose on the host.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_port: Option<i32>,

    /// Number of port to expose on the pod's IP address.
    pub container_port: i32,

    /// Protocol for port. Must be UDP, TCP, or SCTP.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub protocol: String,

    /// What host IP to bind the external port to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host_i_p: String,
}

/// ContainerStatus contains details for the current status of this container.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerStatus {
    /// Name of the container.
    pub name: String,

    /// Details about the container's current condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ContainerState>,

    /// Details about the container's last termination condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_state: Option<ContainerState>,

    /// Specifies whether the container has passed its readiness probe.
    #[serde(default)]
    pub ready: bool,

    /// The number of times the container has been restarted.
    #[serde(default)]
    pub restart_count: i32,

    /// The image the container is running.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub image: String,

    /// ImageID of the container's image.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub image_i_d: String,

    /// Container's ID in the format '<type>://<container_id>'.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub container_i_d: String,

    /// Specifies whether the container has passed its startup probe.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started: Option<bool>,
}

/// ContainerState holds a possible state of container.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerState {
    /// Details about a waiting container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiting: Option<ContainerStateWaiting>,

    /// Details about a running container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub running: Option<ContainerStateRunning>,

    /// Details about a terminated container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminated: Option<ContainerStateTerminated>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerStateWaiting {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerStateRunning {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<Time>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerStateTerminated {
    pub exit_code: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signal: Option<i32>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<Time>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub container_i_d: String,
}

// =============================================================================
// Environment Variables
// =============================================================================

/// EnvVar represents an environment variable present in a Container.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvVar {
    /// Name of the environment variable.
    pub name: String,

    /// Value of the environment variable.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,

    /// Source for the environment variable's value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_from: Option<EnvVarSource>,
}

impl EnvVar {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            value_from: None,
        }
    }
}

/// EnvVarSource represents a source for the value of an EnvVar.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvVarSource {
    /// Selects a field of the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_ref: Option<ObjectFieldSelector>,

    /// Selects a resource of the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_field_ref: Option<ResourceFieldSelector>,

    /// Selects a key of a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_map_key_ref: Option<ConfigMapKeySelector>,

    /// Selects a key of a secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_key_ref: Option<SecretKeySelector>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectFieldSelector {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_version: String,
    pub field_path: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceFieldSelector {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub container_name: String,
    pub resource: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<Quantity>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapKeySelector {
    /// Name of the referent.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// The key to select.
    pub key: String,
    /// Specify whether the ConfigMap or its key must be defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretKeySelector {
    /// Name of the referent.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// The key of the secret to select from.
    pub key: String,
    /// Specify whether the Secret or its key must be defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// EnvFromSource represents the source of a set of ConfigMaps.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvFromSource {
    /// An optional identifier to prepend to each key in the ConfigMap.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub prefix: String,

    /// The ConfigMap to select from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_map_ref: Option<ConfigMapEnvSource>,

    /// The Secret to select from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<SecretEnvSource>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapEnvSource {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretEnvSource {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

// =============================================================================
// Resource Requirements
// =============================================================================

/// ResourceRequirements describes the compute resource requirements.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRequirements {
    /// Limits describes the maximum amount of compute resources allowed.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub limits: BTreeMap<String, Quantity>,

    /// Requests describes the minimum amount of compute resources required.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub requests: BTreeMap<String, Quantity>,

    /// Claims lists the names of resources that are used by this container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub claims: Vec<ResourceClaim>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaim {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
}

// =============================================================================
// Volume
// =============================================================================

/// Volume represents a named volume in a pod.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    /// Volume's name.
    pub name: String,

    /// VolumeSource represents the location and type of the mounted volume.
    #[serde(flatten)]
    pub volume_source: VolumeSource,
}

/// VolumeSource represents the source of a volume to mount.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeSource {
    /// HostPath represents a pre-existing file or directory on the host machine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_path: Option<HostPathVolumeSource>,

    /// EmptyDir represents a temporary directory that shares a pod's lifetime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub empty_dir: Option<EmptyDirVolumeSource>,

    /// Secret represents a secret that should populate this volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<SecretVolumeSource>,

    /// ConfigMap represents a configMap that should populate this volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_map: Option<ConfigMapVolumeSource>,

    /// PersistentVolumeClaim represents a reference to a PersistentVolumeClaim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persistent_volume_claim: Option<PersistentVolumeClaimVolumeSource>,

    /// NFS represents an NFS mount on the host that shares a pod's lifetime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nfs: Option<NFSVolumeSource>,

    /// Projected items for all in one resources secrets, configmaps, and downward API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projected: Option<ProjectedVolumeSource>,

    /// DownwardAPI represents downward API about the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downward_a_p_i: Option<DownwardAPIVolumeSource>,

    /// CSI represents ephemeral storage handled by CSI drivers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csi: Option<CSIVolumeSource>,

    /// GCEPersistentDisk represents a GCE Disk resource (deprecated).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gce_persistent_disk: Option<GCEPersistentDiskVolumeSource>,

    /// AWSElasticBlockStore represents an AWS Disk resource (deprecated).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aws_elastic_block_store: Option<AWSElasticBlockStoreVolumeSource>,

    /// GitRepo represents a git repository at a particular revision (deprecated).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_repo: Option<GitRepoVolumeSource>,

    /// ISCSI represents an ISCSI Disk resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iscsi: Option<ISCSIVolumeSource>,

    /// Glusterfs represents a Glusterfs mount (deprecated).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glusterfs: Option<GlusterfsVolumeSource>,

    /// RBD represents a Rados Block Device mount (deprecated).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rbd: Option<RBDVolumeSource>,

    /// FlexVolume represents a generic volume resource (deprecated).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flex_volume: Option<FlexVolumeSource>,

    /// Cinder represents a cinder volume (deprecated).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cinder: Option<CinderVolumeSource>,

    /// CephFS represents a Ceph FS mount (deprecated).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cephfs: Option<CephFSVolumeSource>,

    /// Flocker represents a Flocker volume (deprecated).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flocker: Option<FlockerVolumeSource>,

    /// FC represents a Fibre Channel resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fc: Option<FCVolumeSource>,

    /// AzureFile represents an Azure File Service mount (deprecated).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azure_file: Option<AzureFileVolumeSource>,

    /// VsphereVolume represents a vSphere volume (deprecated).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vsphere_volume: Option<VsphereVirtualDiskVolumeSource>,

    /// Quobyte represents a Quobyte mount (deprecated).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quobyte: Option<QuobyteVolumeSource>,

    /// AzureDisk represents an Azure Data Disk mount (deprecated).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azure_disk: Option<AzureDiskVolumeSource>,

    /// PhotonPersistentDisk represents a PhotonController persistent disk (deprecated).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub photon_persistent_disk: Option<PhotonPersistentDiskVolumeSource>,

    /// PortworxVolume represents a Portworx volume (deprecated).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub portworx_volume: Option<PortworxVolumeSource>,

    /// ScaleIO represents a ScaleIO persistent volume (deprecated).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale_io: Option<ScaleIOVolumeSource>,

    /// StorageOS represents a StorageOS volume (deprecated).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storageos: Option<StorageOSVolumeSource>,

    /// Ephemeral represents a volume that is handled by a cluster storage driver.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<EphemeralVolumeSource>,

    /// Image represents an OCI object pulled and mounted on the kubelet's host machine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageVolumeSource>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostPathVolumeSource {
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub host_path_type: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmptyDirVolumeSource {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub medium: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_limit: Option<Quantity>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretVolumeSource {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub secret_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<KeyToPath>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapVolumeSource {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<KeyToPath>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyToPath {
    pub key: String,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaimVolumeSource {
    pub claim_name: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectedVolumeSource {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<VolumeProjection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<i32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeProjection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<SecretProjection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_map: Option<ConfigMapProjection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downward_a_p_i: Option<DownwardAPIProjection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_account_token: Option<ServiceAccountTokenProjection>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretProjection {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<KeyToPath>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapProjection {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<KeyToPath>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownwardAPIProjection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DownwardAPIVolumeFile>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccountTokenProjection {
    pub path: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub audience: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownwardAPIVolumeSource {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DownwardAPIVolumeFile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<i32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownwardAPIVolumeFile {
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_ref: Option<ObjectFieldSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_field_ref: Option<ResourceFieldSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSIVolumeSource {
    pub driver: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub volume_attributes: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_publish_secret_ref: Option<LocalObjectReference>,
}

/// VolumeMount describes a mounting of a Volume within a container.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeMount {
    /// This must match the Name of a Volume.
    pub name: String,

    /// Path within the container at which the volume should be mounted.
    pub mount_path: String,

    /// Mounted read-only if true.
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,

    /// Path within the volume from which the container's volume should be mounted.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sub_path: String,

    /// mountPropagation determines how mounts are propagated from the host.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mount_propagation: Option<String>,

    /// Expanded path within the volume from which the container's volume should be mounted.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sub_path_expr: String,

    /// RecursiveReadOnly specifies whether read-only mounts should be handled recursively.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recursive_read_only: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeDevice {
    pub name: String,
    pub device_path: String,
}

// =============================================================================
// Service
// =============================================================================

/// Service is a named abstraction of software service.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: ObjectMeta,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ServiceSpec>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ServiceStatus>,
}

impl Service {
    pub const KIND: &'static str = "Service";
    pub const API_VERSION: &'static str = "v1";

    pub fn new(name: impl Into<String>) -> Self {
        Self {
            type_meta: TypeMeta::new(Self::API_VERSION, Self::KIND),
            metadata: ObjectMeta::named(name),
            ..Default::default()
        }
    }
}

/// ServiceList holds a list of services.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,

    pub items: Vec<Service>,
}

/// ServiceSpec describes the attributes that a user creates on a service.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceSpec {
    /// The list of ports that are exposed by this service.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<ServicePort>,

    /// Route service traffic to pods with label keys and values matching this selector.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub selector: BTreeMap<String, String>,

    /// clusterIP is the IP address of the service.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub cluster_i_p: String,

    /// ClusterIPs is a list of IP addresses assigned to this service.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cluster_i_ps: Vec<String>,

    /// type determines how the Service is exposed.
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "type")]
    pub service_type: String,

    /// externalIPs is a list of IP addresses for which nodes in the cluster will accept traffic.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub external_i_ps: Vec<String>,

    /// sessionAffinity determines the session affinity configuration.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub session_affinity: String,

    /// Only applies to Service Type: LoadBalancer.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub load_balancer_i_p: String,

    /// loadBalancerSourceRanges is a list of IP CIDR ranges.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub load_balancer_source_ranges: Vec<String>,

    /// externalName is the external reference that discovery mechanisms will return.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub external_name: String,

    /// externalTrafficPolicy describes how nodes distribute service traffic.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub external_traffic_policy: String,

    /// healthCheckNodePort specifies the healthcheck nodePort for the service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health_check_node_port: Option<i32>,

    /// publishNotReadyAddresses indicates that any agent should publish addresses.
    #[serde(default, skip_serializing_if = "is_false")]
    pub publish_not_ready_addresses: bool,

    /// sessionAffinityConfig contains the configurations of session affinity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_affinity_config: Option<SessionAffinityConfig>,

    /// IPFamilies is a list of IP families.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ip_families: Vec<String>,

    /// IPFamilyPolicy represents the dual-stack-ness requested.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_family_policy: Option<String>,

    /// allocateLoadBalancerNodePorts defines if NodePorts will be allocated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocate_load_balancer_node_ports: Option<bool>,

    /// loadBalancerClass is the class of the load balancer implementation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_class: Option<String>,

    /// internalTrafficPolicy describes how nodes distribute service traffic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internal_traffic_policy: Option<String>,
}

/// ServicePort contains information on service's port.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServicePort {
    /// The name of this port within the service.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// The IP protocol for this port.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub protocol: String,

    /// The application protocol for this port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_protocol: Option<String>,

    /// The port that will be exposed by this service.
    pub port: i32,

    /// Number or name of the port to access on the pods targeted by the service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_port: Option<IntOrString>,

    /// The port on each node on which this service is exposed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_port: Option<i32>,
}

/// ServiceStatus represents the current status of a service.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceStatus {
    /// LoadBalancer contains the current status of the load-balancer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<LoadBalancerStatus>,

    /// Current service state.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<k8s_apimachinery::Condition>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadBalancerStatus {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingress: Vec<LoadBalancerIngress>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadBalancerIngress {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ip: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hostname: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<PortStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortStatus {
    pub port: i32,
    pub protocol: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionAffinityConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_i_p: Option<ClientIPConfig>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientIPConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
}

// =============================================================================
// ConfigMap
// =============================================================================

/// ConfigMap holds configuration data for pods to consume.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMap {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: ObjectMeta,

    /// Immutable, if set to true, ensures that data stored cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub immutable: Option<bool>,

    /// Data contains the configuration data.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub data: BTreeMap<String, String>,

    /// BinaryData contains the binary data.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub binary_data: BTreeMap<String, Vec<u8>>,
}

impl ConfigMap {
    pub const KIND: &'static str = "ConfigMap";
    pub const API_VERSION: &'static str = "v1";

    pub fn new(name: impl Into<String>) -> Self {
        Self {
            type_meta: TypeMeta::new(Self::API_VERSION, Self::KIND),
            metadata: ObjectMeta::named(name),
            ..Default::default()
        }
    }
}

/// ConfigMapList is a resource containing a list of ConfigMap objects.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,

    pub items: Vec<ConfigMap>,
}

// =============================================================================
// Secret
// =============================================================================

/// Secret holds secret data of a certain type.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Secret {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: ObjectMeta,

    /// Immutable, if set to true, ensures that data stored cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub immutable: Option<bool>,

    /// Data contains the secret data.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub data: BTreeMap<String, Vec<u8>>,

    /// stringData allows specifying non-binary secret data in string form.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub string_data: BTreeMap<String, String>,

    /// Used to facilitate programmatic handling of secret data.
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "type")]
    pub secret_type: String,
}

impl Secret {
    pub const KIND: &'static str = "Secret";
    pub const API_VERSION: &'static str = "v1";

    pub fn new(name: impl Into<String>) -> Self {
        Self {
            type_meta: TypeMeta::new(Self::API_VERSION, Self::KIND),
            metadata: ObjectMeta::named(name),
            ..Default::default()
        }
    }
}

/// SecretList is a list of Secret.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,

    pub items: Vec<Secret>,
}

// =============================================================================
// Namespace
// =============================================================================

/// Namespace provides a scope for Names.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Namespace {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: ObjectMeta,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<NamespaceSpec>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<NamespaceStatus>,
}

impl Namespace {
    pub const KIND: &'static str = "Namespace";
    pub const API_VERSION: &'static str = "v1";

    pub fn new(name: impl Into<String>) -> Self {
        Self {
            type_meta: TypeMeta::new(Self::API_VERSION, Self::KIND),
            metadata: ObjectMeta::named(name),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceSpec {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub finalizers: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceStatus {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub phase: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<NamespaceCondition>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceCondition {
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

/// NamespaceList is a list of Namespaces.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,

    pub items: Vec<Namespace>,
}

// =============================================================================
// Node
// =============================================================================

/// Node is a worker node in Kubernetes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: ObjectMeta,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<NodeSpec>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<NodeStatus>,
}

impl Node {
    pub const KIND: &'static str = "Node";
    pub const API_VERSION: &'static str = "v1";
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeSpec {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub pod_c_i_d_r: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pod_c_i_d_rs: Vec<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub provider_i_d: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub unschedulable: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub taints: Vec<Taint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_source: Option<NodeConfigSource>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeStatus {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub capacity: BTreeMap<String, Quantity>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub allocatable: BTreeMap<String, Quantity>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub phase: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<NodeCondition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<NodeAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub daemon_endpoints: Option<NodeDaemonEndpoints>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_info: Option<NodeSystemInfo>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<ContainerImage>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes_in_use: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes_attached: Vec<AttachedVolume>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeCondition {
    #[serde(rename = "type")]
    pub condition_type: String,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_heartbeat_time: Option<Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Time>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeAddress {
    #[serde(rename = "type")]
    pub address_type: String,
    pub address: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeDaemonEndpoints {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubelet_endpoint: Option<DaemonEndpoint>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DaemonEndpoint {
    pub port: i32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeSystemInfo {
    pub machine_i_d: String,
    pub system_u_u_i_d: String,
    pub boot_i_d: String,
    pub kernel_version: String,
    pub os_image: String,
    pub container_runtime_version: String,
    pub kubelet_version: String,
    pub kube_proxy_version: String,
    pub operating_system: String,
    pub architecture: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerImage {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachedVolume {
    pub name: String,
    pub device_path: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConfigSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_map: Option<ConfigMapNodeConfigSource>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapNodeConfigSource {
    pub namespace: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub uid: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource_version: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kubelet_config_key: String,
}

/// NodeList is the whole list of all Nodes which have been registered.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,

    pub items: Vec<Node>,
}

// =============================================================================
// ServiceAccount
// =============================================================================

/// ServiceAccount binds together a name and references to secrets.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccount {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: ObjectMeta,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<ObjectReference>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image_pull_secrets: Vec<LocalObjectReference>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automount_service_account_token: Option<bool>,
}

impl ServiceAccount {
    pub const KIND: &'static str = "ServiceAccount";
    pub const API_VERSION: &'static str = "v1";

    pub fn new(name: impl Into<String>) -> Self {
        Self {
            type_meta: TypeMeta::new(Self::API_VERSION, Self::KIND),
            metadata: ObjectMeta::named(name),
            ..Default::default()
        }
    }
}

/// ServiceAccountList is a list of ServiceAccount objects.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccountList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,

    pub items: Vec<ServiceAccount>,
}

// =============================================================================
// PersistentVolume and PersistentVolumeClaim
// =============================================================================

/// PersistentVolume (PV) is a storage resource provisioned by an administrator.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolume {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: ObjectMeta,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PersistentVolumeSpec>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PersistentVolumeStatus>,
}

impl PersistentVolume {
    pub const KIND: &'static str = "PersistentVolume";
    pub const API_VERSION: &'static str = "v1";
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeSpec {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub capacity: BTreeMap<String, Quantity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub access_modes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_ref: Option<ObjectReference>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub persistent_volume_reclaim_policy: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub storage_class_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mount_options: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_affinity: Option<VolumeNodeAffinity>,
    // Volume sources (simplified)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_path: Option<HostPathVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nfs: Option<NFSVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csi: Option<CSIPersistentVolumeSource>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeStatus {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub phase: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_phase_transition_time: Option<Time>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeNodeAffinity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<NodeSelector>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeSelector {
    pub node_selector_terms: Vec<NodeSelectorTerm>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeSelectorTerm {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_expressions: Vec<NodeSelectorRequirement>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_fields: Vec<NodeSelectorRequirement>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeSelectorRequirement {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NFSVolumeSource {
    pub server: String,
    pub path: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSIPersistentVolumeSource {
    pub driver: String,
    pub volume_handle: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub volume_attributes: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controller_publish_secret_ref: Option<SecretReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_stage_secret_ref: Option<SecretReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_publish_secret_ref: Option<SecretReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controller_expand_secret_ref: Option<SecretReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_expand_secret_ref: Option<SecretReference>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretReference {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
}

/// PersistentVolumeClaim is a user's request for and claim to a persistent volume.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaim {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: ObjectMeta,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PersistentVolumeClaimSpec>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PersistentVolumeClaimStatus>,
}

impl PersistentVolumeClaim {
    pub const KIND: &'static str = "PersistentVolumeClaim";
    pub const API_VERSION: &'static str = "v1";

    pub fn new(name: impl Into<String>) -> Self {
        Self {
            type_meta: TypeMeta::new(Self::API_VERSION, Self::KIND),
            metadata: ObjectMeta::named(name),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaimSpec {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub access_modes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<VolumeResourceRequirements>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub volume_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_source: Option<TypedLocalObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_source_ref: Option<TypedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_attributes_class_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeResourceRequirements {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub limits: BTreeMap<String, Quantity>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub requests: BTreeMap<String, Quantity>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypedLocalObjectReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_group: Option<String>,
    pub kind: String,
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypedObjectReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_group: Option<String>,
    pub kind: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaimStatus {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub phase: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub access_modes: Vec<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub capacity: BTreeMap<String, Quantity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<PersistentVolumeClaimCondition>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub allocated_resources: BTreeMap<String, Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocated_resource_statuses: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_volume_attributes_class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_volume_status: Option<ModifyVolumeStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaimCondition {
    #[serde(rename = "type")]
    pub condition_type: String,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_probe_time: Option<Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Time>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyVolumeStatus {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub target_volume_attributes_class_name: String,
    pub status: String,
}

/// PersistentVolumeList is a list of PersistentVolume items.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,

    pub items: Vec<PersistentVolume>,
}

/// PersistentVolumeClaimList is a list of PersistentVolumeClaim items.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaimList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,

    pub items: Vec<PersistentVolumeClaim>,
}

// =============================================================================
// Supporting Types
// =============================================================================

/// ObjectReference contains enough information to let you inspect or modify the referred object.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectReference {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub uid: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_version: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource_version: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub field_path: String,
}

/// LocalObjectReference contains enough information to let you locate the referenced object.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalObjectReference {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
}

/// Taint represents a taint on a node.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Taint {
    pub key: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,
    pub effect: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_added: Option<Time>,
}

/// Toleration represents a toleration.
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

/// HostAlias holds the mapping between IP and hostnames.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostAlias {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ip: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hostnames: Vec<String>,
}

/// Affinity is a group of affinity scheduling rules.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Affinity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_affinity: Option<NodeAffinity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_affinity: Option<PodAffinity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_anti_affinity: Option<PodAntiAffinity>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeAffinity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_during_scheduling_ignored_during_execution: Option<NodeSelector>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preferred_during_scheduling_ignored_during_execution: Vec<PreferredSchedulingTerm>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreferredSchedulingTerm {
    pub weight: i32,
    pub preference: NodeSelectorTerm,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodAffinity {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_during_scheduling_ignored_during_execution: Vec<PodAffinityTerm>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preferred_during_scheduling_ignored_during_execution: Vec<WeightedPodAffinityTerm>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodAntiAffinity {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_during_scheduling_ignored_during_execution: Vec<PodAffinityTerm>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preferred_during_scheduling_ignored_during_execution: Vec<WeightedPodAffinityTerm>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodAffinityTerm {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<LabelSelector>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub namespaces: Vec<String>,
    pub topology_key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<LabelSelector>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_label_keys: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mismatch_label_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeightedPodAffinityTerm {
    pub weight: i32,
    pub pod_affinity_term: PodAffinityTerm,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopologySpreadConstraint {
    pub max_skew: i32,
    pub topology_key: String,
    pub when_unsatisfiable: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<LabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_domains: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_affinity_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_taints_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_label_keys: Vec<String>,
}

// =============================================================================
// Security Context
// =============================================================================

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodSecurityContext {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub se_linux_options: Option<SELinuxOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windows_options: Option<WindowsSecurityContextOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_user: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_group: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_non_root: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supplemental_groups: Vec<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_group: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sysctls: Vec<Sysctl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_group_change_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seccomp_profile: Option<SeccompProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_armor_profile: Option<AppArmorProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplemental_groups_policy: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityContext {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub se_linux_options: Option<SELinuxOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windows_options: Option<WindowsSecurityContextOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_user: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_group: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_non_root: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only_root_filesystem: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_privilege_escalation: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proc_mount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seccomp_profile: Option<SeccompProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_armor_profile: Option<AppArmorProfile>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub drop: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SELinuxOptions {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub user: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub role: String,
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "type")]
    pub se_type: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub level: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowsSecurityContextOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gmsa_credential_spec_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gmsa_credential_spec: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_process: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sysctl {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeccompProfile {
    #[serde(rename = "type")]
    pub profile_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localhost_profile: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppArmorProfile {
    #[serde(rename = "type")]
    pub profile_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localhost_profile: Option<String>,
}

// =============================================================================
// Probes and Lifecycle
// =============================================================================

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Probe {
    #[serde(flatten)]
    pub probe_handler: ProbeHandler,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_delay_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success_threshold: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termination_grace_period_seconds: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProbeHandler {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<ExecAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_get: Option<HTTPGetAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp_socket: Option<TCPSocketAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grpc: Option<GRPCAction>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecAction {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HTTPGetAction {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    pub port: IntOrString,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub scheme: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub http_headers: Vec<HTTPHeader>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HTTPHeader {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TCPSocketAction {
    pub port: IntOrString,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GRPCAction {
    pub port: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lifecycle {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_start: Option<LifecycleHandler>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_stop: Option<LifecycleHandler>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifecycleHandler {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<ExecAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_get: Option<HTTPGetAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp_socket: Option<TCPSocketAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sleep: Option<SleepAction>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SleepAction {
    pub seconds: i64,
}

// =============================================================================
// DNS and Pod OS
// =============================================================================

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodDNSConfig {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nameservers: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub searches: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<PodDNSConfigOption>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodDNSConfigOption {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodReadinessGate {
    pub condition_type: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodOS {
    pub name: String,
}

// =============================================================================
// Additional Volume Source Types
// =============================================================================

/// GCEPersistentDiskVolumeSource represents a GCE Disk resource.
///
/// Deprecated: GCEPersistentDisk is deprecated.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GCEPersistentDiskVolumeSource {
    pub pd_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partition: Option<i32>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
}

/// AWSElasticBlockStoreVolumeSource represents an AWS Disk resource.
///
/// Deprecated: AWSElasticBlockStore is deprecated.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AWSElasticBlockStoreVolumeSource {
    #[serde(rename = "volumeID")]
    pub volume_id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partition: Option<i32>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
}

/// GitRepoVolumeSource represents a git repository.
///
/// DEPRECATED: GitRepo is deprecated.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitRepoVolumeSource {
    pub repository: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub revision: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub directory: String,
}

/// ISCSIVolumeSource represents an ISCSI disk.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ISCSIVolumeSource {
    pub target_portal: String,
    pub iqn: String,
    pub lun: i32,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub iscsi_interface: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub portals: Vec<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub chap_auth_discovery: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub chap_auth_session: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<LocalObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initiator_name: Option<String>,
}

/// GlusterfsVolumeSource represents a Glusterfs mount.
///
/// Deprecated: Glusterfs is deprecated.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlusterfsVolumeSource {
    pub endpoints: String,
    pub path: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
}

/// RBDVolumeSource represents a Rados Block Device mount.
///
/// Deprecated: RBD is deprecated.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RBDVolumeSource {
    pub monitors: Vec<String>,
    pub image: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub pool: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub user: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub keyring: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<LocalObjectReference>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
}

/// FlexVolumeSource represents a generic volume resource.
///
/// Deprecated: FlexVolume is deprecated.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexVolumeSource {
    pub driver: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<LocalObjectReference>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub options: BTreeMap<String, String>,
}

/// CinderVolumeSource represents a cinder volume.
///
/// Deprecated: Cinder is deprecated.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CinderVolumeSource {
    #[serde(rename = "volumeID")]
    pub volume_id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<LocalObjectReference>,
}

/// CephFSVolumeSource represents a Ceph FS mount.
///
/// Deprecated: CephFS is deprecated.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CephFSVolumeSource {
    pub monitors: Vec<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub user: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub secret_file: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<LocalObjectReference>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
}

/// FlockerVolumeSource represents a Flocker volume.
///
/// Deprecated: Flocker is deprecated.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlockerVolumeSource {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub dataset_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "datasetUUID")]
    pub dataset_uuid: String,
}

/// FCVolumeSource represents a Fibre Channel volume.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FCVolumeSource {
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "targetWWNs")]
    pub target_wwns: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wwids: Vec<String>,
}

/// AzureFileVolumeSource represents an Azure File Service mount.
///
/// Deprecated: AzureFile is deprecated.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AzureFileVolumeSource {
    pub secret_name: String,
    pub share_name: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
}

/// VsphereVirtualDiskVolumeSource represents a vSphere volume.
///
/// Deprecated: VsphereVolume is deprecated.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VsphereVirtualDiskVolumeSource {
    pub volume_path: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub storage_policy_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "storagePolicyID")]
    pub storage_policy_id: String,
}

/// QuobyteVolumeSource represents a Quobyte mount.
///
/// Deprecated: Quobyte is deprecated.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuobyteVolumeSource {
    pub registry: String,
    pub volume: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub user: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub tenant: String,
}

/// AzureDiskVolumeSource represents an Azure Data Disk mount.
///
/// Deprecated: AzureDisk is deprecated.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AzureDiskVolumeSource {
    pub disk_name: String,
    #[serde(rename = "diskURI")]
    pub disk_uri: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caching_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

/// PhotonPersistentDiskVolumeSource represents a PhotonController persistent disk.
///
/// Deprecated: PhotonPersistentDisk is deprecated.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhotonPersistentDiskVolumeSource {
    #[serde(rename = "pdID")]
    pub pd_id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
}

/// PortworxVolumeSource represents a Portworx volume.
///
/// Deprecated: PortworxVolume is deprecated.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortworxVolumeSource {
    #[serde(rename = "volumeID")]
    pub volume_id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
}

/// ScaleIOVolumeSource represents a ScaleIO persistent volume.
///
/// Deprecated: ScaleIO is deprecated.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScaleIOVolumeSource {
    pub gateway: String,
    pub system: String,
    pub secret_ref: LocalObjectReference,
    #[serde(default, skip_serializing_if = "is_false", rename = "sslEnabled")]
    pub ssl_enabled: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub protection_domain: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub storage_pool: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub storage_mode: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub volume_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
}

/// StorageOSVolumeSource represents a StorageOS volume.
///
/// Deprecated: StorageOS is deprecated.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageOSVolumeSource {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub volume_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub volume_namespace: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<LocalObjectReference>,
}

/// EphemeralVolumeSource represents an ephemeral volume.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EphemeralVolumeSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_claim_template: Option<PersistentVolumeClaimTemplate>,
}

/// PersistentVolumeClaimTemplate is used to produce PersistentVolumeClaim objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaimTemplate {
    #[serde(default)]
    pub metadata: ObjectMeta,
    pub spec: PersistentVolumeClaimSpec,
}

/// ImageVolumeSource represents an OCI object.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageVolumeSource {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reference: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub pull_policy: String,
}

// =============================================================================
// Additional Core Resources
// =============================================================================

/// Endpoints is a collection of endpoints that implement the actual service.
///
/// Deprecated: This API is deprecated in v1.33+. Use discoveryv1.EndpointSlice instead.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoints {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subsets: Vec<EndpointSubset>,
}

/// EndpointSubset is a group of addresses with a common set of ports.
///
/// Deprecated: This API is deprecated in v1.33+.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointSubset {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<EndpointAddress>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub not_ready_addresses: Vec<EndpointAddress>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<EndpointPort>,
}

/// EndpointAddress is a tuple that describes single IP address.
///
/// Deprecated: This API is deprecated in v1.33+.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointAddress {
    pub ip: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hostname: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_ref: Option<ObjectReference>,
}

/// EndpointPort is a tuple that describes a single port.
///
/// Deprecated: This API is deprecated in v1.33+.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointPort {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    pub port: i32,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub protocol: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_protocol: Option<String>,
}

/// ReplicationController represents the configuration of a replication controller.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplicationController {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ReplicationControllerSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReplicationControllerStatus>,
}

/// ReplicationControllerSpec is the specification of a replication controller.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplicationControllerSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    #[serde(default)]
    pub selector: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<PodTemplateSpec>,
    #[serde(default)]
    pub min_ready_seconds: i32,
}

/// ReplicationControllerStatus represents the current status of a replication controller.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplicationControllerStatus {
    pub replicas: i32,
    #[serde(default)]
    pub fully_labeled_replicas: i32,
    #[serde(default)]
    pub ready_replicas: i32,
    #[serde(default)]
    pub available_replicas: i32,
    #[serde(default)]
    pub observed_generation: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<ReplicationControllerCondition>,
}

/// ReplicationControllerCondition describes the state of a replication controller.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplicationControllerCondition {
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

/// LimitRange sets resource usage limits for each kind of resource in a Namespace.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitRange {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<LimitRangeSpec>,
}

/// LimitRangeSpec defines a min/max usage limit for resources.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitRangeSpec {
    pub limits: Vec<LimitRangeItem>,
}

/// LimitRangeItem defines a min/max usage limit for any resource.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitRangeItem {
    #[serde(rename = "type")]
    pub limit_type: String,
    #[serde(default)]
    pub max: HashMap<String, Quantity>,
    #[serde(default)]
    pub min: HashMap<String, Quantity>,
    #[serde(default)]
    pub default: HashMap<String, Quantity>,
    #[serde(default)]
    pub default_request: HashMap<String, Quantity>,
    #[serde(default)]
    pub max_limit_request_ratio: HashMap<String, Quantity>,
}

/// ResourceQuota sets aggregate quota restrictions enforced per namespace.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceQuota {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ResourceQuotaSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResourceQuotaStatus>,
}

/// ResourceQuotaSpec defines the desired hard limits to enforce for Quota.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceQuotaSpec {
    #[serde(default)]
    pub hard: HashMap<String, Quantity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_selector: Option<ScopeSelector>,
}

/// ResourceQuotaStatus defines the enforced hard limits and observed use.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceQuotaStatus {
    #[serde(default)]
    pub hard: HashMap<String, Quantity>,
    #[serde(default)]
    pub used: HashMap<String, Quantity>,
}

/// ScopeSelector represents the AND of the selectors.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopeSelector {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_expressions: Vec<ScopedResourceSelectorRequirement>,
}

/// ScopedResourceSelectorRequirement is a selector that contains values and an operator.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopedResourceSelectorRequirement {
    pub scope_name: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}

// =============================================================================
// Missing List Types
// =============================================================================

/// ResourceQuotaList is a list of ResourceQuota items.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceQuotaList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<ResourceQuota>,
}

/// LimitRangeList is a list of LimitRange items.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitRangeList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<LimitRange>,
}

/// EndpointsList is a list of Endpoints items.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointsList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<Endpoints>,
}

// =============================================================================
// Event
// =============================================================================

/// Event is a report of an event somewhere in the cluster.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    #[serde(default)]
    pub metadata: ObjectMeta,

    /// The object that this event is about.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub involved_object: Option<ObjectReference>,

    /// This should be a short, machine understandable string.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,

    /// A human-readable description of the status of this operation.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,

    /// The component reporting this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<EventSource>,

    /// The time at which the event was first recorded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_timestamp: Option<Time>,

    /// The time at which the most recent occurrence of this event was recorded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_timestamp: Option<Time>,

    /// The number of times this event has occurred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,

    /// Type of this event (Normal, Warning).
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "type")]
    pub event_type: String,

    /// Time when this Event was first observed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_time: Option<Time>,

    /// Data about the Event series this event represents.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series: Option<EventSeries>,

    /// What action was taken/failed regarding the Regarding object.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub action: String,

    /// Optional secondary object for more complex actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<ObjectReference>,

    /// Name of the controller that emitted this Event.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reporting_controller: String,

    /// ID of the controller instance.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reporting_instance: String,
}

impl Event {
    pub const KIND: &'static str = "Event";
    pub const API_VERSION: &'static str = "v1";
}

/// EventList is a list of events.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<Event>,
}

/// EventSource contains information for an event.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventSource {
    /// Component from which the event is generated.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub component: String,
    /// Node name on which the event is generated.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host: String,
}

/// EventSeries contains information on series of events.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventSeries {
    /// Number of occurrences in this series up to the last heartbeat time.
    #[serde(default)]
    pub count: i32,
    /// Time of the last occurrence observed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_observed_time: Option<Time>,
}

// =============================================================================
// Binding
// =============================================================================

/// Binding ties one object to another; for example, a pod is bound to a node by a scheduler.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Binding {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    #[serde(default)]
    pub metadata: ObjectMeta,

    /// The target object that you want to bind to the standard object.
    pub target: ObjectReference,
}

impl Binding {
    pub const KIND: &'static str = "Binding";
    pub const API_VERSION: &'static str = "v1";

    pub fn new(name: impl Into<String>, target_node: impl Into<String>) -> Self {
        Self {
            type_meta: TypeMeta::new(Self::API_VERSION, Self::KIND),
            metadata: ObjectMeta::named(name),
            target: ObjectReference {
                kind: "Node".to_string(),
                name: target_node.into(),
                ..Default::default()
            },
        }
    }
}

// =============================================================================
// ComponentStatus (deprecated but still used)
// =============================================================================

/// ComponentStatus is the status of an individual component.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentStatus {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: ObjectMeta,

    /// List of component conditions observed.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<ComponentCondition>,
}

impl ComponentStatus {
    pub const KIND: &'static str = "ComponentStatus";
    pub const API_VERSION: &'static str = "v1";
}

/// ComponentStatusList is a list of ComponentStatus objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentStatusList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<ComponentStatus>,
}

/// ComponentCondition describes the condition of a component.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentCondition {
    /// Type of condition.
    #[serde(rename = "type")]
    pub condition_type: String,
    /// Status of the condition.
    pub status: String,
    /// Message about the condition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    /// Condition error code.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub error: String,
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pod_serialization() {
        let pod = Pod {
            type_meta: TypeMeta::new("v1", "Pod"),
            metadata: ObjectMeta::namespaced("default", "nginx"),
            spec: Some(PodSpec {
                containers: vec![Container::new("nginx", "nginx:latest")],
                ..Default::default()
            }),
            status: None,
        };

        let json = serde_json::to_string_pretty(&pod).unwrap();
        assert!(json.contains("\"kind\": \"Pod\""));
        assert!(json.contains("\"name\": \"nginx\""));

        let parsed: Pod = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.metadata.name, "nginx");
    }

    #[test]
    fn test_service_serialization() {
        let svc = Service {
            type_meta: TypeMeta::new("v1", "Service"),
            metadata: ObjectMeta::namespaced("default", "my-service"),
            spec: Some(ServiceSpec {
                ports: vec![ServicePort {
                    port: 80,
                    target_port: Some(IntOrString::Int(8080)),
                    ..Default::default()
                }],
                selector: [("app".to_string(), "nginx".to_string())]
                    .into_iter()
                    .collect(),
                ..Default::default()
            }),
            status: None,
        };

        let json = serde_json::to_string_pretty(&svc).unwrap();
        assert!(json.contains("\"kind\": \"Service\""));
    }

    #[test]
    fn test_configmap() {
        let cm = ConfigMap {
            type_meta: TypeMeta::new("v1", "ConfigMap"),
            metadata: ObjectMeta::namespaced("default", "my-config"),
            data: [("key".to_string(), "value".to_string())]
                .into_iter()
                .collect(),
            ..Default::default()
        };

        let json = serde_json::to_string_pretty(&cm).unwrap();
        let parsed: ConfigMap = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.data.get("key"), Some(&"value".to_string()));
    }
}
