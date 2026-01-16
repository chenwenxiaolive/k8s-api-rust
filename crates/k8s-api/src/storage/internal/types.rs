//! Internal type definitions for storage.

use k8s_apimachinery::apis::meta::v1::{ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub use crate::core::internal::{TopologySelectorLabelRequirement, TopologySelectorTerm};

pub type FSGroupPolicy = String;
pub type VolumeBindingMode = String;
pub type VolumeLifecycleMode = String;

pub const FS_GROUP_POLICY_FILE: &str = "File";
pub const FS_GROUP_POLICY_NONE: &str = "None";
pub const FS_GROUP_POLICY_READ_WRITE_ONCE_WITH_FSTYPE: &str = "ReadWriteOnceWithFSType";
pub const FS_GROUP_POLICY_READ_WRITE_ONCE_WITH_FS_TYPE: &str = "ReadWriteOnceWithFSType";
pub const VOLUME_BINDING_IMMEDIATE: &str = "Immediate";
pub const VOLUME_BINDING_WAIT_FOR_FIRST_CONSUMER: &str = "WaitForFirstConsumer";
pub const VOLUME_LIFECYCLE_EPHEMERAL: &str = "Ephemeral";
pub const VOLUME_LIFECYCLE_PERSISTENT: &str = "Persistent";


/// CSIDriver captures information about a Container Storage Interface (CSI) volume driver deployed on the cluster.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSIDriver {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    pub spec: CSIDriverSpec,
}


/// CSIDriverList is a collection of CSIDriver objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSIDriverList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<CSIDriver>,
}


/// CSIDriverSpec is the specification of a CSIDriver.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSIDriverSpec {
    /// AttachRequired indicates this CSI volume driver requires an attach operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attach_required: Option<bool>,
    /// PodInfoOnMount indicates this CSI volume driver requires additional pod information during mount operations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_info_on_mount: Option<bool>,
    /// VolumeLifecycleModes defines what kind of volumes this CSI volume driver supports.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volume_lifecycle_modes: Vec<VolumeLifecycleMode>,
    /// StorageCapacity indicates that the CSI volume driver wants pod scheduling to consider the storage capacity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_capacity: Option<bool>,
    /// FSGroupPolicy defines if the underlying volume supports changing ownership and permission of the volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_group_policy: Option<FSGroupPolicy>,
    /// TokenRequests indicates the CSI driver needs pod service account tokens.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub token_requests: Vec<TokenRequest>,
    /// RequiresRepublish indicates the CSI driver wants NodePublishVolume being periodically called.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requires_republish: Option<bool>,
    /// SELinuxMount specifies if the CSI driver supports "-o context" mount option.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub se_linux_mount: Option<bool>,
}


/// CSINode holds information about all CSI drivers installed on a node.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSINode {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    pub spec: CSINodeSpec,
}


/// CSINodeDriver holds information about the specification of one CSI driver installed on a node.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSINodeDriver {
    /// Name is the name of the CSI driver.
    pub name: String,
    /// NodeID is the ID of the node from the driver point of view.
    pub node_id: String,
    /// TopologyKeys is the list of keys supported by the driver.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topology_keys: Vec<String>,
    /// Allocatable represents the volume resources of a node that are available for scheduling.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocatable: Option<VolumeNodeResources>,
}


/// CSINodeList is a collection of CSINode objects.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSINodeList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<CSINode>,
}


/// CSINodeSpec holds information about the specification of all CSI drivers installed on a node.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSINodeSpec {
    pub drivers: Vec<CSINodeDriver>,
}


/// CSIStorageCapacity stores the result of one CSI GetCapacity call.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSIStorageCapacity {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// StorageClassName is the name of the StorageClass that the reported capacity applies to.
    pub storage_class_name: String,
    /// NodeTopology defines which nodes have access to the storage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_topology: Option<k8s_apimachinery::apis::meta::v1::LabelSelector>,
    /// Capacity is the value reported by the CSI driver in its GetCapacityResponse.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<String>,
    /// MaximumVolumeSize is the value reported by the CSI driver in its GetCapacityResponse.
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
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<CSIStorageCapacity>,
}


/// StorageClass describes the parameters for a class of storage for which PersistentVolumes can be dynamically provisioned.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageClass {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// Provisioner indicates the type of the provisioner.
    pub provisioner: String,
    /// Parameters holds the parameters for the provisioner that should create volumes of this storage class.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub parameters: BTreeMap<String, String>,
    /// ReclaimPolicy controls the reclaimPolicy for dynamically provisioned PersistentVolumes of this storage class.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reclaim_policy: Option<String>,
    /// MountOptions controls the mountOptions for dynamically provisioned PersistentVolumes of this storage class.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mount_options: Vec<String>,
    /// AllowVolumeExpansion shows whether the storage class allow volume expand.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_volume_expansion: Option<bool>,
    /// VolumeBindingMode indicates how PersistentVolumeClaims should be provisioned and bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_binding_mode: Option<VolumeBindingMode>,
    /// AllowedTopologies restrict the node topologies where volumes can be dynamically provisioned.
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
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<StorageClass>,
}


/// TokenRequest contains parameters of a service account token.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenRequest {
    pub audience: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: Option<i64>,
}


/// VolumeAttachment captures the intent to attach or detach the specified volume to/from the specified node.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttachment {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    pub spec: VolumeAttachmentSpec,
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
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<VolumeAttachment>,
}


/// VolumeAttachmentSource represents a volume that should be attached.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttachmentSource {
    /// PersistentVolumeName represents the name of the persistent volume to attach.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persistent_volume_name: Option<String>,
    /// InlineVolumeSpec contains all the information necessary to attach a persistent volume defined by a pod's inline VolumeSource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline_volume_spec: Option<serde_json::Value>,
}


/// VolumeAttachmentSpec is the specification of a VolumeAttachment request.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttachmentSpec {
    /// Attacher indicates the name of the volume driver that MUST handle this request.
    pub attacher: String,
    /// Source represents the volume that should be attached.
    pub source: VolumeAttachmentSource,
    /// NodeName is the node that the volume should be attached to.
    pub node_name: String,
}


/// VolumeAttachmentStatus is the status of a VolumeAttachment request.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttachmentStatus {
    /// Attached indicates the volume is successfully attached.
    pub attached: bool,
    /// AttachmentMetadata is populated with any information returned by the attach operation.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub attachment_metadata: BTreeMap<String, String>,
    /// AttachError represents the last error encountered during attach operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attach_error: Option<VolumeError>,
    /// DetachError represents the last error encountered during detach operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detach_error: Option<VolumeError>,
}


/// VolumeAttributesClass represents a specification of mutable volume attributes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttributesClass {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// DriverName is the name of the CSI driver.
    pub driver_name: String,
    /// Parameters hold volume attributes defined by the CSI driver.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub parameters: BTreeMap<String, String>,
}


/// VolumeAttributesClassList is a collection of VolumeAttributesClass objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttributesClassList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    pub items: Vec<VolumeAttributesClass>,
}


/// VolumeError captures an error encountered during a volume operation.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}


/// VolumeNodeResources is a set of resource limits for scheduling of volumes.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeNodeResources {
    /// Maximum number of unique volumes managed by the CSI driver that can be used on a node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}
