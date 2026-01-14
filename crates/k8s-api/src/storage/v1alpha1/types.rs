//! Storage v1alpha1 type definitions
//!
//! This module provides alpha-level storage types including VolumeAttributesClass.

use k8s_apimachinery::apis::meta::v1::{LabelSelector, ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// VolumeAttributesClass (K8s 1.29+)
// =============================================================================

/// VolumeAttributesClass represents a specification of mutable volume attributes
/// defined by the CSI driver. The class can be specified during dynamic provisioning
/// of PersistentVolumeClaims, and changed in the PersistentVolumeClaim spec after provisioning.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttributesClass {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// Name of the CSI driver. This field is immutable.
    pub driver_name: String,
    /// Parameters hold volume attributes defined by the CSI driver.
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
    pub metadata: k8s_apimachinery::apis::meta::v1::ListMeta,
    /// Items is the list of VolumeAttributesClass objects.
    pub items: Vec<VolumeAttributesClass>,
}

// =============================================================================
// VolumeAttachment (deprecated, replaced by v1)
// =============================================================================

/// VolumeAttachment captures the intent to attach or detach the specified volume
/// to/from the specified node. VolumeAttachment objects are non-namespaced.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttachment {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// Spec represents specification of the desired attach/detach volume behavior.
    pub spec: VolumeAttachmentSpec,
    /// Status represents status of the VolumeAttachment request.
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
    /// Items is the list of VolumeAttachments.
    pub items: Vec<VolumeAttachment>,
}

/// VolumeAttachmentSpec is the specification of a VolumeAttachment request.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttachmentSpec {
    /// Attacher indicates the name of the volume driver that MUST handle this request.
    pub attacher: String,
    /// Source represents the volume that should be attached.
    pub source: VolumeAttachmentSource,
    /// NodeName represents the node that the volume should be attached to.
    pub node_name: String,
}

/// VolumeAttachmentSource represents a volume that should be attached.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttachmentSource {
    /// PersistentVolumeName represents the name of the persistent volume to attach.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persistent_volume_name: Option<String>,
    /// InlineVolumeSpec contains all the information necessary to attach a persistent volume
    /// defined by a pod's inline VolumeSource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline_volume_spec: Option<serde_json::Value>,
}

/// VolumeAttachmentStatus is the status of a VolumeAttachment request.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttachmentStatus {
    /// Attached indicates the volume is successfully attached.
    pub attached: bool,
    /// AttachmentMetadata is populated with any information returned by the attach operation.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub attachment_metadata: HashMap<String, String>,
    /// AttachError represents the last error encountered during attach operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attach_error: Option<VolumeError>,
    /// DetachError represents the last error encountered during detach operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detach_error: Option<VolumeError>,
}

/// VolumeError captures an error encountered during a volume operation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeError {
    /// Time represents the time the error was encountered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<k8s_apimachinery::apis::meta::v1::Time>,
    /// Message represents the error encountered during Attach or Detach operation.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    /// ErrorCode is a numeric gRPC code representing the error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
}

// =============================================================================
// CSIStorageCapacity (deprecated, replaced by v1beta1/v1)
// =============================================================================

/// CSIStorageCapacity stores the result of one CSI GetCapacity call.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSIStorageCapacity {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ObjectMeta,
    /// NodeTopology defines which nodes have access to the storage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_topology: Option<LabelSelector>,
    /// StorageClassName represents the name of the StorageClass that the reported capacity applies to.
    pub storage_class_name: String,
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
    /// Items is the list of CSIStorageCapacity objects.
    pub items: Vec<CSIStorageCapacity>,
}
