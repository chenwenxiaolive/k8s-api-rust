//! Container-related types for Kubernetes core/v1 API.
//!
//! This module contains types related to containers, including Container,
//! ContainerPort, EnvVar, VolumeMount, Probe, and related types.

use crate::common::{ConfigMapKeySelector, LocalObjectReference, SecretKeySelector};
use crate::resource::ResourceRequirements;
use k8s_api_core::IntOrString;
use serde::{Deserialize, Serialize};

/// Protocol defines network protocols supported for things like container ports.
pub type Protocol = String;

/// Protocol constants.
pub mod protocol {
    /// TCP protocol.
    pub const TCP: &str = "TCP";
    /// UDP protocol.
    pub const UDP: &str = "UDP";
    /// SCTP protocol.
    pub const SCTP: &str = "SCTP";
}

/// PullPolicy describes a policy for if/when to pull a container image.
pub type PullPolicy = String;

/// Pull policy constants.
pub mod pull_policy {
    /// Always pull the image.
    pub const ALWAYS: &str = "Always";
    /// Never pull the image.
    pub const NEVER: &str = "Never";
    /// Pull only if not present.
    pub const IF_NOT_PRESENT: &str = "IfNotPresent";
}

/// TerminationMessagePolicy describes how termination messages are retrieved from a container.
pub type TerminationMessagePolicy = String;

/// Termination message policy constants.
pub mod termination_message_policy {
    /// Use the contents of terminationMessagePath.
    pub const FILE: &str = "File";
    /// Use the last chunk of container log output if the termination message file is empty.
    pub const FALLBACK_TO_LOGS_ON_ERROR: &str = "FallbackToLogsOnError";
}

/// ContainerRestartPolicy defines the restart policy for individual containers.
pub type ContainerRestartPolicy = String;

/// Container restart policy constants.
pub mod container_restart_policy {
    /// Always restart the container.
    pub const ALWAYS: &str = "Always";
}

/// Container represents a single container within a pod.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Container {
    /// Name of the container specified as a DNS_LABEL.
    /// Each container in a pod must have a unique name.
    pub name: String,

    /// Container image name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    /// Entrypoint array. Not executed within a shell.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,

    /// Arguments to the entrypoint.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,

    /// Container's working directory.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,

    /// List of ports to expose from the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<ContainerPort>,

    /// List of sources to populate environment variables in the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env_from: Vec<EnvFromSource>,

    /// List of environment variables to set in the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<EnvVar>,

    /// Compute Resources required by this container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceRequirements>,

    /// Resources resize policy for the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resize_policy: Vec<ContainerResizePolicy>,

    /// RestartPolicy defines the restart behavior of individual containers in a pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<ContainerRestartPolicy>,

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

    /// Actions that the management system should take in response to container lifecycle events.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,

    /// Path at which the file to which the container's termination message will be written.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termination_message_path: Option<String>,

    /// Indicate how the termination message should be populated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termination_message_policy: Option<TerminationMessagePolicy>,

    /// Image pull policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_pull_policy: Option<PullPolicy>,

    /// SecurityContext defines the security options the container should be run with.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_context: Option<SecurityContext>,

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

impl Container {
    /// Creates a new Container with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Creates a new Container with the given name and image.
    pub fn with_image(name: impl Into<String>, image: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            image: Some(image.into()),
            ..Default::default()
        }
    }
}

/// ContainerPort represents a network port in a single container.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerPort {
    /// If specified, this must be an IANA_SVC_NAME and unique within the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Number of port to expose on the host.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_port: Option<i32>,

    /// Number of port to expose on the pod's IP address.
    pub container_port: i32,

    /// Protocol for port. Must be UDP, TCP, or SCTP. Defaults to "TCP".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,

    /// What host IP to bind the external port to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_ip: Option<String>,
}

impl ContainerPort {
    /// Creates a new ContainerPort with the given port number.
    pub fn new(port: i32) -> Self {
        Self {
            container_port: port,
            ..Default::default()
        }
    }

    /// Creates a new named ContainerPort.
    pub fn named(name: impl Into<String>, port: i32) -> Self {
        Self {
            name: Some(name.into()),
            container_port: port,
            ..Default::default()
        }
    }
}

/// EnvVar represents an environment variable present in a Container.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvVar {
    /// Name of the environment variable.
    pub name: String,

    /// Variable references $(VAR_NAME) are expanded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Source for the environment variable's value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_from: Option<EnvVarSource>,
}

impl EnvVar {
    /// Creates a new EnvVar with the given name and value.
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: Some(value.into()),
            value_from: None,
        }
    }

    /// Creates a new EnvVar that gets its value from a source.
    pub fn from_source(name: impl Into<String>, source: EnvVarSource) -> Self {
        Self {
            name: name.into(),
            value: None,
            value_from: Some(source),
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

    /// Selects a key of a secret in the pod's namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_key_ref: Option<SecretKeySelector>,
}

/// ObjectFieldSelector selects an APIVersioned field of an object.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectFieldSelector {
    /// Version of the schema the FieldPath is written in terms of.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Path of the field to select in the specified API version.
    pub field_path: String,
}

/// ResourceFieldSelector represents container resources (cpu, memory) and their output format.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceFieldSelector {
    /// Container name: required for volumes, optional for env vars.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,

    /// Required: resource to select.
    pub resource: String,

    /// Specifies the output format of the exposed resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<k8s_api_core::Quantity>,
}

/// EnvFromSource represents the source of a set of ConfigMaps or Secrets.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvFromSource {
    /// An optional identifier to prepend to each key in the ConfigMap or Secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    /// The ConfigMap to select from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_map_ref: Option<ConfigMapEnvSource>,

    /// The Secret to select from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<SecretEnvSource>,
}

/// ConfigMapEnvSource selects a ConfigMap to populate environment variables.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapEnvSource {
    /// The ConfigMap to select from.
    #[serde(flatten)]
    pub local_object_reference: LocalObjectReference,

    /// Specify whether the ConfigMap must be defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// SecretEnvSource selects a Secret to populate environment variables.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretEnvSource {
    /// The Secret to select from.
    #[serde(flatten)]
    pub local_object_reference: LocalObjectReference,

    /// Specify whether the Secret must be defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// VolumeMount describes a mounting of a Volume within a container.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeMount {
    /// This must match the Name of a Volume.
    pub name: String,

    /// Mounted read-only if true, read-write otherwise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    /// Path within the container at which the volume should be mounted.
    pub mount_path: String,

    /// Path within the volume from which the container's volume should be mounted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_path: Option<String>,

    /// Expanded path within the volume from which the container's volume should be mounted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_path_expr: Option<String>,

    /// mountPropagation determines how mounts are propagated from the host to container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mount_propagation: Option<String>,
}

impl VolumeMount {
    /// Creates a new VolumeMount.
    pub fn new(name: impl Into<String>, mount_path: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            mount_path: mount_path.into(),
            ..Default::default()
        }
    }

    /// Creates a read-only VolumeMount.
    pub fn read_only(name: impl Into<String>, mount_path: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            mount_path: mount_path.into(),
            read_only: Some(true),
            ..Default::default()
        }
    }
}

/// VolumeDevice describes a mapping of a raw block device within a container.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeDevice {
    /// name must match the name of a persistentVolumeClaim in the pod.
    pub name: String,

    /// devicePath is the path inside of the container that the device will be mapped to.
    pub device_path: String,
}

/// ContainerResizePolicy represents resource resize policy for a container.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerResizePolicy {
    /// Name of the resource to which this resource resize policy applies.
    pub resource_name: String,

    /// Restart policy to apply when specified resource is resized.
    pub restart_policy: String,
}

/// Probe describes a health check to be performed against a container.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Probe {
    /// The action taken to determine the health of a container.
    #[serde(flatten)]
    pub handler: ProbeHandler,

    /// Number of seconds after the container has started before probes are initiated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_delay_seconds: Option<i32>,

    /// Number of seconds after which the probe times out. Defaults to 1 second.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,

    /// How often (in seconds) to perform the probe. Defaults to 10 seconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_seconds: Option<i32>,

    /// Minimum consecutive successes for the probe to be considered successful.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success_threshold: Option<i32>,

    /// Minimum consecutive failures for the probe to be considered failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i32>,

    /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termination_grace_period_seconds: Option<i64>,
}

/// ProbeHandler defines a specific action that should be taken.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProbeHandler {
    /// Exec specifies the action to take.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<ExecAction>,

    /// HTTPGet specifies the http request to perform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_get: Option<HTTPGetAction>,

    /// TCPSocket specifies an action involving a TCP port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp_socket: Option<TCPSocketAction>,

    /// GRPC specifies an action involving a GRPC port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grpc: Option<GRPCAction>,
}

/// ExecAction describes a "run in container" action.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecAction {
    /// Command is the command line to execute inside the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
}

/// HTTPGetAction describes an action based on HTTP Get requests.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HTTPGetAction {
    /// Path to access on the HTTP server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Name or number of the port to access on the container.
    pub port: IntOrString,

    /// Host name to connect to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// Scheme to use for connecting to the host. Defaults to HTTP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,

    /// Custom headers to set in the request.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub http_headers: Vec<HTTPHeader>,
}

/// HTTPHeader describes a custom header to be used in HTTP probes.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HTTPHeader {
    /// The header field name.
    pub name: String,

    /// The header field value.
    pub value: String,
}

/// TCPSocketAction describes an action based on opening a socket.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TCPSocketAction {
    /// Number or name of the port to access on the container.
    pub port: IntOrString,

    /// Host name to connect to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}

/// GRPCAction describes an action based on GRPC health checks.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GRPCAction {
    /// Port number of the gRPC service.
    pub port: i32,

    /// Service is the name of the service to place in the gRPC HealthCheckRequest.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

/// Lifecycle describes actions that the management system should take in response
/// to container lifecycle events.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lifecycle {
    /// PostStart is called immediately after a container is created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_start: Option<LifecycleHandler>,

    /// PreStop is called immediately before a container is terminated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_stop: Option<LifecycleHandler>,
}

/// LifecycleHandler defines a specific action that should be taken in a lifecycle hook.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifecycleHandler {
    /// Exec specifies the action to take.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<ExecAction>,

    /// HTTPGet specifies the http request to perform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_get: Option<HTTPGetAction>,

    /// TCPSocket specifies an action involving a TCP port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp_socket: Option<TCPSocketAction>,

    /// Sleep represents the duration that the container should sleep before being terminated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sleep: Option<SleepAction>,
}

/// SleepAction represents a sleep action.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SleepAction {
    /// Seconds is the number of seconds to sleep.
    pub seconds: i64,
}

/// SecurityContext holds security configuration for a container.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityContext {
    /// The capabilities to add/drop when running containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,

    /// Run container in privileged mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,

    /// The SELinux context to be applied to the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub se_linux_options: Option<SELinuxOptions>,

    /// The Windows specific settings applied to all containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windows_options: Option<WindowsSecurityContextOptions>,

    /// The UID to run the entrypoint of the container process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_user: Option<i64>,

    /// The GID to run the entrypoint of the container process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_group: Option<i64>,

    /// Indicates that the container must run as a non-root user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_non_root: Option<bool>,

    /// Whether this container has a read-only root filesystem.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only_root_filesystem: Option<bool>,

    /// AllowPrivilegeEscalation controls whether a process can gain more privileges than its parent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_privilege_escalation: Option<bool>,

    /// The type of proc mount to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proc_mount: Option<String>,

    /// The seccomp options to use by this container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seccomp_profile: Option<SeccompProfile>,

    /// The AppArmor options to use by this container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_armor_profile: Option<AppArmorProfile>,
}

/// Capabilities represents POSIX capabilities to add/drop.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    /// Added capabilities.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add: Vec<String>,

    /// Removed capabilities.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub drop: Vec<String>,
}

/// SELinuxOptions are the labels to be applied to the container.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SELinuxOptions {
    /// User is the SELinux user label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// Role is the SELinux role label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    /// Type is the SELinux type label.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,

    /// Level is the SELinux level label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
}

/// WindowsSecurityContextOptions are Windows-specific security settings.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowsSecurityContextOptions {
    /// The name of the GMSA credential spec to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gmsa_credential_spec_name: Option<String>,

    /// The contents of the GMSA credential spec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gmsa_credential_spec: Option<String>,

    /// The user name in Windows to run the entrypoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_user_name: Option<String>,

    /// Sets the host process container attribute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_process: Option<bool>,
}

/// SeccompProfile defines a pod/container's seccomp profile settings.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeccompProfile {
    /// Type indicates which kind of seccomp profile will be applied.
    #[serde(rename = "type")]
    pub type_: String,

    /// localhostProfile indicates a profile defined in a file on the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localhost_profile: Option<String>,
}

/// AppArmorProfile defines a pod/container's AppArmor profile settings.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppArmorProfile {
    /// Type indicates which kind of AppArmor profile will be applied.
    #[serde(rename = "type")]
    pub type_: String,

    /// localhostProfile indicates a profile loaded on the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localhost_profile: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_new() {
        let container = Container::new("app");
        assert_eq!(container.name, "app");
        assert!(container.image.is_none());
    }

    #[test]
    fn test_container_with_image() {
        let container = Container::with_image("app", "nginx:latest");
        assert_eq!(container.name, "app");
        assert_eq!(container.image, Some("nginx:latest".to_string()));
    }

    #[test]
    fn test_container_serialize() {
        let container = Container {
            name: "web".to_string(),
            image: Some("nginx:1.21".to_string()),
            ports: vec![ContainerPort::new(80)],
            ..Default::default()
        };

        let json = serde_json::to_string(&container).unwrap();
        assert!(json.contains("\"name\":\"web\""));
        assert!(json.contains("\"image\":\"nginx:1.21\""));
        assert!(json.contains("\"containerPort\":80"));
    }

    #[test]
    fn test_container_roundtrip() {
        let original = Container {
            name: "app".to_string(),
            image: Some("busybox".to_string()),
            command: vec!["sh".to_string(), "-c".to_string()],
            args: vec!["echo hello".to_string()],
            env: vec![EnvVar::new("FOO", "bar")],
            ..Default::default()
        };

        let json = serde_json::to_string(&original).unwrap();
        let parsed: Container = serde_json::from_str(&json).unwrap();

        assert_eq!(original.name, parsed.name);
        assert_eq!(original.image, parsed.image);
        assert_eq!(original.command, parsed.command);
        assert_eq!(original.args, parsed.args);
    }

    #[test]
    fn test_env_var() {
        let env = EnvVar::new("DATABASE_URL", "postgres://localhost/db");
        let json = serde_json::to_string(&env).unwrap();
        assert!(json.contains("\"name\":\"DATABASE_URL\""));
        assert!(json.contains("\"value\":\"postgres://localhost/db\""));
    }

    #[test]
    fn test_volume_mount() {
        let mount = VolumeMount::read_only("config", "/etc/config");
        let json = serde_json::to_string(&mount).unwrap();
        let parsed: VolumeMount = serde_json::from_str(&json).unwrap();
        assert_eq!(mount, parsed);
        assert_eq!(mount.read_only, Some(true));
    }

    #[test]
    fn test_probe_with_http_get() {
        let probe = Probe {
            handler: ProbeHandler {
                http_get: Some(HTTPGetAction {
                    path: Some("/health".to_string()),
                    port: IntOrString::Int(8080),
                    ..Default::default()
                }),
                ..Default::default()
            },
            initial_delay_seconds: Some(10),
            period_seconds: Some(5),
            ..Default::default()
        };

        let json = serde_json::to_string(&probe).unwrap();
        assert!(json.contains("\"path\":\"/health\""));
        assert!(json.contains("\"port\":8080"));
    }

    #[test]
    fn test_security_context() {
        let ctx = SecurityContext {
            run_as_non_root: Some(true),
            read_only_root_filesystem: Some(true),
            capabilities: Some(Capabilities {
                drop: vec!["ALL".to_string()],
                ..Default::default()
            }),
            ..Default::default()
        };

        let json = serde_json::to_string(&ctx).unwrap();
        let parsed: SecurityContext = serde_json::from_str(&json).unwrap();
        assert_eq!(ctx, parsed);
    }
}
