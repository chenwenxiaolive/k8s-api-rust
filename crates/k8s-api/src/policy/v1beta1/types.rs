//! Policy v1beta1 API type definitions (deprecated)
//!
//! This module provides deprecated beta types for backwards compatibility.

use k8s_apimachinery::apis::meta::v1::{LabelSelector, ListMeta, ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

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

// =============================================================================
// PodSecurityPolicy (removed in K8s 1.25, but kept for backwards compatibility)
// =============================================================================

/// PodSecurityPolicy governs the ability to make requests that affect the Security Context.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodSecurityPolicy {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PodSecurityPolicySpec>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodSecurityPolicySpec {
    /// privileged determines if a pod can request to be run as privileged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// defaultAddCapabilities is the default set of capabilities that will be added.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub default_add_capabilities: Vec<String>,
    /// requiredDropCapabilities are the capabilities that will be dropped.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_drop_capabilities: Vec<String>,
    /// allowedCapabilities is a list of capabilities that can be requested.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_capabilities: Vec<String>,
    /// volumes is a list of allowed volume types.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes: Vec<String>,
    /// hostNetwork determines if the policy allows the use of HostNetwork.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_network: Option<bool>,
    /// hostPorts determines which host port ranges are allowed.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub host_ports: Vec<HostPortRange>,
    /// hostPID determines if the policy allows the use of HostPID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_pid: Option<bool>,
    /// hostIPC determines if the policy allows the use of HostIPC.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_ipc: Option<bool>,
    /// seLinux is the strategy that will dictate the allowable labels.
    pub se_linux: SELinuxStrategyOptions,
    /// runAsUser is the strategy that will dictate the allowable RunAsUser values.
    pub run_as_user: RunAsUserStrategyOptions,
    /// runAsGroup is the strategy that will dictate the allowable RunAsGroup values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_group: Option<RunAsGroupStrategyOptions>,
    /// supplementalGroups is the strategy that will dictate what supplemental groups.
    pub supplemental_groups: SupplementalGroupsStrategyOptions,
    /// fsGroup is the strategy that will dictate what fs group is used.
    pub fs_group: FSGroupStrategyOptions,
    /// readOnlyRootFilesystem when set to true will force containers to run with read-only root file system.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only_root_filesystem: Option<bool>,
    /// allowPrivilegeEscalation determines if a pod can request to allow privilege escalation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_privilege_escalation: Option<bool>,
    /// defaultAllowPrivilegeEscalation controls the default setting.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_allow_privilege_escalation: Option<bool>,
    /// allowedHostPaths is a list of host paths that are allowed to be used.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_host_paths: Vec<AllowedHostPath>,
    /// allowedFlexVolumes is a list of allowed Flexvolumes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_flex_volumes: Vec<AllowedFlexVolume>,
    /// allowedCSIDrivers is a list of CSI drivers that are allowed to be used.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_csi_drivers: Vec<AllowedCSIDriver>,
    /// allowedUnsafeSysctls is a list of explicitly allowed unsafe sysctls.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_unsafe_sysctls: Vec<String>,
    /// forbiddenSysctls is a list of sysctls that are explicitly forbidden.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub forbidden_sysctls: Vec<String>,
    /// allowedProcMountTypes is a list of allowed ProcMountTypes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_proc_mount_types: Vec<String>,
    /// runtimeClass is the strategy that will dictate the allowable RuntimeClasses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_class: Option<RuntimeClassStrategyOptions>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostPortRange {
    pub min: i32,
    pub max: i32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SELinuxStrategyOptions {
    pub rule: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub se_linux_options: Option<crate::core::v1::SELinuxOptions>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunAsUserStrategyOptions {
    pub rule: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ranges: Vec<IDRange>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunAsGroupStrategyOptions {
    pub rule: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ranges: Vec<IDRange>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupplementalGroupsStrategyOptions {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub rule: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ranges: Vec<IDRange>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FSGroupStrategyOptions {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub rule: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ranges: Vec<IDRange>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IDRange {
    pub min: i64,
    pub max: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowedHostPath {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path_prefix: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowedFlexVolume {
    pub driver: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowedCSIDriver {
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeClassStrategyOptions {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_runtime_class_names: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_runtime_class_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodSecurityPolicyList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    pub items: Vec<PodSecurityPolicy>,
}

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
    pub delete_options: Option<DeleteOptions>,
}

/// DeleteOptions may be provided when deleting an API object.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grace_period_seconds: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preconditions: Option<Preconditions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orphan_dependents: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub propagation_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dry_run: Vec<String>,
}

/// Preconditions must be fulfilled before an operation (update, delete, etc.) is carried out.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preconditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<String>,
}
