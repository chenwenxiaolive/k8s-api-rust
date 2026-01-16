//! Storage v1beta1 type definitions
//!
//! This module provides beta-level storage types including:
//! - VolumeAttributesClass (K8s 1.31+): Mutable volume attributes
//! - CSIStorageCapacity: Storage capacity reporting
//! - StorageClass, VolumeAttachment, CSIDriver, CSINode (deprecated, use v1)

use k8s_apimachinery::apis::meta::v1::{LabelSelector, ListMeta, ObjectMeta, Time, TypeMeta};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::core::v1::{TopologySelectorLabelRequirement, TopologySelectorTerm};

pub type VolumeBindingMode = String;
pub type FSGroupPolicy = String;
pub type VolumeLifecycleMode = String;

// =============================================================================
// VolumeAttributesClass (K8s 1.31+)
// =============================================================================

/// VolumeAttributesClass represents a specification of mutable volume attributes
/// defined by the CSI driver.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttributesClass {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// Name of the CSI driver. This field is immutable.
    pub driver_name: String,
    /// parameters hold volume attributes defined by the CSI driver.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub parameters: HashMap<String, String>,
}

/// VolumeAttributesClassList is a collection of VolumeAttributesClass objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttributesClassList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// items is the list of VolumeAttributesClass objects.
    pub items: Vec<VolumeAttributesClass>,
}

// =============================================================================
// CSIStorageCapacity (deprecated in 1.24, use v1)
// =============================================================================

/// CSIStorageCapacity stores the result of one CSI GetCapacity call.
/// Deprecated: use storage.k8s.io/v1 CSIStorageCapacity instead.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSIStorageCapacity {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// nodeTopology defines which nodes have access to the storage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_topology: Option<LabelSelector>,
    /// storageClassName represents the name of the StorageClass.
    pub storage_class_name: String,
    /// capacity is the value reported by the CSI driver in its GetCapacityResponse.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<String>,
    /// maximumVolumeSize is the largest size that may be used to create a volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_volume_size: Option<String>,
}

/// CSIStorageCapacityList is a collection of CSIStorageCapacity objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSIStorageCapacityList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// items is the list of CSIStorageCapacity objects.
    pub items: Vec<CSIStorageCapacity>,
}

// =============================================================================
// StorageClass (deprecated in 1.19, use v1)
// =============================================================================

/// StorageClass describes the parameters for a class of storage.
/// Deprecated: use storage.k8s.io/v1 StorageClass instead.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageClass {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// provisioner indicates the type of the provisioner.
    pub provisioner: String,
    /// parameters holds the parameters for the provisioner.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub parameters: HashMap<String, String>,
    /// reclaimPolicy controls the reclaimPolicy for dynamically provisioned PersistentVolumes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reclaim_policy: Option<String>,
    /// mountOptions controls the mountOptions for dynamically provisioned PersistentVolumes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mount_options: Vec<String>,
    /// allowVolumeExpansion shows whether the storage class allow volume expand.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_volume_expansion: Option<bool>,
    /// volumeBindingMode indicates how PersistentVolumeClaims should be provisioned and bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_binding_mode: Option<VolumeBindingMode>,
    /// allowedTopologies restrict the node topologies where volumes can be dynamically provisioned.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_topologies: Vec<TopologySelectorTerm>,
}

/// StorageClassList is a collection of storage classes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageClassList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// items is the list of StorageClasses.
    pub items: Vec<StorageClass>,
}

// Volume binding mode constants
pub const VOLUME_BINDING_IMMEDIATE: &str = "Immediate";
pub const VOLUME_BINDING_WAIT_FOR_FIRST_CONSUMER: &str = "WaitForFirstConsumer";

// =============================================================================
// VolumeAttachment (deprecated in 1.19, use v1)
// =============================================================================

/// VolumeAttachment captures the intent to attach or detach the specified volume.
/// Deprecated: use storage.k8s.io/v1 VolumeAttachment instead.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttachment {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// spec represents specification of the desired attach/detach volume behavior.
    pub spec: VolumeAttachmentSpec,
    /// status represents status of the VolumeAttachment request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<VolumeAttachmentStatus>,
}

/// VolumeAttachmentList is a collection of VolumeAttachment objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttachmentList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// items is the list of VolumeAttachments.
    pub items: Vec<VolumeAttachment>,
}

/// VolumeAttachmentSpec is the specification of a VolumeAttachment request.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttachmentSpec {
    /// attacher indicates the name of the volume driver that MUST handle this request.
    pub attacher: String,
    /// source represents the volume that should be attached.
    pub source: VolumeAttachmentSource,
    /// nodeName represents the node that the volume should be attached to.
    pub node_name: String,
}

/// VolumeAttachmentSource represents a volume that should be attached.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttachmentSource {
    /// persistentVolumeName represents the name of the persistent volume to attach.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persistent_volume_name: Option<String>,
    /// inlineVolumeSpec contains all the information necessary to attach a persistent volume
    /// defined by a pod's inline VolumeSource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline_volume_spec: Option<serde_json::Value>,
}

/// VolumeAttachmentStatus is the status of a VolumeAttachment request.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttachmentStatus {
    /// attached indicates the volume is successfully attached.
    pub attached: bool,
    /// attachmentMetadata is populated with any information returned by the attach operation.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub attachment_metadata: HashMap<String, String>,
    /// attachError represents the last error encountered during attach operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attach_error: Option<VolumeError>,
    /// detachError represents the last error encountered during detach operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detach_error: Option<VolumeError>,
}

/// VolumeError captures an error encountered during a volume operation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeError {
    /// time represents the time the error was encountered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Time>,
    /// message represents the error encountered during Attach or Detach operation.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    /// errorCode is a numeric gRPC code representing the error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
}

// =============================================================================
// CSIDriver (deprecated in 1.19, use v1)
// =============================================================================

/// CSIDriver captures information about a Container Storage Interface (CSI) volume driver.
/// Deprecated: use storage.k8s.io/v1 CSIDriver instead.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSIDriver {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// spec represents the specification of the CSI Driver.
    pub spec: CSIDriverSpec,
}

/// CSIDriverList is a collection of CSIDriver objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSIDriverList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// items is the list of CSIDriver.
    pub items: Vec<CSIDriver>,
}

/// CSIDriverSpec is the specification of a CSIDriver.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSIDriverSpec {
    /// attachRequired indicates this CSI volume driver requires an attach operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attach_required: Option<bool>,
    /// podInfoOnMount indicates this CSI volume driver requires additional pod information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_info_on_mount: Option<bool>,
    /// volumeLifecycleModes defines what kind of volumes this CSI volume driver supports.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volume_lifecycle_modes: Vec<VolumeLifecycleMode>,
    /// storageCapacity indicates that the CSI volume driver wants pod scheduling to consider
    /// the storage capacity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_capacity: Option<bool>,
    /// fsGroupPolicy defines if the underlying volume supports changing ownership and permission.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_group_policy: Option<FSGroupPolicy>,
    /// tokenRequests indicates the CSI driver needs pods' service account tokens.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub token_requests: Vec<TokenRequest>,
    /// requiresRepublish indicates the CSI driver wants NodePublishVolume being periodically called.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requires_republish: Option<bool>,
    /// seLinuxMount specifies if the CSI driver supports "-o context" mount option.
    #[serde(rename = "seLinuxMount", default, skip_serializing_if = "Option::is_none")]
    pub se_linux_mount: Option<bool>,
    /// nodeAllocatableUpdatePeriodSeconds specifies the interval between periodic updates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_allocatable_update_period_seconds: Option<i64>,
}

/// TokenRequest contains parameters of a service account token.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenRequest {
    /// audience is the intended audience of the token.
    pub audience: String,
    /// expirationSeconds is the duration of validity of the token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: Option<i64>,
}

// FSGroupPolicy constants
pub const FS_GROUP_POLICY_READ_WRITE_ONCE_WITH_FS_TYPE: &str = "ReadWriteOnceWithFSType";
pub const FS_GROUP_POLICY_FILE: &str = "File";
pub const FS_GROUP_POLICY_NONE: &str = "None";

// VolumeLifecycleMode constants
pub const VOLUME_LIFECYCLE_PERSISTENT: &str = "Persistent";
pub const VOLUME_LIFECYCLE_EPHEMERAL: &str = "Ephemeral";

// =============================================================================
// CSINode (deprecated in 1.17, removed in 1.22, use v1)
// =============================================================================

/// CSINode holds information about all CSI drivers installed on a node.
/// Deprecated: use storage.k8s.io/v1 CSINode instead.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSINode {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// spec is the specification of CSINode.
    pub spec: CSINodeSpec,
}

/// CSINodeList is a collection of CSINode objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSINodeList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// items is the list of CSINode.
    pub items: Vec<CSINode>,
}

/// CSINodeSpec holds information about the specification of all CSI drivers installed on a node.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSINodeSpec {
    /// drivers is a list of information of all CSI Drivers existing on a node.
    pub drivers: Vec<CSINodeDriver>,
}

/// CSINodeDriver holds information about the specification of one CSI driver installed on a node.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSINodeDriver {
    /// name represents the name of the CSI driver.
    pub name: String,
    /// nodeID of the node from the driver point of view.
    pub node_id: String,
    /// topologyKeys is the list of keys supported by the driver.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topology_keys: Vec<String>,
    /// allocatable represents the volume resources of a node that are available for scheduling.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocatable: Option<VolumeNodeResources>,
}

/// VolumeNodeResources is a set of resource limits for scheduling of volumes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeNodeResources {
    /// count indicates the maximum number of unique volumes managed by the CSI driver.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}
