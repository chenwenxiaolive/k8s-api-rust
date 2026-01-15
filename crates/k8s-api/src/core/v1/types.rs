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

pub type ConditionStatus = String;
pub type PodConditionType = String;
pub type Protocol = String;
pub type ResourceName = String;
pub type ResourceList = BTreeMap<ResourceName, Quantity>;
pub type RestartPolicy = String;
pub type DNSPolicy = String;
pub type PodPhase = String;
pub type PodQOSClass = String;
pub type PullPolicy = String;
pub type TerminationMessagePolicy = String;
pub type PreemptionPolicy = String;
pub type ServiceAffinity = String;
pub type ServiceType = String;
pub type ServiceExternalTrafficPolicy = String;
pub type ServiceInternalTrafficPolicy = String;
pub type ServiceInternalTrafficPolicyType = ServiceInternalTrafficPolicy;
pub type ServiceExternalTrafficPolicyType = ServiceExternalTrafficPolicy;
pub type LoadBalancerIPMode = String;
pub type IPFamily = String;
pub type IPFamilyPolicy = String;
pub type IPFamilyPolicyType = IPFamilyPolicy;
pub type ServiceTrafficDistribution = String;
pub type PersistentVolumeReclaimPolicy = String;
pub type PersistentVolumeAccessMode = String;
pub type PersistentVolumePhase = String;
pub type PersistentVolumeClaimPhase = String;
pub type PersistentVolumeClaimConditionType = String;
pub type PersistentVolumeClaimModifyVolumeStatus = String;
pub type PodResizeStatus = String;
pub type PersistentVolumeMode = String;
pub type ResourceResizeRestartPolicy = String;
pub type ContainerRestartPolicy = String;
pub type ContainerRestartRuleAction = String;
pub type ContainerRestartRuleOnExitCodesOperator = String;
pub type TaintEffect = String;
pub type TolerationOperator = String;
pub type UnsatisfiableConstraintAction = String;
pub type NodeInclusionPolicy = String;
pub type PodFSGroupChangePolicy = String;
pub type SupplementalGroupsPolicy = String;
pub type PodSELinuxChangePolicy = String;
pub type SeccompProfileType = String;
pub type AppArmorProfileType = String;
pub type HostPathType = String;
pub type StorageMedium = String;
pub type AzureDataDiskCachingMode = String;
pub type AzureDataDiskKind = String;
pub type MountPropagationMode = String;
pub type RecursiveReadOnlyMode = String;
pub type URIScheme = String;
pub type NodeSelectorOperator = String;
pub type OSName = String;
pub type ClaimResourceStatus = String;
pub type ResourceHealthStatus = String;
pub type ResourceID = String;
pub type Signal = String;
pub type UniqueVolumeName = String;
pub type NodePhase = String;
pub type NodeConditionType = String;
pub type NodeAddressType = String;
pub type FinalizerName = String;
pub type NamespacePhase = String;
pub type NamespaceConditionType = String;
pub type LimitType = String;
pub type ResourceQuotaScope = String;
pub type ScopeSelectorOperator = String;
pub type SecretType = String;
pub type ReplicationControllerConditionType = String;
pub type ComponentConditionType = String;
pub type ProcMountType = String;
pub type Capability = String;

// Condition status constants
pub const CONDITION_TRUE: &str = "True";
pub const CONDITION_FALSE: &str = "False";
pub const CONDITION_UNKNOWN: &str = "Unknown";

// Pod condition type constants
pub const POD_CONDITION_CONTAINERS_READY: &str = "ContainersReady";
pub const POD_CONDITION_INITIALIZED: &str = "Initialized";
pub const POD_CONDITION_READY: &str = "Ready";
pub const POD_CONDITION_SCHEDULED: &str = "PodScheduled";
pub const POD_CONDITION_DISRUPTION_TARGET: &str = "DisruptionTarget";
pub const POD_CONDITION_READY_TO_START_CONTAINERS: &str = "PodReadyToStartContainers";
pub const POD_CONDITION_RESIZE_PENDING: &str = "PodResizePending";
pub const POD_CONDITION_RESIZE_IN_PROGRESS: &str = "PodResizeInProgress";

// Pod condition reason constants
pub const POD_REASON_UNSCHEDULABLE: &str = "Unschedulable";
pub const POD_REASON_SCHEDULING_GATED: &str = "SchedulingGated";
pub const POD_REASON_SCHEDULER_ERROR: &str = "SchedulerError";
pub const POD_REASON_TERMINATION_BY_KUBELET: &str = "TerminationByKubelet";
pub const POD_REASON_PREEMPTION_BY_SCHEDULER: &str = "PreemptionByScheduler";
pub const POD_REASON_DEFERRED: &str = "Deferred";
pub const POD_REASON_INFEASIBLE: &str = "Infeasible";
pub const POD_REASON_ERROR: &str = "Error";

// Protocol constants
pub const PROTOCOL_TCP: &str = "TCP";
pub const PROTOCOL_UDP: &str = "UDP";
pub const PROTOCOL_SCTP: &str = "SCTP";

// RestartPolicy constants
pub const RESTART_POLICY_ALWAYS: &str = "Always";
pub const RESTART_POLICY_ON_FAILURE: &str = "OnFailure";
pub const RESTART_POLICY_NEVER: &str = "Never";

// DNSPolicy constants
pub const DNS_POLICY_CLUSTER_FIRST_WITH_HOST_NET: &str = "ClusterFirstWithHostNet";
pub const DNS_POLICY_CLUSTER_FIRST: &str = "ClusterFirst";
pub const DNS_POLICY_DEFAULT: &str = "Default";
pub const DNS_POLICY_NONE: &str = "None";

// PodPhase constants
pub const POD_PHASE_PENDING: &str = "Pending";
pub const POD_PHASE_RUNNING: &str = "Running";
pub const POD_PHASE_SUCCEEDED: &str = "Succeeded";
pub const POD_PHASE_FAILED: &str = "Failed";
pub const POD_PHASE_UNKNOWN: &str = "Unknown";

// PodQOSClass constants
pub const POD_QOS_GUARANTEED: &str = "Guaranteed";
pub const POD_QOS_BURSTABLE: &str = "Burstable";
pub const POD_QOS_BEST_EFFORT: &str = "BestEffort";

// PullPolicy constants
pub const PULL_ALWAYS: &str = "Always";
pub const PULL_NEVER: &str = "Never";
pub const PULL_IF_NOT_PRESENT: &str = "IfNotPresent";

// ResourceResizeRestartPolicy constants
pub const RESOURCE_RESIZE_RESTART_POLICY_NOT_REQUIRED: &str = "NotRequired";
pub const RESOURCE_RESIZE_RESTART_POLICY_RESTART_CONTAINER: &str = "RestartContainer";

// TerminationMessagePolicy constants
pub const TERMINATION_MESSAGE_READ_FILE: &str = "File";
pub const TERMINATION_MESSAGE_FALLBACK_TO_LOGS_ON_ERROR: &str = "FallbackToLogsOnError";

// PreemptionPolicy constants
pub const PREEMPT_LOWER_PRIORITY: &str = "PreemptLowerPriority";
pub const PREEMPT_NEVER: &str = "Never";

// ContainerRestartPolicy constants
pub const CONTAINER_RESTART_POLICY_ALWAYS: &str = "Always";
pub const CONTAINER_RESTART_POLICY_NEVER: &str = "Never";
pub const CONTAINER_RESTART_POLICY_ON_FAILURE: &str = "OnFailure";

// ContainerRestartRuleAction constants
pub const CONTAINER_RESTART_RULE_ACTION_RESTART: &str = "Restart";

// ContainerRestartRuleOnExitCodesOperator constants
pub const CONTAINER_RESTART_RULE_ON_EXIT_CODES_OP_IN: &str = "In";
pub const CONTAINER_RESTART_RULE_ON_EXIT_CODES_OP_NOT_IN: &str = "NotIn";

// ServiceAffinity constants
pub const SERVICE_AFFINITY_CLIENT_IP: &str = "ClientIP";
pub const SERVICE_AFFINITY_NONE: &str = "None";

// ServiceType constants
pub const SERVICE_TYPE_CLUSTER_IP: &str = "ClusterIP";
pub const SERVICE_TYPE_NODE_PORT: &str = "NodePort";
pub const SERVICE_TYPE_LOAD_BALANCER: &str = "LoadBalancer";
pub const SERVICE_TYPE_EXTERNAL_NAME: &str = "ExternalName";

// ServiceExternalTrafficPolicy constants
pub const SERVICE_EXTERNAL_TRAFFIC_POLICY_CLUSTER: &str = "Cluster";
pub const SERVICE_EXTERNAL_TRAFFIC_POLICY_LOCAL: &str = "Local";

// ServiceInternalTrafficPolicy constants
pub const SERVICE_INTERNAL_TRAFFIC_POLICY_CLUSTER: &str = "Cluster";
pub const SERVICE_INTERNAL_TRAFFIC_POLICY_LOCAL: &str = "Local";

// ServiceTrafficDistribution constants
pub const SERVICE_TRAFFIC_DISTRIBUTION_PREFER_CLOSE: &str = "PreferClose";
pub const SERVICE_TRAFFIC_DISTRIBUTION_PREFER_SAME_ZONE: &str = "PreferSameZone";
pub const SERVICE_TRAFFIC_DISTRIBUTION_PREFER_SAME_NODE: &str = "PreferSameNode";

// LoadBalancerIPMode constants
pub const LOAD_BALANCER_IP_MODE_VIP: &str = "VIP";
pub const LOAD_BALANCER_IP_MODE_PROXY: &str = "Proxy";

// IPFamily constants
pub const IP_FAMILY_IPV4: &str = "IPv4";
pub const IP_FAMILY_IPV6: &str = "IPv6";
pub const IP_FAMILY_UNKNOWN: &str = "";

// IPFamilyPolicy constants
pub const IP_FAMILY_POLICY_SINGLE_STACK: &str = "SingleStack";
pub const IP_FAMILY_POLICY_PREFER_DUAL_STACK: &str = "PreferDualStack";
pub const IP_FAMILY_POLICY_REQUIRE_DUAL_STACK: &str = "RequireDualStack";

// PersistentVolumeReclaimPolicy constants
pub const PV_RECLAIM_RECYCLE: &str = "Recycle";
pub const PV_RECLAIM_DELETE: &str = "Delete";
pub const PV_RECLAIM_RETAIN: &str = "Retain";

// PersistentVolumeAccessMode constants
pub const PV_ACCESS_READ_WRITE_ONCE: &str = "ReadWriteOnce";
pub const PV_ACCESS_READ_ONLY_MANY: &str = "ReadOnlyMany";
pub const PV_ACCESS_READ_WRITE_MANY: &str = "ReadWriteMany";
pub const PV_ACCESS_READ_WRITE_ONCE_POD: &str = "ReadWriteOncePod";

// PersistentVolumePhase constants
pub const PV_PHASE_PENDING: &str = "Pending";
pub const PV_PHASE_AVAILABLE: &str = "Available";
pub const PV_PHASE_BOUND: &str = "Bound";
pub const PV_PHASE_RELEASED: &str = "Released";
pub const PV_PHASE_FAILED: &str = "Failed";

// PersistentVolumeClaimPhase constants
pub const PVC_PHASE_PENDING: &str = "Pending";
pub const PVC_PHASE_BOUND: &str = "Bound";
pub const PVC_PHASE_LOST: &str = "Lost";

// PersistentVolumeClaimConditionType constants
pub const PVC_CONDITION_RESIZING: &str = "Resizing";
pub const PVC_CONDITION_FILESYSTEM_RESIZE_PENDING: &str = "FileSystemResizePending";
pub const PVC_CONDITION_CONTROLLER_RESIZE_ERROR: &str = "ControllerResizeError";
pub const PVC_CONDITION_NODE_RESIZE_ERROR: &str = "NodeResizeError";
pub const PVC_CONDITION_MODIFY_VOLUME_ERROR: &str = "ModifyVolumeError";
pub const PVC_CONDITION_MODIFYING_VOLUME: &str = "ModifyingVolume";

// PersistentVolumeClaimModifyVolumeStatus constants
pub const PVC_MODIFY_VOLUME_STATUS_PENDING: &str = "Pending";
pub const PVC_MODIFY_VOLUME_STATUS_IN_PROGRESS: &str = "InProgress";
pub const PVC_MODIFY_VOLUME_STATUS_INFEASIBLE: &str = "Infeasible";

// PersistentVolumeMode constants
pub const PV_MODE_BLOCK: &str = "Block";
pub const PV_MODE_FILESYSTEM: &str = "Filesystem";

// TaintEffect constants
pub const TAINT_EFFECT_NO_SCHEDULE: &str = "NoSchedule";
pub const TAINT_EFFECT_PREFER_NO_SCHEDULE: &str = "PreferNoSchedule";
pub const TAINT_EFFECT_NO_EXECUTE: &str = "NoExecute";

// TolerationOperator constants
pub const TOLERATION_OP_EXISTS: &str = "Exists";
pub const TOLERATION_OP_EQUAL: &str = "Equal";

// UnsatisfiableConstraintAction constants
pub const UNSATISFIABLE_CONSTRAINT_ACTION_DO_NOT_SCHEDULE: &str = "DoNotSchedule";
pub const UNSATISFIABLE_CONSTRAINT_ACTION_SCHEDULE_ANYWAY: &str = "ScheduleAnyway";

// NodeInclusionPolicy constants
pub const NODE_INCLUSION_POLICY_IGNORE: &str = "Ignore";
pub const NODE_INCLUSION_POLICY_HONOR: &str = "Honor";

// PodFSGroupChangePolicy constants
pub const FS_GROUP_CHANGE_ON_ROOT_MISMATCH: &str = "OnRootMismatch";
pub const FS_GROUP_CHANGE_ALWAYS: &str = "Always";

// SupplementalGroupsPolicy constants
pub const SUPPLEMENTAL_GROUPS_POLICY_MERGE: &str = "Merge";
pub const SUPPLEMENTAL_GROUPS_POLICY_STRICT: &str = "Strict";

// PodSELinuxChangePolicy constants
pub const SELINUX_CHANGE_POLICY_RECURSIVE: &str = "Recursive";
pub const SELINUX_CHANGE_POLICY_MOUNT_OPTION: &str = "MountOption";

// SeccompProfileType constants
pub const SECCOMP_PROFILE_TYPE_UNCONFINED: &str = "Unconfined";
pub const SECCOMP_PROFILE_TYPE_RUNTIME_DEFAULT: &str = "RuntimeDefault";
pub const SECCOMP_PROFILE_TYPE_LOCALHOST: &str = "Localhost";

// AppArmorProfileType constants
pub const APP_ARMOR_PROFILE_TYPE_UNCONFINED: &str = "Unconfined";
pub const APP_ARMOR_PROFILE_TYPE_RUNTIME_DEFAULT: &str = "RuntimeDefault";
pub const APP_ARMOR_PROFILE_TYPE_LOCALHOST: &str = "Localhost";

// HostPathType constants
pub const HOST_PATH_UNSET: &str = "";
pub const HOST_PATH_DIRECTORY_OR_CREATE: &str = "DirectoryOrCreate";
pub const HOST_PATH_DIRECTORY: &str = "Directory";
pub const HOST_PATH_FILE_OR_CREATE: &str = "FileOrCreate";
pub const HOST_PATH_FILE: &str = "File";
pub const HOST_PATH_SOCKET: &str = "Socket";
pub const HOST_PATH_CHAR_DEVICE: &str = "CharDevice";
pub const HOST_PATH_BLOCK_DEVICE: &str = "BlockDevice";

// StorageMedium constants
pub const STORAGE_MEDIUM_DEFAULT: &str = "";
pub const STORAGE_MEDIUM_MEMORY: &str = "Memory";
pub const STORAGE_MEDIUM_HUGE_PAGES: &str = "HugePages";
pub const STORAGE_MEDIUM_HUGE_PAGES_PREFIX: &str = "HugePages-";

// AzureDataDiskCachingMode constants
pub const AZURE_DATA_DISK_CACHING_NONE: &str = "None";
pub const AZURE_DATA_DISK_CACHING_READ_ONLY: &str = "ReadOnly";
pub const AZURE_DATA_DISK_CACHING_READ_WRITE: &str = "ReadWrite";

// AzureDataDiskKind constants
pub const AZURE_DATA_DISK_KIND_SHARED: &str = "Shared";
pub const AZURE_DATA_DISK_KIND_DEDICATED: &str = "Dedicated";
pub const AZURE_DATA_DISK_KIND_MANAGED: &str = "Managed";

// MountPropagationMode constants
pub const MOUNT_PROPAGATION_NONE: &str = "None";
pub const MOUNT_PROPAGATION_HOST_TO_CONTAINER: &str = "HostToContainer";
pub const MOUNT_PROPAGATION_BIDIRECTIONAL: &str = "Bidirectional";

// RecursiveReadOnlyMode constants
pub const RECURSIVE_READ_ONLY_DISABLED: &str = "Disabled";
pub const RECURSIVE_READ_ONLY_IF_POSSIBLE: &str = "IfPossible";
pub const RECURSIVE_READ_ONLY_ENABLED: &str = "Enabled";

// URIScheme constants
pub const URI_SCHEME_HTTP: &str = "HTTP";
pub const URI_SCHEME_HTTPS: &str = "HTTPS";

// NodeSelectorOperator constants
pub const NODE_SELECTOR_OP_IN: &str = "In";
pub const NODE_SELECTOR_OP_NOT_IN: &str = "NotIn";
pub const NODE_SELECTOR_OP_EXISTS: &str = "Exists";
pub const NODE_SELECTOR_OP_DOES_NOT_EXIST: &str = "DoesNotExist";
pub const NODE_SELECTOR_OP_GT: &str = "Gt";
pub const NODE_SELECTOR_OP_LT: &str = "Lt";

// OSName constants
pub const OS_NAME_LINUX: &str = "linux";
pub const OS_NAME_WINDOWS: &str = "windows";

// ClaimResourceStatus constants
pub const PVC_RESOURCE_STATUS_CONTROLLER_RESIZE_IN_PROGRESS: &str = "ControllerResizeInProgress";
pub const PVC_RESOURCE_STATUS_CONTROLLER_RESIZE_INFEASIBLE: &str = "ControllerResizeInfeasible";
pub const PVC_RESOURCE_STATUS_NODE_RESIZE_PENDING: &str = "NodeResizePending";
pub const PVC_RESOURCE_STATUS_NODE_RESIZE_IN_PROGRESS: &str = "NodeResizeInProgress";
pub const PVC_RESOURCE_STATUS_NODE_RESIZE_INFEASIBLE: &str = "NodeResizeInfeasible";

// NodePhase constants
pub const NODE_PHASE_PENDING: &str = "Pending";
pub const NODE_PHASE_RUNNING: &str = "Running";
pub const NODE_PHASE_TERMINATED: &str = "Terminated";

// NodeConditionType constants
pub const NODE_CONDITION_READY: &str = "Ready";
pub const NODE_CONDITION_MEMORY_PRESSURE: &str = "MemoryPressure";
pub const NODE_CONDITION_DISK_PRESSURE: &str = "DiskPressure";
pub const NODE_CONDITION_PID_PRESSURE: &str = "PIDPressure";
pub const NODE_CONDITION_NETWORK_UNAVAILABLE: &str = "NetworkUnavailable";

// NodeAddressType constants
pub const NODE_ADDRESS_HOSTNAME: &str = "Hostname";
pub const NODE_ADDRESS_INTERNAL_IP: &str = "InternalIP";
pub const NODE_ADDRESS_EXTERNAL_IP: &str = "ExternalIP";
pub const NODE_ADDRESS_INTERNAL_DNS: &str = "InternalDNS";
pub const NODE_ADDRESS_EXTERNAL_DNS: &str = "ExternalDNS";

// FinalizerName constants
pub const FINALIZER_KUBERNETES: &str = "kubernetes";

// NamespacePhase constants
pub const NAMESPACE_PHASE_ACTIVE: &str = "Active";
pub const NAMESPACE_PHASE_TERMINATING: &str = "Terminating";

// NamespaceConditionType constants
pub const NAMESPACE_CONDITION_DELETION_DISCOVERY_FAILURE: &str = "NamespaceDeletionDiscoveryFailure";
pub const NAMESPACE_CONDITION_DELETION_CONTENT_FAILURE: &str = "NamespaceDeletionContentFailure";
pub const NAMESPACE_CONDITION_DELETION_GV_PARSING_FAILURE: &str = "NamespaceDeletionGroupVersionParsingFailure";
pub const NAMESPACE_CONDITION_CONTENT_REMAINING: &str = "NamespaceContentRemaining";
pub const NAMESPACE_CONDITION_FINALIZERS_REMAINING: &str = "NamespaceFinalizersRemaining";

// LimitType constants
pub const LIMIT_TYPE_POD: &str = "Pod";
pub const LIMIT_TYPE_CONTAINER: &str = "Container";
pub const LIMIT_TYPE_PERSISTENT_VOLUME_CLAIM: &str = "PersistentVolumeClaim";

// ResourceQuotaScope constants
pub const RESOURCE_QUOTA_SCOPE_TERMINATING: &str = "Terminating";
pub const RESOURCE_QUOTA_SCOPE_NOT_TERMINATING: &str = "NotTerminating";
pub const RESOURCE_QUOTA_SCOPE_BEST_EFFORT: &str = "BestEffort";
pub const RESOURCE_QUOTA_SCOPE_NOT_BEST_EFFORT: &str = "NotBestEffort";
pub const RESOURCE_QUOTA_SCOPE_PRIORITY_CLASS: &str = "PriorityClass";
pub const RESOURCE_QUOTA_SCOPE_CROSS_NAMESPACE_POD_AFFINITY: &str = "CrossNamespacePodAffinity";
pub const RESOURCE_QUOTA_SCOPE_VOLUME_ATTRIBUTES_CLASS: &str = "VolumeAttributesClass";

// ScopeSelectorOperator constants
pub const SCOPE_SELECTOR_OP_IN: &str = "In";
pub const SCOPE_SELECTOR_OP_NOT_IN: &str = "NotIn";
pub const SCOPE_SELECTOR_OP_EXISTS: &str = "Exists";
pub const SCOPE_SELECTOR_OP_DOES_NOT_EXIST: &str = "DoesNotExist";

// SecretType constants
pub const SECRET_TYPE_OPAQUE: &str = "Opaque";
pub const SECRET_TYPE_SERVICE_ACCOUNT_TOKEN: &str = "kubernetes.io/service-account-token";
pub const SECRET_TYPE_DOCKERCFG: &str = "kubernetes.io/dockercfg";
pub const SECRET_TYPE_DOCKER_CONFIG_JSON: &str = "kubernetes.io/dockerconfigjson";
pub const SECRET_TYPE_BASIC_AUTH: &str = "kubernetes.io/basic-auth";
pub const SECRET_TYPE_SSH_AUTH: &str = "kubernetes.io/ssh-auth";
pub const SECRET_TYPE_TLS: &str = "kubernetes.io/tls";
pub const SECRET_TYPE_BOOTSTRAP_TOKEN: &str = "bootstrap.kubernetes.io/token";

// Secret data keys
pub const SERVICE_ACCOUNT_NAME_KEY: &str = "kubernetes.io/service-account.name";
pub const SERVICE_ACCOUNT_UID_KEY: &str = "kubernetes.io/service-account.uid";
pub const SERVICE_ACCOUNT_TOKEN_KEY: &str = "token";
pub const SERVICE_ACCOUNT_KUBECONFIG_KEY: &str = "kubernetes.kubeconfig";
pub const SERVICE_ACCOUNT_ROOT_CA_KEY: &str = "ca.crt";
pub const SERVICE_ACCOUNT_NAMESPACE_KEY: &str = "namespace";
pub const DOCKER_CONFIG_KEY: &str = ".dockercfg";
pub const DOCKER_CONFIG_JSON_KEY: &str = ".dockerconfigjson";
pub const BASIC_AUTH_USERNAME_KEY: &str = "username";
pub const BASIC_AUTH_PASSWORD_KEY: &str = "password";
pub const SSH_AUTH_PRIVATE_KEY: &str = "ssh-privatekey";
pub const TLS_CERT_KEY: &str = "tls.crt";
pub const TLS_PRIVATE_KEY_KEY: &str = "tls.key";
pub const MAX_SECRET_SIZE: i32 = 1 * 1024 * 1024;

// ReplicationControllerConditionType constants
pub const REPLICATION_CONTROLLER_CONDITION_REPLICA_FAILURE: &str = "ReplicaFailure";

// ComponentConditionType constants
pub const COMPONENT_CONDITION_HEALTHY: &str = "Healthy";

// ProcMountType constants
pub const PROC_MOUNT_DEFAULT: &str = "Default";
pub const PROC_MOUNT_UNMASKED: &str = "Unmasked";

// Signal constants
pub const SIGNAL_SIGABRT: &str = "SIGABRT";
pub const SIGNAL_SIGALRM: &str = "SIGALRM";
pub const SIGNAL_SIGBUS: &str = "SIGBUS";
pub const SIGNAL_SIGCHLD: &str = "SIGCHLD";
pub const SIGNAL_SIGCLD: &str = "SIGCLD";
pub const SIGNAL_SIGCONT: &str = "SIGCONT";
pub const SIGNAL_SIGFPE: &str = "SIGFPE";
pub const SIGNAL_SIGHUP: &str = "SIGHUP";
pub const SIGNAL_SIGILL: &str = "SIGILL";
pub const SIGNAL_SIGINT: &str = "SIGINT";
pub const SIGNAL_SIGIO: &str = "SIGIO";
pub const SIGNAL_SIGIOT: &str = "SIGIOT";
pub const SIGNAL_SIGKILL: &str = "SIGKILL";
pub const SIGNAL_SIGPIPE: &str = "SIGPIPE";
pub const SIGNAL_SIGPOLL: &str = "SIGPOLL";
pub const SIGNAL_SIGPROF: &str = "SIGPROF";
pub const SIGNAL_SIGPWR: &str = "SIGPWR";
pub const SIGNAL_SIGQUIT: &str = "SIGQUIT";
pub const SIGNAL_SIGSEGV: &str = "SIGSEGV";
pub const SIGNAL_SIGSTKFLT: &str = "SIGSTKFLT";
pub const SIGNAL_SIGSTOP: &str = "SIGSTOP";
pub const SIGNAL_SIGSYS: &str = "SIGSYS";
pub const SIGNAL_SIGTERM: &str = "SIGTERM";
pub const SIGNAL_SIGTRAP: &str = "SIGTRAP";
pub const SIGNAL_SIGTSTP: &str = "SIGTSTP";
pub const SIGNAL_SIGTTIN: &str = "SIGTTIN";
pub const SIGNAL_SIGTTOU: &str = "SIGTTOU";
pub const SIGNAL_SIGURG: &str = "SIGURG";
pub const SIGNAL_SIGUSR1: &str = "SIGUSR1";
pub const SIGNAL_SIGUSR2: &str = "SIGUSR2";
pub const SIGNAL_SIGVTALRM: &str = "SIGVTALRM";
pub const SIGNAL_SIGWINCH: &str = "SIGWINCH";
pub const SIGNAL_SIGXCPU: &str = "SIGXCPU";
pub const SIGNAL_SIGXFSZ: &str = "SIGXFSZ";
pub const SIGNAL_SIGRTMIN: &str = "SIGRTMIN";
pub const SIGNAL_SIGRTMINPLUS1: &str = "SIGRTMIN+1";
pub const SIGNAL_SIGRTMINPLUS2: &str = "SIGRTMIN+2";
pub const SIGNAL_SIGRTMINPLUS3: &str = "SIGRTMIN+3";
pub const SIGNAL_SIGRTMINPLUS4: &str = "SIGRTMIN+4";
pub const SIGNAL_SIGRTMINPLUS5: &str = "SIGRTMIN+5";
pub const SIGNAL_SIGRTMINPLUS6: &str = "SIGRTMIN+6";
pub const SIGNAL_SIGRTMINPLUS7: &str = "SIGRTMIN+7";
pub const SIGNAL_SIGRTMINPLUS8: &str = "SIGRTMIN+8";
pub const SIGNAL_SIGRTMINPLUS9: &str = "SIGRTMIN+9";
pub const SIGNAL_SIGRTMINPLUS10: &str = "SIGRTMIN+10";
pub const SIGNAL_SIGRTMINPLUS11: &str = "SIGRTMIN+11";
pub const SIGNAL_SIGRTMINPLUS12: &str = "SIGRTMIN+12";
pub const SIGNAL_SIGRTMINPLUS13: &str = "SIGRTMIN+13";
pub const SIGNAL_SIGRTMINPLUS14: &str = "SIGRTMIN+14";
pub const SIGNAL_SIGRTMINPLUS15: &str = "SIGRTMIN+15";
pub const SIGNAL_SIGRTMAXMINUS14: &str = "SIGRTMAX-14";
pub const SIGNAL_SIGRTMAXMINUS13: &str = "SIGRTMAX-13";
pub const SIGNAL_SIGRTMAXMINUS12: &str = "SIGRTMAX-12";
pub const SIGNAL_SIGRTMAXMINUS11: &str = "SIGRTMAX-11";
pub const SIGNAL_SIGRTMAXMINUS10: &str = "SIGRTMAX-10";
pub const SIGNAL_SIGRTMAXMINUS9: &str = "SIGRTMAX-9";
pub const SIGNAL_SIGRTMAXMINUS8: &str = "SIGRTMAX-8";
pub const SIGNAL_SIGRTMAXMINUS7: &str = "SIGRTMAX-7";
pub const SIGNAL_SIGRTMAXMINUS6: &str = "SIGRTMAX-6";
pub const SIGNAL_SIGRTMAXMINUS5: &str = "SIGRTMAX-5";
pub const SIGNAL_SIGRTMAXMINUS4: &str = "SIGRTMAX-4";
pub const SIGNAL_SIGRTMAXMINUS3: &str = "SIGRTMAX-3";
pub const SIGNAL_SIGRTMAXMINUS2: &str = "SIGRTMAX-2";
pub const SIGNAL_SIGRTMAXMINUS1: &str = "SIGRTMAX-1";
pub const SIGNAL_SIGRTMAX: &str = "SIGRTMAX";

// PodResizeStatus constants
pub const POD_RESIZE_STATUS_IN_PROGRESS: &str = "InProgress";
pub const POD_RESIZE_STATUS_DEFERRED: &str = "Deferred";
pub const POD_RESIZE_STATUS_INFEASIBLE: &str = "Infeasible";

// ResourceName constants
pub const RESOURCE_CPU: &str = "cpu";
pub const RESOURCE_MEMORY: &str = "memory";
pub const RESOURCE_STORAGE: &str = "storage";
pub const RESOURCE_EPHEMERAL_STORAGE: &str = "ephemeral-storage";
pub const RESOURCE_PODS: &str = "pods";
pub const RESOURCE_SERVICES: &str = "services";
pub const RESOURCE_REPLICATION_CONTROLLERS: &str = "replicationcontrollers";
pub const RESOURCE_QUOTAS: &str = "resourcequotas";
pub const RESOURCE_SECRETS: &str = "secrets";
pub const RESOURCE_CONFIG_MAPS: &str = "configmaps";
pub const RESOURCE_PERSISTENT_VOLUME_CLAIMS: &str = "persistentvolumeclaims";
pub const RESOURCE_SERVICES_NODE_PORTS: &str = "services.nodeports";
pub const RESOURCE_SERVICES_LOAD_BALANCERS: &str = "services.loadbalancers";
pub const RESOURCE_REQUESTS_CPU: &str = "requests.cpu";
pub const RESOURCE_REQUESTS_MEMORY: &str = "requests.memory";
pub const RESOURCE_REQUESTS_STORAGE: &str = "requests.storage";
pub const RESOURCE_REQUESTS_EPHEMERAL_STORAGE: &str = "requests.ephemeral-storage";
pub const RESOURCE_LIMITS_CPU: &str = "limits.cpu";
pub const RESOURCE_LIMITS_MEMORY: &str = "limits.memory";
pub const RESOURCE_LIMITS_EPHEMERAL_STORAGE: &str = "limits.ephemeral-storage";
pub const RESOURCE_CLAIMS_PER_CLASS: &str = ".deviceclass.resource.k8s.io/devices";
pub const RESOURCE_REQUESTS_HUGE_PAGES_PREFIX: &str = "requests.hugepages-";
pub const RESOURCE_DEFAULT_NAMESPACE_PREFIX: &str = "kubernetes.io/";
pub const RESOURCE_HUGE_PAGES_PREFIX: &str = "hugepages-";
pub const RESOURCE_ATTACHABLE_VOLUMES_PREFIX: &str = "attachable-volumes-";
pub const DEFAULT_RESOURCE_REQUESTS_PREFIX: &str = "requests.";

// PersistentVolume annotation constants
pub const BETA_STORAGE_CLASS_ANNOTATION: &str = "volume.beta.kubernetes.io/storage-class";
pub const MOUNT_OPTION_ANNOTATION: &str = "volume.beta.kubernetes.io/mount-options";

// Scheduling defaults
pub const DEFAULT_SCHEDULER_NAME: &str = "default-scheduler";
pub const DEFAULT_HARD_POD_AFFINITY_SYMMETRIC_WEIGHT: i32 = 1;

// Log stream constants
pub const LOG_STREAM_STDOUT: &str = "Stdout";
pub const LOG_STREAM_STDERR: &str = "Stderr";
pub const LOG_STREAM_ALL: &str = "All";

// Event type constants
pub const EVENT_TYPE_NORMAL: &str = "Normal";
pub const EVENT_TYPE_WARNING: &str = "Warning";

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
    pub restart_policy: RestartPolicy,

    /// Optional duration in seconds for graceful termination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termination_grace_period_seconds: Option<i64>,

    /// Optional duration in seconds the pod may be active before termination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_deadline_seconds: Option<i64>,

    /// Set DNS policy for the pod.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub dns_policy: DNSPolicy,

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

    /// Share a single process namespace between all containers in a pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_process_namespace: Option<bool>,

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
    pub preemption_policy: Option<PreemptionPolicy>,

    /// Overhead represents the resource overhead.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub overhead: BTreeMap<ResourceName, Quantity>,

    /// TopologySpreadConstraints describes how pods should spread across topology domains.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topology_spread_constraints: Vec<TopologySpreadConstraint>,

    /// If true the pod's hostname will be configured as the pod's FQDN.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "setHostnameAsFQDN")]
    pub set_hostname_as_fqdn: Option<bool>,

    /// OS specifies the target OS for the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<PodOS>,

    /// Use the host's user namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_users: Option<bool>,

    /// SchedulingGates is a list of values that block pod scheduling.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scheduling_gates: Vec<PodSchedulingGate>,

    /// ResourceClaims defines which ResourceClaims must be allocated.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_claims: Vec<PodResourceClaim>,

    /// Resources is the total amount of CPU and Memory resources required by all containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceRequirements>,

    /// HostnameOverride specifies an explicit override for the pod's hostname.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname_override: Option<String>,
}

fn is_false(b: &bool) -> bool {
    !*b
}

/// PodStatus represents information about the status of a pod.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodStatus {
    /// If set, this represents the .metadata.generation that the pod status was set based upon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,

    /// Current condition of the pod.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub phase: PodPhase,

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

    /// IP addresses allocated to the host.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub host_i_ps: Vec<HostIP>,

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
    pub qos_class: PodQOSClass,

    /// Status of resources resize desired for pod's containers.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resize: PodResizeStatus,

    /// Status of resource claims.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_claim_statuses: Vec<PodResourceClaimStatus>,

    /// Status of extended resource claim backed by DRA.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extended_resource_claim_status: Option<PodExtendedResourceClaimStatus>,
}

/// PodCondition contains details for the current condition of this pod.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodCondition {
    /// Type is the type of the condition.
    #[serde(rename = "type")]
    pub condition_type: PodConditionType,

    /// Status is the status of the condition.
    pub status: ConditionStatus,

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

/// HostIP represents a single IP address allocated to the host.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostIP {
    /// IP is the IP address assigned to the host.
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

    /// Resources resize policy for the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resize_policy: Vec<ContainerResizePolicy>,

    /// RestartPolicy defines the restart behavior of individual containers in a pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<ContainerRestartPolicy>,

    /// Represents a list of rules to be checked to determine if the container should be restarted.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restart_policy_rules: Vec<ContainerRestartRule>,

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
    pub termination_message_policy: TerminationMessagePolicy,

    /// Image pull policy.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub image_pull_policy: PullPolicy,

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

pub type EphemeralContainerCommon = Container;

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

    /// Resources resize policy for the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resize_policy: Vec<ContainerResizePolicy>,

    /// RestartPolicy defines the restart behavior of individual containers in a pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<ContainerRestartPolicy>,

    /// Represents a list of rules to be checked to determine if the container should be restarted.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restart_policy_rules: Vec<ContainerRestartRule>,

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
    pub termination_message_policy: TerminationMessagePolicy,

    /// Image pull policy.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub image_pull_policy: PullPolicy,

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

    /// Target container name for this ephemeral container.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub target_container_name: String,
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
    pub protocol: Protocol,

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

    /// AllocatedResources represents the compute resources allocated for this container.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub allocated_resources: BTreeMap<ResourceName, Quantity>,

    /// Resources represents the compute resource requests and limits enacted on the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceRequirements>,

    /// Status of volume mounts.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volume_mounts: Vec<VolumeMountStatus>,

    /// User represents user identity information for the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<ContainerUser>,

    /// AllocatedResourcesStatus represents the status of various resources allocated for this container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allocated_resources_status: Vec<ResourceStatus>,

    /// StopSignal reports the effective stop signal for this container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<Signal>,
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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerResizePolicy {
    pub resource_name: ResourceName,
    pub restart_policy: ResourceResizeRestartPolicy,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerRestartRule {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub action: ContainerRestartRuleAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_codes: Option<ContainerRestartRuleOnExitCodes>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerRestartRuleOnExitCodes {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub operator: ContainerRestartRuleOnExitCodesOperator,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<i32>,
}

/// VolumeMountStatus shows status of volume mounts.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeMountStatus {
    pub name: String,
    pub mount_path: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recursive_read_only: Option<RecursiveReadOnlyMode>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceStatus {
    pub name: ResourceName,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<ResourceHealth>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceHealth {
    pub resource_i_d: ResourceID,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub health: ResourceHealthStatus,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linux: Option<LinuxContainerUser>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxContainerUser {
    pub uid: i64,
    pub gid: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supplemental_groups: Vec<i64>,
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

    /// Selects a key of the env file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_key_ref: Option<FileKeySelector>,
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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileKeySelector {
    pub volume_name: String,
    pub path: String,
    pub key: String,
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
    pub limits: BTreeMap<ResourceName, Quantity>,

    /// Requests describes the minimum amount of compute resources required.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub requests: BTreeMap<ResourceName, Quantity>,

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
    pub host_path_type: Option<HostPathType>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmptyDirVolumeSource {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub medium: StorageMedium,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_trust_bundle: Option<ClusterTrustBundleProjection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_certificate: Option<PodCertificateProjection>,
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
pub struct ClusterTrustBundleProjection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signer_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<LabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    pub path: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodCertificateProjection {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub signer_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub key_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_expiration_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub credential_bundle_path: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub key_path: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub certificate_chain_path: String,
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
    pub mount_propagation: Option<MountPropagationMode>,

    /// Expanded path within the volume from which the container's volume should be mounted.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sub_path_expr: String,

    /// RecursiveReadOnly specifies whether read-only mounts should be handled recursively.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recursive_read_only: Option<RecursiveReadOnlyMode>,
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
    pub service_type: ServiceType,

    /// externalIPs is a list of IP addresses for which nodes in the cluster will accept traffic.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub external_i_ps: Vec<String>,

    /// sessionAffinity determines the session affinity configuration.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub session_affinity: ServiceAffinity,

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
    pub external_traffic_policy: ServiceExternalTrafficPolicy,

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
    pub ip_families: Vec<IPFamily>,

    /// IPFamilyPolicy represents the dual-stack-ness requested.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_family_policy: Option<IPFamilyPolicy>,

    /// allocateLoadBalancerNodePorts defines if NodePorts will be allocated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocate_load_balancer_node_ports: Option<bool>,

    /// loadBalancerClass is the class of the load balancer implementation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_class: Option<String>,

    /// internalTrafficPolicy describes how nodes distribute service traffic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internal_traffic_policy: Option<ServiceInternalTrafficPolicy>,

    /// TrafficDistribution offers a way to express preferences for how traffic is distributed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traffic_distribution: Option<ServiceTrafficDistribution>,
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
    pub protocol: Protocol,

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_mode: Option<LoadBalancerIPMode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<PortStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortStatus {
    pub port: i32,
    pub protocol: Protocol,
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
    pub secret_type: SecretType,
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
    pub finalizers: Vec<FinalizerName>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceStatus {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub phase: NamespacePhase,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<NamespaceCondition>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceCondition {
    #[serde(rename = "type")]
    pub condition_type: NamespaceConditionType,
    pub status: ConditionStatus,
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
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub external_i_d: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeStatus {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub capacity: BTreeMap<ResourceName, Quantity>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub allocatable: BTreeMap<ResourceName, Quantity>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub phase: NodePhase,
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
    pub volumes_in_use: Vec<UniqueVolumeName>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes_attached: Vec<AttachedVolume>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<NodeConfigStatus>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runtime_handlers: Vec<NodeRuntimeHandler>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<NodeFeatures>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeCondition {
    #[serde(rename = "type")]
    pub condition_type: NodeConditionType,
    pub status: ConditionStatus,
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
    pub address_type: NodeAddressType,
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
pub struct NodeRuntimeHandlerFeatures {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recursive_read_only_mounts: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_namespaces: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeRuntimeHandler {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<NodeRuntimeHandlerFeatures>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeFeatures {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplemental_groups_policy: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeSwapStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swap: Option<NodeSwapStatus>,
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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConfigStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigned: Option<NodeConfigSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<NodeConfigSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_known_good: Option<NodeConfigSource>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub error: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvoidPods {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prefer_avoid_pods: Vec<PreferAvoidPodsEntry>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreferAvoidPodsEntry {
    pub pod_signature: PodSignature,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eviction_time: Option<Time>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodSignature {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_controller: Option<k8s_apimachinery::apis::meta::v1::OwnerReference>,
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
pub struct PersistentVolumeSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gce_persistent_disk: Option<GCEPersistentDiskVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aws_elastic_block_store: Option<AWSElasticBlockStoreVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_path: Option<HostPathVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glusterfs: Option<GlusterfsPersistentVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nfs: Option<NFSVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rbd: Option<RBDPersistentVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iscsi: Option<ISCSIPersistentVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cinder: Option<CinderPersistentVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cephfs: Option<CephFSPersistentVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fc: Option<FCVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flocker: Option<FlockerVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flex_volume: Option<FlexPersistentVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azure_file: Option<AzureFilePersistentVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vsphere_volume: Option<VsphereVirtualDiskVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quobyte: Option<QuobyteVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azure_disk: Option<AzureDiskVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub photon_persistent_disk: Option<PhotonPersistentDiskVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub portworx_volume: Option<PortworxVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale_io: Option<ScaleIOPersistentVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local: Option<LocalVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storageos: Option<StorageOSPersistentVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csi: Option<CSIPersistentVolumeSource>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeSpec {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub capacity: ResourceList,
    #[serde(flatten)]
    pub persistent_volume_source: PersistentVolumeSource,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub access_modes: Vec<PersistentVolumeAccessMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_ref: Option<ObjectReference>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub persistent_volume_reclaim_policy: PersistentVolumeReclaimPolicy,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub storage_class_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mount_options: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_mode: Option<PersistentVolumeMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_affinity: Option<VolumeNodeAffinity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_attributes_class_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeStatus {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub phase: PersistentVolumePhase,
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
    pub operator: NodeSelectorOperator,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopologySelectorTerm {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_label_expressions: Vec<TopologySelectorLabelRequirement>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopologySelectorLabelRequirement {
    pub key: String,
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
pub struct LocalVolumeSource {
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
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
    pub access_modes: Vec<PersistentVolumeAccessMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<VolumeResourceRequirements>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub volume_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_mode: Option<PersistentVolumeMode>,
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
    pub limits: BTreeMap<ResourceName, Quantity>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub requests: BTreeMap<ResourceName, Quantity>,
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
    pub phase: PersistentVolumeClaimPhase,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub access_modes: Vec<PersistentVolumeAccessMode>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub capacity: BTreeMap<ResourceName, Quantity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<PersistentVolumeClaimCondition>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub allocated_resources: BTreeMap<ResourceName, Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocated_resource_statuses: Option<BTreeMap<ResourceName, ClaimResourceStatus>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_volume_attributes_class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_volume_status: Option<ModifyVolumeStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaimCondition {
    #[serde(rename = "type")]
    pub condition_type: PersistentVolumeClaimConditionType,
    pub status: ConditionStatus,
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
    pub status: PersistentVolumeClaimModifyVolumeStatus,
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
    pub effect: TaintEffect,
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
    pub operator: TolerationOperator,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub effect: TaintEffect,
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
    pub when_unsatisfiable: UnsatisfiableConstraintAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<LabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_domains: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_affinity_policy: Option<NodeInclusionPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_taints_policy: Option<NodeInclusionPolicy>,
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
    pub fs_group_change_policy: Option<PodFSGroupChangePolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seccomp_profile: Option<SeccompProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_armor_profile: Option<AppArmorProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub se_linux_change_policy: Option<PodSELinuxChangePolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplemental_groups_policy: Option<SupplementalGroupsPolicy>,
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
    pub proc_mount: Option<ProcMountType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seccomp_profile: Option<SeccompProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_armor_profile: Option<AppArmorProfile>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add: Vec<Capability>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub drop: Vec<Capability>,
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
    pub profile_type: SeccompProfileType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localhost_profile: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppArmorProfile {
    #[serde(rename = "type")]
    pub profile_type: AppArmorProfileType,
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
    pub scheme: URIScheme,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<Signal>,
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
    pub condition_type: PodConditionType,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodOS {
    pub name: OSName,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodSchedulingGate {
    pub name: String,
}

// =============================================================================
// Pod Resource Claims
// =============================================================================

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodResourceClaim {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_claim_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_claim_template_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodResourceClaimStatus {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_claim_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodExtendedResourceClaimStatus {
    pub request_mappings: Vec<ContainerExtendedResourceRequest>,
    pub resource_claim_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerExtendedResourceRequest {
    pub container_name: String,
    pub resource_name: String,
    pub request_name: String,
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

/// ISCSIPersistentVolumeSource represents an ISCSI disk.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ISCSIPersistentVolumeSource {
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
    pub secret_ref: Option<SecretReference>,
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

/// GlusterfsPersistentVolumeSource represents a Glusterfs mount.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlusterfsPersistentVolumeSource {
    pub endpoints: String,
    pub path: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoints_namespace: Option<String>,
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

/// RBDPersistentVolumeSource represents a Rados Block Device mount.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RBDPersistentVolumeSource {
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
    pub secret_ref: Option<SecretReference>,
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

/// FlexPersistentVolumeSource represents a generic volume resource.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexPersistentVolumeSource {
    pub driver: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<SecretReference>,
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

/// CinderPersistentVolumeSource represents a cinder volume.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CinderPersistentVolumeSource {
    #[serde(rename = "volumeID")]
    pub volume_id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<SecretReference>,
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

/// CephFSPersistentVolumeSource represents a Ceph FS mount.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CephFSPersistentVolumeSource {
    pub monitors: Vec<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub user: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub secret_file: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<SecretReference>,
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

/// AzureFilePersistentVolumeSource represents an Azure File Service mount.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AzureFilePersistentVolumeSource {
    pub secret_name: String,
    pub share_name: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_namespace: Option<String>,
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
    pub caching_mode: Option<AzureDataDiskCachingMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<AzureDataDiskKind>,
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

/// ScaleIOPersistentVolumeSource represents a ScaleIO persistent volume.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScaleIOPersistentVolumeSource {
    pub gateway: String,
    pub system: String,
    pub secret_ref: SecretReference,
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

/// StorageOSPersistentVolumeSource represents a StorageOS volume.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageOSPersistentVolumeSource {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub volume_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub volume_namespace: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<ObjectReference>,
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
    pub pull_policy: PullPolicy,
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
    pub protocol: Protocol,
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

/// ReplicationControllerList is a collection of replication controllers.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplicationControllerList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<ReplicationController>,
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
    pub condition_type: ReplicationControllerConditionType,
    pub status: ConditionStatus,
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
    pub limit_type: LimitType,
    #[serde(default)]
    pub max: HashMap<ResourceName, Quantity>,
    #[serde(default)]
    pub min: HashMap<ResourceName, Quantity>,
    #[serde(default)]
    pub default: HashMap<ResourceName, Quantity>,
    #[serde(default)]
    pub default_request: HashMap<ResourceName, Quantity>,
    #[serde(default)]
    pub max_limit_request_ratio: HashMap<ResourceName, Quantity>,
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
    pub hard: HashMap<ResourceName, Quantity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<ResourceQuotaScope>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_selector: Option<ScopeSelector>,
}

/// ResourceQuotaStatus defines the enforced hard limits and observed use.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceQuotaStatus {
    #[serde(default)]
    pub hard: HashMap<ResourceName, Quantity>,
    #[serde(default)]
    pub used: HashMap<ResourceName, Quantity>,
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
    pub scope_name: ResourceQuotaScope,
    pub operator: ScopeSelectorOperator,
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
// Subresource Options
// =============================================================================

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preconditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<k8s_apimachinery::types::UID>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodStatusResult {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PodStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodLogOptions {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub container: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub follow: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub previous: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub since_seconds: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub since_time: Option<Time>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub timestamps: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tail_lines: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_bytes: Option<i64>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub insecure_skip_tls_verify_backend: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodAttachOptions {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default, skip_serializing_if = "is_false")]
    pub stdin: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub stdout: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub stderr: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub tty: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub container: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodExecOptions {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default, skip_serializing_if = "is_false")]
    pub stdin: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub stdout: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub stderr: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub tty: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub container: String,
    pub command: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodPortForwardOptions {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<i32>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodProxyOptions {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeProxyOptions {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceProxyOptions {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedReference {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<ObjectReference>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RangeAllocation {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    pub range: String,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct List {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<serde_json::Value>,
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
    pub condition_type: ComponentConditionType,
    /// Status of the condition.
    pub status: ConditionStatus,
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
