//! Pod-related types for Kubernetes core/v1 API.
//!
//! This module contains types related to Pods, including Pod, PodSpec,
//! PodStatus, PodCondition, and related types.

use crate::common::LocalObjectReference;
use crate::container::Container;
use k8s_api_core::{NamespacedResource, Resource};
use k8s_api_meta::{LabelSelector, ObjectMeta, Time, TypeMeta};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;

/// RestartPolicy describes how the container should be restarted.
pub type RestartPolicy = String;

/// Restart policy constants.
pub mod restart_policy {
    /// Always restart the container.
    pub const ALWAYS: &str = "Always";
    /// Restart only on failure.
    pub const ON_FAILURE: &str = "OnFailure";
    /// Never restart.
    pub const NEVER: &str = "Never";
}

/// DNSPolicy defines how a pod's DNS will be configured.
pub type DNSPolicy = String;

/// DNS policy constants.
pub mod dns_policy {
    /// ClusterFirstWithHostNet indicates that the pod should use cluster DNS
    /// first, unless hostNetwork is true.
    pub const CLUSTER_FIRST_WITH_HOST_NET: &str = "ClusterFirstWithHostNet";
    /// ClusterFirst indicates that the pod should use cluster DNS first.
    pub const CLUSTER_FIRST: &str = "ClusterFirst";
    /// Default indicates that the pod should use the default DNS settings.
    pub const DEFAULT: &str = "Default";
    /// None indicates that the pod should use empty DNS settings.
    pub const NONE: &str = "None";
}

/// PodPhase is a label for the condition of a pod at the current time.
pub type PodPhase = String;

/// Pod phase constants.
pub mod pod_phase {
    /// Pending means the pod has been accepted but not yet scheduled.
    pub const PENDING: &str = "Pending";
    /// Running means the pod has been bound to a node.
    pub const RUNNING: &str = "Running";
    /// Succeeded means all containers terminated successfully.
    pub const SUCCEEDED: &str = "Succeeded";
    /// Failed means all containers terminated, at least one with failure.
    pub const FAILED: &str = "Failed";
    /// Unknown means the pod state could not be obtained.
    pub const UNKNOWN: &str = "Unknown";
}

/// PodConditionType is a valid value for PodCondition.Type.
pub type PodConditionType = String;

/// Pod condition type constants.
pub mod pod_condition_type {
    /// PodScheduled represents status of the scheduling process.
    pub const POD_SCHEDULED: &str = "PodScheduled";
    /// PodReady means the pod is ready to serve requests.
    pub const POD_READY: &str = "Ready";
    /// ContainersReady indicates all containers in the pod are ready.
    pub const CONTAINERS_READY: &str = "ContainersReady";
    /// PodInitialized means all init containers have started successfully.
    pub const POD_INITIALIZED: &str = "Initialized";
}

/// Pod is a collection of containers that can run on a host.
///
/// This resource is created by clients and scheduled onto hosts.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pod {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Specification of the desired behavior of the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PodSpec>,

    /// Most recently observed status of the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PodStatus>,
}

impl Pod {
    /// API version for Pod.
    pub const API_VERSION: &'static str = "v1";
    /// Group for Pod (empty for core API).
    pub const GROUP: &'static str = "";
    /// Kind for Pod.
    pub const KIND: &'static str = "Pod";
    /// Version for Pod.
    pub const VERSION: &'static str = "v1";
    /// Plural name for Pod.
    pub const PLURAL: &'static str = "pods";

    /// Creates a new Pod with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            type_meta: TypeMeta::new(Self::API_VERSION, Self::KIND),
            metadata: Some(ObjectMeta::named(name)),
            spec: None,
            status: None,
        }
    }

    /// Creates a new Pod with the given name and namespace.
    pub fn namespaced(name: impl Into<String>, namespace: impl Into<String>) -> Self {
        Self {
            type_meta: TypeMeta::new(Self::API_VERSION, Self::KIND),
            metadata: Some(ObjectMeta::namespaced(name, namespace)),
            spec: None,
            status: None,
        }
    }
}

impl Resource for Pod {
    const API_VERSION: &'static str = Pod::API_VERSION;
    const GROUP: &'static str = Pod::GROUP;
    const KIND: &'static str = Pod::KIND;
    const VERSION: &'static str = Pod::VERSION;
    const PLURAL: &'static str = Pod::PLURAL;

    fn api_version(&self) -> Cow<'_, str> {
        Cow::Borrowed(Self::API_VERSION)
    }

    fn kind(&self) -> Cow<'_, str> {
        Cow::Borrowed(Self::KIND)
    }
}

impl NamespacedResource for Pod {}

/// PodList is a list of Pods.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// List of pods.
    pub items: Vec<Pod>,
}

/// ListMeta describes metadata for a list response.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListMeta {
    /// String that identifies the server's internal version of this object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<String>,

    /// Continue token for pagination.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "continue")]
    pub continue_: Option<String>,

    /// Remaining item count (for pagination).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining_item_count: Option<i64>,
}

/// PodTemplateSpec describes the data a pod should have when created from a template.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplateSpec {
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Specification of the desired behavior of the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PodSpec>,
}

/// PodSpec is a description of a pod.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodSpec {
    /// List of volumes that can be mounted by containers belonging to the pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes: Vec<Volume>,

    /// List of initialization containers belonging to the pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub init_containers: Vec<Container>,

    /// List of containers belonging to the pod.
    pub containers: Vec<Container>,

    /// List of ephemeral containers run in this pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ephemeral_containers: Vec<EphemeralContainer>,

    /// Restart policy for all containers within the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<RestartPolicy>,

    /// Optional duration in seconds the pod needs to terminate gracefully.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termination_grace_period_seconds: Option<i64>,

    /// Optional duration in seconds the pod may be active on the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_deadline_seconds: Option<i64>,

    /// Set DNS policy for the pod. Defaults to "ClusterFirst".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns_policy: Option<DNSPolicy>,

    /// NodeSelector is a selector which must be true for the pod to fit on a node.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub node_selector: BTreeMap<String, String>,

    /// ServiceAccountName is the name of the ServiceAccount to use to run this pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_account_name: Option<String>,

    /// AutomountServiceAccountToken indicates whether a service account token should be automatically mounted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automount_service_account_token: Option<bool>,

    /// NodeName is a request to schedule this pod onto a specific node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,

    /// Host networking requested for this pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_network: Option<bool>,

    /// Use the host's pid namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_pid: Option<bool>,

    /// Use the host's ipc namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_ipc: Option<bool>,

    /// Share a single process namespace between all containers in a pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_process_namespace: Option<bool>,

    /// SecurityContext holds pod-level security attributes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_context: Option<PodSecurityContext>,

    /// ImagePullSecrets is a list of references to secrets for pulling images.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image_pull_secrets: Vec<LocalObjectReference>,

    /// Specifies the hostname of the Pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    /// Specifies the subdomain of the Pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<String>,

    /// If specified, the pod's scheduling constraints.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affinity: Option<Affinity>,

    /// If specified, the pod will be dispatched by specified scheduler.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduler_name: Option<String>,

    /// If specified, the pod's tolerations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tolerations: Vec<Toleration>,

    /// HostAliases is a list of hosts and IPs to inject into the pod's hosts file.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub host_aliases: Vec<HostAlias>,

    /// If specified, indicates the pod's priority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority_class_name: Option<String>,

    /// The priority value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,

    /// Specifies the DNS parameters of a pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<PodDNSConfig>,

    /// If true the pod's hostname will be configured as the pod's FQDN.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_hostname_as_fqdn: Option<bool>,

    /// EnableServiceLinks indicates whether information about services should be injected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_service_links: Option<bool>,

    /// TopologySpreadConstraints describes how a group of pods ought to spread.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topology_spread_constraints: Vec<TopologySpreadConstraint>,
}

impl PodSpec {
    /// Creates a new PodSpec with the given containers.
    pub fn new(containers: Vec<Container>) -> Self {
        Self {
            containers,
            ..Default::default()
        }
    }

    /// Creates a PodSpec with a single container.
    pub fn single_container(container: Container) -> Self {
        Self::new(vec![container])
    }
}

/// PodStatus represents information about the status of a pod.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodStatus {
    /// The phase of a Pod is a simple, high-level summary of where the Pod is in its lifecycle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<PodPhase>,

    /// Current service state of pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<PodCondition>,

    /// A human readable message indicating details about why the pod is in this condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// A brief CamelCase message indicating details about why the pod is in this state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// nominatedNodeName is set when this pod preempts other pods on the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nominated_node_name: Option<String>,

    /// hostIP holds the IP address of the host to which the pod is assigned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_ip: Option<String>,

    /// hostIPs holds the IP addresses allocated to the host.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub host_i_ps: Vec<HostIP>,

    /// podIP address allocated to the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_ip: Option<String>,

    /// podIPs holds the IP addresses allocated to the pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pod_i_ps: Vec<PodIP>,

    /// RFC 3339 date and time at which the object was acknowledged by the Kubelet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Time>,

    /// The list has one entry per init container in the manifest.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub init_container_statuses: Vec<ContainerStatus>,

    /// The list has one entry per container in the manifest.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub container_statuses: Vec<ContainerStatus>,

    /// The Quality of Service (QOS) classification assigned to the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qos_class: Option<String>,

    /// Status for any ephemeral containers that have run in this pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ephemeral_container_statuses: Vec<ContainerStatus>,
}

/// PodCondition contains details for the current condition of this pod.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodCondition {
    /// Type is the type of the condition.
    #[serde(rename = "type")]
    pub type_: PodConditionType,

    /// Status is the status of the condition.
    pub status: String,

    /// Last time we probed the condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_probe_time: Option<Time>,

    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Time>,

    /// Unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Human-readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// HostIP represents a single IP address allocated to the host.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostIP {
    /// IP is the IP address assigned to the host.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

/// PodIP represents a single IP address allocated to the pod.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodIP {
    /// IP is the IP address assigned to the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

/// ContainerStatus contains details for the current status of this container.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerStatus {
    /// Name is a DNS_LABEL representing the unique name of the container.
    pub name: String,

    /// State holds details about the container's current condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ContainerState>,

    /// LastTerminationState holds the last termination state of the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_state: Option<ContainerState>,

    /// Ready specifies whether the container is currently passing its readiness check.
    #[serde(default)]
    pub ready: bool,

    /// RestartCount holds the number of times the container has been restarted.
    #[serde(default)]
    pub restart_count: i32,

    /// Image is the name of container image that the container is running.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    /// ImageID is the image ID of the container's image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,

    /// ContainerID is the ID of the container in the format '<type>://<container_id>'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,

    /// Started indicates whether the container has finished its postStart lifecycle hook.
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

/// ContainerStateWaiting is a waiting state of a container.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerStateWaiting {
    /// Reason is a (brief) reason the container is not yet running.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Message is a human-readable message indicating details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// ContainerStateRunning is a running state of a container.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerStateRunning {
    /// Time at which the container was last (re-)started.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<Time>,
}

/// ContainerStateTerminated is a terminated state of a container.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerStateTerminated {
    /// Exit status from the last termination of the container.
    pub exit_code: i32,

    /// Signal from the last termination of the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signal: Option<i32>,

    /// Reason is a (brief) reason from the last termination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Message is a human-readable message indicating details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Time at which previous execution of the container started.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<Time>,

    /// Time at which the container last terminated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<Time>,

    /// Container's ID in the format '<type>://<container_id>'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
}

/// Volume represents a named volume in a pod.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    /// Name of the volume.
    pub name: String,

    /// VolumeSource represents the location and type of the mounted volume.
    #[serde(flatten)]
    pub volume_source: VolumeSource,
}

impl Volume {
    /// Creates a new Volume with the given name and source.
    pub fn new(name: impl Into<String>, source: VolumeSource) -> Self {
        Self {
            name: name.into(),
            volume_source: source,
        }
    }

    /// Creates an emptyDir volume.
    pub fn empty_dir(name: impl Into<String>) -> Self {
        Self::new(
            name,
            VolumeSource {
                empty_dir: Some(EmptyDirVolumeSource::default()),
                ..Default::default()
            },
        )
    }

    /// Creates a configMap volume.
    pub fn config_map(name: impl Into<String>, config_map_name: impl Into<String>) -> Self {
        Self::new(
            name,
            VolumeSource {
                config_map: Some(ConfigMapVolumeSource {
                    name: Some(config_map_name.into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
    }

    /// Creates a secret volume.
    pub fn secret(name: impl Into<String>, secret_name: impl Into<String>) -> Self {
        Self::new(
            name,
            VolumeSource {
                secret: Some(SecretVolumeSource {
                    secret_name: Some(secret_name.into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
    }
}

/// VolumeSource represents the source of a volume to mount.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeSource {
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

    /// HostPath represents a pre-existing file or directory on the host machine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_path: Option<HostPathVolumeSource>,

    /// DownwardAPI represents downward API about the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downward_api: Option<DownwardAPIVolumeSource>,

    /// Projected items for all in one resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projected: Option<ProjectedVolumeSource>,
}

/// EmptyDirVolumeSource represents an empty directory for a Pod.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmptyDirVolumeSource {
    /// What type of storage medium should back this directory.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,

    /// Total amount of local storage required for this EmptyDir volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_limit: Option<k8s_api_core::Quantity>,
}

/// SecretVolumeSource represents a Secret.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretVolumeSource {
    /// Name of the secret in the pod's namespace to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,

    /// If unspecified, each key-value pair in the Data field of the referenced Secret
    /// will be projected into the volume as a file whose name is the key.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<KeyToPath>,

    /// Optional: mode bits used to set permissions on created files.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<i32>,

    /// Specify whether the Secret or its keys must be defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// ConfigMapVolumeSource represents a ConfigMap.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapVolumeSource {
    /// Name of the configmap in the pod's namespace to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// If unspecified, each key-value pair in the Data field of the referenced ConfigMap
    /// will be projected into the volume as a file whose name is the key.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<KeyToPath>,

    /// Optional: mode bits used to set permissions on created files.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<i32>,

    /// Specify whether the ConfigMap or its keys must be defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// KeyToPath maps a string key to a path within a volume.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyToPath {
    /// The key to project.
    pub key: String,

    /// The relative path of the file to map the key to.
    pub path: String,

    /// Optional: mode bits used to set permissions on this file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
}

/// PersistentVolumeClaimVolumeSource references a PVC.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaimVolumeSource {
    /// ClaimName is the name of a PersistentVolumeClaim in the same namespace.
    pub claim_name: String,

    /// Will force the ReadOnly setting in VolumeMounts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

/// HostPathVolumeSource represents a host path mapped into a pod.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostPathVolumeSource {
    /// Path of the directory on the host.
    pub path: String,

    /// Type for HostPath Volume.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
}

/// DownwardAPIVolumeSource represents a volume containing Downward API info.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownwardAPIVolumeSource {
    /// Items is a list of downward API volume file.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DownwardAPIVolumeFile>,

    /// Optional: mode bits to use on created files.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<i32>,
}

/// DownwardAPIVolumeFile represents information to create the file containing the pod field.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownwardAPIVolumeFile {
    /// Required: Path is the relative path name of the file to be created.
    pub path: String,

    /// Required: Selects a field of the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_ref: Option<crate::container::ObjectFieldSelector>,

    /// Selects a resource of the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_field_ref: Option<crate::container::ResourceFieldSelector>,

    /// Optional: mode bits used to set permissions on this file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
}

/// ProjectedVolumeSource represents a projected volume source.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectedVolumeSource {
    /// List of volume projections.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<VolumeProjection>,

    /// Mode bits used to set permissions on created files.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<i32>,
}

/// VolumeProjection that may be projected along with other supported volume types.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeProjection {
    /// Information about the secret data to project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<SecretProjection>,

    /// Information about the configMap data to project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_map: Option<ConfigMapProjection>,

    /// Information about the downwardAPI data to project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downward_api: Option<DownwardAPIProjection>,

    /// Information about the serviceAccountToken data to project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_account_token: Option<ServiceAccountTokenProjection>,
}

/// SecretProjection adapts a secret into a projected volume.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretProjection {
    /// Name of the secret in the pod's namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// If unspecified, each key-value pair will be projected into the volume.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<KeyToPath>,

    /// Specify whether the Secret or its key must be defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// ConfigMapProjection adapts a ConfigMap into a projected volume.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapProjection {
    /// Name of the configmap in the pod's namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// If unspecified, each key-value pair will be projected into the volume.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<KeyToPath>,

    /// Specify whether the ConfigMap or its key must be defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// DownwardAPIProjection projects Downward API info into a projected volume.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownwardAPIProjection {
    /// Items is a list of DownwardAPIVolume file.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DownwardAPIVolumeFile>,
}

/// ServiceAccountTokenProjection represents a projected service account token volume.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccountTokenProjection {
    /// Audience is the intended audience of the token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,

    /// ExpirationSeconds is the requested duration of validity of the service account token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: Option<i64>,

    /// Path is the path relative to the mount point of the file to project the token into.
    pub path: String,
}

/// EphemeralContainer is a temporary container for user-initiated activities.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EphemeralContainer {
    /// Name of the ephemeral container.
    pub name: String,

    /// Container image name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    /// Entrypoint array.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,

    /// Arguments to the entrypoint.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,

    /// Container's working directory.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,

    /// List of environment variables to set in the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<crate::container::EnvVar>,

    /// Pod volumes to mount into the container's filesystem.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volume_mounts: Vec<crate::container::VolumeMount>,

    /// Name of the container from PodSpec that this ephemeral container targets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_container_name: Option<String>,

    /// Whether this container should allocate a buffer for stdin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdin: Option<bool>,

    /// Whether the container runtime should close the stdin channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdin_once: Option<bool>,

    /// Whether this container should allocate a TTY.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
}

/// PodSecurityContext holds pod-level security attributes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodSecurityContext {
    /// The SELinux context to be applied to all containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub se_linux_options: Option<crate::container::SELinuxOptions>,

    /// The Windows specific settings applied to all containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windows_options: Option<crate::container::WindowsSecurityContextOptions>,

    /// The UID to run the entrypoint of the container process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_user: Option<i64>,

    /// The GID to run the entrypoint of the container process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_group: Option<i64>,

    /// Indicates that the container must run as a non-root user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_non_root: Option<bool>,

    /// A list of groups applied to the first process run in each container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supplemental_groups: Vec<i64>,

    /// A special supplemental group that applies to all containers in a pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_group: Option<i64>,

    /// Sysctls hold a list of namespaced sysctls used for the pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sysctls: Vec<Sysctl>,

    /// fsGroupChangePolicy defines behavior of changing ownership.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_group_change_policy: Option<String>,

    /// The seccomp options to use by the containers in this pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seccomp_profile: Option<crate::container::SeccompProfile>,

    /// The AppArmor options to use by the containers in this pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_armor_profile: Option<crate::container::AppArmorProfile>,
}

/// Sysctl defines a kernel parameter to be set.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sysctl {
    /// Name of a property to set.
    pub name: String,

    /// Value of a property to set.
    pub value: String,
}

/// Affinity is a group of affinity scheduling rules.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Affinity {
    /// Describes node affinity scheduling rules for the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_affinity: Option<NodeAffinity>,

    /// Describes pod affinity scheduling rules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_affinity: Option<PodAffinity>,

    /// Describes pod anti-affinity scheduling rules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_anti_affinity: Option<PodAntiAffinity>,
}

/// NodeAffinity is a group of node affinity scheduling rules.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeAffinity {
    /// If the affinity requirements are not met at scheduling time, the pod will not be scheduled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_during_scheduling_ignored_during_execution: Option<NodeSelector>,

    /// The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preferred_during_scheduling_ignored_during_execution: Vec<PreferredSchedulingTerm>,
}

/// NodeSelector represents a selector that matches nodes.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeSelector {
    /// Required. A list of node selector terms.
    pub node_selector_terms: Vec<NodeSelectorTerm>,
}

/// NodeSelectorTerm contains expressions to match against nodes.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeSelectorTerm {
    /// A list of node selector requirements by node's labels.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_expressions: Vec<NodeSelectorRequirement>,

    /// A list of node selector requirements by node's fields.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_fields: Vec<NodeSelectorRequirement>,
}

/// NodeSelectorRequirement is a requirement for a selector that matches nodes.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeSelectorRequirement {
    /// The label key that the selector applies to.
    pub key: String,

    /// Represents a key's relationship to a set of values.
    pub operator: String,

    /// An array of string values.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}

/// PreferredSchedulingTerm represents a weighted preference.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreferredSchedulingTerm {
    /// Weight associated with matching the corresponding nodeSelectorTerm.
    pub weight: i32,

    /// A node selector term, associated with the corresponding weight.
    pub preference: NodeSelectorTerm,
}

/// PodAffinity is a group of inter pod affinity scheduling rules.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodAffinity {
    /// If the affinity requirements are not met at scheduling time, the pod will not be scheduled.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_during_scheduling_ignored_during_execution: Vec<PodAffinityTerm>,

    /// The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preferred_during_scheduling_ignored_during_execution: Vec<WeightedPodAffinityTerm>,
}

/// PodAntiAffinity is a group of inter pod anti affinity scheduling rules.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodAntiAffinity {
    /// If the anti-affinity requirements are not met at scheduling time, the pod will not be scheduled.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_during_scheduling_ignored_during_execution: Vec<PodAffinityTerm>,

    /// The scheduler will prefer to schedule pods to nodes that satisfy the anti-affinity expressions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preferred_during_scheduling_ignored_during_execution: Vec<WeightedPodAffinityTerm>,
}

/// PodAffinityTerm defines a set of pods to co-locate with or avoid.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodAffinityTerm {
    /// A label query over a set of resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<LabelSelector>,

    /// namespaces specifies a static list of namespace names that the term applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub namespaces: Vec<String>,

    /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces.
    pub topology_key: String,

    /// A label query over the set of namespaces that the term applies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<LabelSelector>,
}

/// WeightedPodAffinityTerm represents a weighted affinity term.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeightedPodAffinityTerm {
    /// Weight associated with matching the corresponding podAffinityTerm.
    pub weight: i32,

    /// A pod affinity term, associated with the corresponding weight.
    pub pod_affinity_term: PodAffinityTerm,
}

/// Toleration represents the toleration object.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Toleration {
    /// Key is the taint key that the toleration applies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// Operator represents a key's relationship to the value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,

    /// Value is the taint value the toleration matches to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Effect indicates the taint effect to match.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,

    /// TolerationSeconds represents the period of time the toleration tolerates the taint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub toleration_seconds: Option<i64>,
}

/// HostAlias holds the mapping between IP and hostnames.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostAlias {
    /// IP address of the host file entry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// Hostnames for the above IP address.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hostnames: Vec<String>,
}

/// PodDNSConfig defines the DNS parameters of a pod.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodDNSConfig {
    /// A list of DNS name server IP addresses.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nameservers: Vec<String>,

    /// A list of DNS search domains for host-name lookup.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub searches: Vec<String>,

    /// A list of DNS resolver options.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<PodDNSConfigOption>,
}

/// PodDNSConfigOption defines DNS resolver options.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodDNSConfigOption {
    /// Required.
    pub name: String,

    /// Value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// TopologySpreadConstraint specifies how to spread matching pods among the given topology.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopologySpreadConstraint {
    /// MaxSkew describes the degree to which pods may be unevenly distributed.
    pub max_skew: i32,

    /// TopologyKey is the key of node labels.
    pub topology_key: String,

    /// WhenUnsatisfiable indicates how to deal with a pod if it doesn't satisfy the spread constraint.
    pub when_unsatisfiable: String,

    /// LabelSelector is used to find matching pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<LabelSelector>,

    /// MinDomains indicates a minimum number of eligible domains.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_domains: Option<i32>,

    /// NodeAffinityPolicy indicates how we will treat Pod's nodeAffinity/nodeSelector when calculating pod topology spread skew.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_affinity_policy: Option<String>,

    /// NodeTaintsPolicy indicates how we will treat node taints when calculating pod topology spread skew.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_taints_policy: Option<String>,

    /// MatchLabelKeys is a set of pod label keys to select the pods over which spreading will be calculated.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_label_keys: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::container::Container;

    #[test]
    fn test_pod_new() {
        let pod = Pod::new("my-pod");
        assert_eq!(pod.type_meta.api_version, "v1");
        assert_eq!(pod.type_meta.kind, "Pod");
        assert_eq!(
            pod.metadata.as_ref().and_then(|m| m.name.as_ref()),
            Some(&"my-pod".to_string())
        );
    }

    #[test]
    fn test_pod_namespaced() {
        let pod = Pod::namespaced("my-pod", "production");
        assert_eq!(
            pod.metadata.as_ref().and_then(|m| m.namespace.as_ref()),
            Some(&"production".to_string())
        );
    }

    #[test]
    fn test_pod_resource_trait() {
        assert_eq!(Pod::API_VERSION, "v1");
        assert_eq!(Pod::GROUP, "");
        assert_eq!(Pod::KIND, "Pod");
        assert_eq!(Pod::PLURAL, "pods");
    }

    #[test]
    fn test_pod_serialize() {
        let pod = Pod {
            type_meta: TypeMeta::new("v1", "Pod"),
            metadata: Some(ObjectMeta::namespaced("nginx", "default")),
            spec: Some(PodSpec::single_container(Container::with_image(
                "nginx",
                "nginx:1.21",
            ))),
            status: None,
        };

        let json = serde_json::to_string(&pod).unwrap();
        assert!(json.contains("\"apiVersion\":\"v1\""));
        assert!(json.contains("\"kind\":\"Pod\""));
        assert!(json.contains("\"name\":\"nginx\""));
        assert!(json.contains("\"image\":\"nginx:1.21\""));
    }

    #[test]
    fn test_pod_roundtrip() {
        let original = Pod {
            type_meta: TypeMeta::new("v1", "Pod"),
            metadata: Some(ObjectMeta::named("test-pod")),
            spec: Some(PodSpec {
                containers: vec![Container::with_image("app", "busybox")],
                restart_policy: Some("Always".to_string()),
                ..Default::default()
            }),
            status: None,
        };

        let json = serde_json::to_string(&original).unwrap();
        let parsed: Pod = serde_json::from_str(&json).unwrap();

        assert_eq!(original.type_meta, parsed.type_meta);
        assert_eq!(original.metadata, parsed.metadata);
    }

    #[test]
    fn test_pod_spec_single_container() {
        let spec = PodSpec::single_container(Container::new("app"));
        assert_eq!(spec.containers.len(), 1);
        assert_eq!(spec.containers[0].name, "app");
    }

    #[test]
    fn test_volume_helpers() {
        let empty_dir = Volume::empty_dir("data");
        assert!(empty_dir.volume_source.empty_dir.is_some());

        let config = Volume::config_map("config", "app-config");
        assert!(config.volume_source.config_map.is_some());

        let secret = Volume::secret("certs", "tls-secret");
        assert!(secret.volume_source.secret.is_some());
    }

    #[test]
    fn test_pod_condition() {
        let condition = PodCondition {
            type_: "Ready".to_string(),
            status: "True".to_string(),
            reason: Some("ContainersReady".to_string()),
            message: Some("All containers are ready".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&condition).unwrap();
        let parsed: PodCondition = serde_json::from_str(&json).unwrap();
        assert_eq!(condition, parsed);
    }

    #[test]
    fn test_container_status() {
        let status = ContainerStatus {
            name: "app".to_string(),
            ready: true,
            restart_count: 0,
            state: Some(ContainerState {
                running: Some(ContainerStateRunning { started_at: None }),
                ..Default::default()
            }),
            ..Default::default()
        };

        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"ready\":true"));
    }

    #[test]
    fn test_toleration() {
        let toleration = Toleration {
            key: Some("node.kubernetes.io/not-ready".to_string()),
            operator: Some("Exists".to_string()),
            effect: Some("NoExecute".to_string()),
            toleration_seconds: Some(300),
            ..Default::default()
        };

        let json = serde_json::to_string(&toleration).unwrap();
        let parsed: Toleration = serde_json::from_str(&json).unwrap();
        assert_eq!(toleration, parsed);
    }
}
