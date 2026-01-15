//! Storage API conversions
//!
//! This module provides conversions between storage API versions.
//!
//! Note: v1beta1 is deprecated for most types. v1 is the stable version.
//! Some v1beta1 types (like VolumeAttributesClass) are newer features not yet in v1.

use crate::{ConversionError, Convertible};

// =============================================================================
// StorageClass: v1 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::storage::v1::StorageClass> for k8s_api::storage::v1beta1::StorageClass {
    fn convert_to(&self) -> Result<k8s_api::storage::v1::StorageClass, ConversionError> {
        Ok(k8s_api::storage::v1::StorageClass {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1",
                "StorageClass",
            ),
            metadata: self.metadata.clone(),
            provisioner: self.provisioner.clone(),
            parameters: self.parameters.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            reclaim_policy: self.reclaim_policy.clone(),
            mount_options: self.mount_options.clone(),
            allow_volume_expansion: self.allow_volume_expansion,
            volume_binding_mode: self.volume_binding_mode.clone(),
            allowed_topologies: self
                .allowed_topologies
                .iter()
                .map(convert_topology_term_to_v1)
                .collect(),
        })
    }

    fn convert_from(other: &k8s_api::storage::v1::StorageClass) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1beta1",
                "StorageClass",
            ),
            metadata: other.metadata.clone(),
            provisioner: other.provisioner.clone(),
            parameters: other.parameters.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            reclaim_policy: other.reclaim_policy.clone(),
            mount_options: other.mount_options.clone(),
            allow_volume_expansion: other.allow_volume_expansion,
            volume_binding_mode: other.volume_binding_mode.clone(),
            allowed_topologies: other
                .allowed_topologies
                .iter()
                .map(convert_topology_term_from_v1)
                .collect(),
        })
    }
}

fn convert_topology_term_to_v1(
    term: &k8s_api::storage::v1beta1::TopologySelectorTerm,
) -> k8s_api::storage::v1::TopologySelectorTerm {
    k8s_api::storage::v1::TopologySelectorTerm {
        match_label_expressions: term
            .match_label_expressions
            .iter()
            .map(|req| k8s_api::storage::v1::TopologySelectorLabelRequirement {
                key: req.key.clone(),
                values: req.values.clone(),
            })
            .collect(),
    }
}

fn convert_topology_term_from_v1(
    term: &k8s_api::storage::v1::TopologySelectorTerm,
) -> k8s_api::storage::v1beta1::TopologySelectorTerm {
    k8s_api::storage::v1beta1::TopologySelectorTerm {
        match_label_expressions: term
            .match_label_expressions
            .iter()
            .map(|req| k8s_api::storage::v1beta1::TopologySelectorLabelRequirement {
                key: req.key.clone(),
                values: req.values.clone(),
            })
            .collect(),
    }
}

// =============================================================================
// StorageClassList: v1 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::storage::v1::StorageClassList>
    for k8s_api::storage::v1beta1::StorageClassList
{
    fn convert_to(&self) -> Result<k8s_api::storage::v1::StorageClassList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::storage::v1::StorageClassList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1",
                "StorageClassList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::storage::v1::StorageClassList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::storage::v1beta1::StorageClass::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1beta1",
                "StorageClassList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

// =============================================================================
// VolumeAttachment: v1 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::storage::v1::VolumeAttachment>
    for k8s_api::storage::v1beta1::VolumeAttachment
{
    fn convert_to(&self) -> Result<k8s_api::storage::v1::VolumeAttachment, ConversionError> {
        Ok(k8s_api::storage::v1::VolumeAttachment {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1",
                "VolumeAttachment",
            ),
            metadata: self.metadata.clone(),
            spec: convert_volume_attachment_spec_to_v1(&self.spec),
            status: self.status.as_ref().map(convert_volume_attachment_status_to_v1),
        })
    }

    fn convert_from(
        other: &k8s_api::storage::v1::VolumeAttachment,
    ) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1beta1",
                "VolumeAttachment",
            ),
            metadata: other.metadata.clone(),
            spec: convert_volume_attachment_spec_from_v1(&other.spec),
            status: other
                .status
                .as_ref()
                .map(convert_volume_attachment_status_from_v1),
        })
    }
}

fn convert_volume_attachment_spec_to_v1(
    spec: &k8s_api::storage::v1beta1::VolumeAttachmentSpec,
) -> k8s_api::storage::v1::VolumeAttachmentSpec {
    k8s_api::storage::v1::VolumeAttachmentSpec {
        attacher: spec.attacher.clone(),
        source: k8s_api::storage::v1::VolumeAttachmentSource {
            persistent_volume_name: spec.source.persistent_volume_name.clone(),
            inline_volume_spec: spec.source.inline_volume_spec.clone(),
        },
        node_name: spec.node_name.clone(),
    }
}

fn convert_volume_attachment_spec_from_v1(
    spec: &k8s_api::storage::v1::VolumeAttachmentSpec,
) -> k8s_api::storage::v1beta1::VolumeAttachmentSpec {
    k8s_api::storage::v1beta1::VolumeAttachmentSpec {
        attacher: spec.attacher.clone(),
        source: k8s_api::storage::v1beta1::VolumeAttachmentSource {
            persistent_volume_name: spec.source.persistent_volume_name.clone(),
            inline_volume_spec: spec.source.inline_volume_spec.clone(),
        },
        node_name: spec.node_name.clone(),
    }
}

fn convert_volume_attachment_status_to_v1(
    status: &k8s_api::storage::v1beta1::VolumeAttachmentStatus,
) -> k8s_api::storage::v1::VolumeAttachmentStatus {
    k8s_api::storage::v1::VolumeAttachmentStatus {
        attached: status.attached,
        attachment_metadata: status
            .attachment_metadata
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect(),
        attach_error: status.attach_error.as_ref().map(|e| {
            k8s_api::storage::v1::VolumeError {
                // Convert Time to RFC 3339 string
                time: e.time.as_ref().and_then(|t| t.0.map(|dt| dt.to_rfc3339())),
                message: e.message.clone(),
            }
        }),
        detach_error: status.detach_error.as_ref().map(|e| {
            k8s_api::storage::v1::VolumeError {
                time: e.time.as_ref().and_then(|t| t.0.map(|dt| dt.to_rfc3339())),
                message: e.message.clone(),
            }
        }),
    }
}

fn convert_volume_attachment_status_from_v1(
    status: &k8s_api::storage::v1::VolumeAttachmentStatus,
) -> k8s_api::storage::v1beta1::VolumeAttachmentStatus {
    k8s_api::storage::v1beta1::VolumeAttachmentStatus {
        attached: status.attached,
        attachment_metadata: status
            .attachment_metadata
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect(),
        attach_error: status.attach_error.as_ref().map(|e| {
            k8s_api::storage::v1beta1::VolumeError {
                // Parse RFC 3339 string back to Time
                time: e.time.as_ref().and_then(|t| {
                    chrono::DateTime::parse_from_rfc3339(t)
                        .ok()
                        .map(|dt| k8s_apimachinery::apis::meta::v1::Time(Some(dt.with_timezone(&chrono::Utc))))
                }),
                message: e.message.clone(),
                error_code: None, // v1 doesn't have error_code
            }
        }),
        detach_error: status.detach_error.as_ref().map(|e| {
            k8s_api::storage::v1beta1::VolumeError {
                time: e.time.as_ref().and_then(|t| {
                    chrono::DateTime::parse_from_rfc3339(t)
                        .ok()
                        .map(|dt| k8s_apimachinery::apis::meta::v1::Time(Some(dt.with_timezone(&chrono::Utc))))
                }),
                message: e.message.clone(),
                error_code: None,
            }
        }),
    }
}

// =============================================================================
// VolumeAttachment: v1 <-> v1alpha1
// =============================================================================

impl Convertible<k8s_api::storage::v1::VolumeAttachment>
    for k8s_api::storage::v1alpha1::VolumeAttachment
{
    fn convert_to(&self) -> Result<k8s_api::storage::v1::VolumeAttachment, ConversionError> {
        Ok(k8s_api::storage::v1::VolumeAttachment {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1",
                "VolumeAttachment",
            ),
            metadata: self.metadata.clone(),
            spec: convert_volume_attachment_spec_alpha_to_v1(&self.spec),
            status: self
                .status
                .as_ref()
                .map(convert_volume_attachment_status_alpha_to_v1),
        })
    }

    fn convert_from(
        other: &k8s_api::storage::v1::VolumeAttachment,
    ) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1alpha1",
                "VolumeAttachment",
            ),
            metadata: other.metadata.clone(),
            spec: convert_volume_attachment_spec_alpha_from_v1(&other.spec),
            status: other
                .status
                .as_ref()
                .map(convert_volume_attachment_status_alpha_from_v1),
        })
    }
}

fn convert_volume_attachment_spec_alpha_to_v1(
    spec: &k8s_api::storage::v1alpha1::VolumeAttachmentSpec,
) -> k8s_api::storage::v1::VolumeAttachmentSpec {
    k8s_api::storage::v1::VolumeAttachmentSpec {
        attacher: spec.attacher.clone(),
        source: k8s_api::storage::v1::VolumeAttachmentSource {
            persistent_volume_name: spec.source.persistent_volume_name.clone(),
            inline_volume_spec: spec.source.inline_volume_spec.clone(),
        },
        node_name: spec.node_name.clone(),
    }
}

fn convert_volume_attachment_spec_alpha_from_v1(
    spec: &k8s_api::storage::v1::VolumeAttachmentSpec,
) -> k8s_api::storage::v1alpha1::VolumeAttachmentSpec {
    k8s_api::storage::v1alpha1::VolumeAttachmentSpec {
        attacher: spec.attacher.clone(),
        source: k8s_api::storage::v1alpha1::VolumeAttachmentSource {
            persistent_volume_name: spec.source.persistent_volume_name.clone(),
            inline_volume_spec: spec.source.inline_volume_spec.clone(),
        },
        node_name: spec.node_name.clone(),
    }
}

fn convert_volume_attachment_status_alpha_to_v1(
    status: &k8s_api::storage::v1alpha1::VolumeAttachmentStatus,
) -> k8s_api::storage::v1::VolumeAttachmentStatus {
    k8s_api::storage::v1::VolumeAttachmentStatus {
        attached: status.attached,
        attachment_metadata: status
            .attachment_metadata
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect(),
        attach_error: status.attach_error.as_ref().map(|e| {
            k8s_api::storage::v1::VolumeError {
                time: e.time.as_ref().and_then(|t| t.0.map(|dt| dt.to_rfc3339())),
                message: e.message.clone(),
            }
        }),
        detach_error: status.detach_error.as_ref().map(|e| {
            k8s_api::storage::v1::VolumeError {
                time: e.time.as_ref().and_then(|t| t.0.map(|dt| dt.to_rfc3339())),
                message: e.message.clone(),
            }
        }),
    }
}

fn convert_volume_attachment_status_alpha_from_v1(
    status: &k8s_api::storage::v1::VolumeAttachmentStatus,
) -> k8s_api::storage::v1alpha1::VolumeAttachmentStatus {
    k8s_api::storage::v1alpha1::VolumeAttachmentStatus {
        attached: status.attached,
        attachment_metadata: status
            .attachment_metadata
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect(),
        attach_error: status.attach_error.as_ref().map(|e| {
            k8s_api::storage::v1alpha1::VolumeError {
                time: e.time.as_ref().and_then(|t| {
                    chrono::DateTime::parse_from_rfc3339(t)
                        .ok()
                        .map(|dt| {
                            k8s_apimachinery::apis::meta::v1::Time(Some(
                                dt.with_timezone(&chrono::Utc),
                            ))
                        })
                }),
                message: e.message.clone(),
                error_code: None,
            }
        }),
        detach_error: status.detach_error.as_ref().map(|e| {
            k8s_api::storage::v1alpha1::VolumeError {
                time: e.time.as_ref().and_then(|t| {
                    chrono::DateTime::parse_from_rfc3339(t)
                        .ok()
                        .map(|dt| {
                            k8s_apimachinery::apis::meta::v1::Time(Some(
                                dt.with_timezone(&chrono::Utc),
                            ))
                        })
                }),
                message: e.message.clone(),
                error_code: None,
            }
        }),
    }
}

// =============================================================================
// VolumeAttachmentList: v1 <-> v1beta1/v1alpha1
// =============================================================================

impl Convertible<k8s_api::storage::v1::VolumeAttachmentList>
    for k8s_api::storage::v1beta1::VolumeAttachmentList
{
    fn convert_to(&self) -> Result<k8s_api::storage::v1::VolumeAttachmentList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::storage::v1::VolumeAttachmentList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1",
                "VolumeAttachmentList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::storage::v1::VolumeAttachmentList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::storage::v1beta1::VolumeAttachment::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1beta1",
                "VolumeAttachmentList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

impl Convertible<k8s_api::storage::v1::VolumeAttachmentList>
    for k8s_api::storage::v1alpha1::VolumeAttachmentList
{
    fn convert_to(&self) -> Result<k8s_api::storage::v1::VolumeAttachmentList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::storage::v1::VolumeAttachmentList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1",
                "VolumeAttachmentList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::storage::v1::VolumeAttachmentList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::storage::v1alpha1::VolumeAttachment::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1alpha1",
                "VolumeAttachmentList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

// =============================================================================
// CSIDriver: v1 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::storage::v1::CSIDriver> for k8s_api::storage::v1beta1::CSIDriver {
    fn convert_to(&self) -> Result<k8s_api::storage::v1::CSIDriver, ConversionError> {
        Ok(k8s_api::storage::v1::CSIDriver {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1",
                "CSIDriver",
            ),
            metadata: self.metadata.clone(),
            spec: convert_csi_driver_spec_to_v1(&self.spec),
        })
    }

    fn convert_from(other: &k8s_api::storage::v1::CSIDriver) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1beta1",
                "CSIDriver",
            ),
            metadata: other.metadata.clone(),
            spec: convert_csi_driver_spec_from_v1(&other.spec),
        })
    }
}

fn convert_csi_driver_spec_to_v1(
    spec: &k8s_api::storage::v1beta1::CSIDriverSpec,
) -> k8s_api::storage::v1::CSIDriverSpec {
    k8s_api::storage::v1::CSIDriverSpec {
        attach_required: spec.attach_required,
        pod_info_on_mount: spec.pod_info_on_mount,
        volume_lifecycle_modes: spec.volume_lifecycle_modes.clone(),
        storage_capacity: spec.storage_capacity,
        fs_group_policy: spec.fs_group_policy.clone(),
        token_requests: spec
            .token_requests
            .iter()
            .map(|tr| k8s_api::storage::v1::TokenRequest {
                audience: tr.audience.clone(),
                expiration_seconds: tr.expiration_seconds,
            })
            .collect(),
        requires_republish: spec.requires_republish,
        se_linux_mount: spec.se_linux_mount,
    }
}

fn convert_csi_driver_spec_from_v1(
    spec: &k8s_api::storage::v1::CSIDriverSpec,
) -> k8s_api::storage::v1beta1::CSIDriverSpec {
    k8s_api::storage::v1beta1::CSIDriverSpec {
        attach_required: spec.attach_required,
        pod_info_on_mount: spec.pod_info_on_mount,
        volume_lifecycle_modes: spec.volume_lifecycle_modes.clone(),
        storage_capacity: spec.storage_capacity,
        fs_group_policy: spec.fs_group_policy.clone(),
        token_requests: spec
            .token_requests
            .iter()
            .map(|tr| k8s_api::storage::v1beta1::TokenRequest {
                audience: tr.audience.clone(),
                expiration_seconds: tr.expiration_seconds,
            })
            .collect(),
        requires_republish: spec.requires_republish,
        se_linux_mount: spec.se_linux_mount,
        node_allocatable_update_period_seconds: None, // v1 doesn't have this
    }
}

// =============================================================================
// CSIDriverList: v1 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::storage::v1::CSIDriverList>
    for k8s_api::storage::v1beta1::CSIDriverList
{
    fn convert_to(&self) -> Result<k8s_api::storage::v1::CSIDriverList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::storage::v1::CSIDriverList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1",
                "CSIDriverList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::storage::v1::CSIDriverList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::storage::v1beta1::CSIDriver::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1beta1",
                "CSIDriverList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

// =============================================================================
// CSINode: v1 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::storage::v1::CSINode> for k8s_api::storage::v1beta1::CSINode {
    fn convert_to(&self) -> Result<k8s_api::storage::v1::CSINode, ConversionError> {
        Ok(k8s_api::storage::v1::CSINode {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1",
                "CSINode",
            ),
            metadata: self.metadata.clone(),
            spec: convert_csi_node_spec_to_v1(&self.spec),
        })
    }

    fn convert_from(other: &k8s_api::storage::v1::CSINode) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1beta1",
                "CSINode",
            ),
            metadata: other.metadata.clone(),
            spec: convert_csi_node_spec_from_v1(&other.spec),
        })
    }
}

fn convert_csi_node_spec_to_v1(
    spec: &k8s_api::storage::v1beta1::CSINodeSpec,
) -> k8s_api::storage::v1::CSINodeSpec {
    k8s_api::storage::v1::CSINodeSpec {
        drivers: spec
            .drivers
            .iter()
            .map(|d| k8s_api::storage::v1::CSINodeDriver {
                name: d.name.clone(),
                node_id: d.node_id.clone(),
                topology_keys: d.topology_keys.clone(),
                allocatable: d.allocatable.as_ref().map(|a| {
                    k8s_api::storage::v1::VolumeNodeResources { count: a.count }
                }),
            })
            .collect(),
    }
}

fn convert_csi_node_spec_from_v1(
    spec: &k8s_api::storage::v1::CSINodeSpec,
) -> k8s_api::storage::v1beta1::CSINodeSpec {
    k8s_api::storage::v1beta1::CSINodeSpec {
        drivers: spec
            .drivers
            .iter()
            .map(|d| k8s_api::storage::v1beta1::CSINodeDriver {
                name: d.name.clone(),
                node_id: d.node_id.clone(),
                topology_keys: d.topology_keys.clone(),
                allocatable: d.allocatable.as_ref().map(|a| {
                    k8s_api::storage::v1beta1::VolumeNodeResources { count: a.count }
                }),
            })
            .collect(),
    }
}

// =============================================================================
// CSINodeList: v1 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::storage::v1::CSINodeList> for k8s_api::storage::v1beta1::CSINodeList {
    fn convert_to(&self) -> Result<k8s_api::storage::v1::CSINodeList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::storage::v1::CSINodeList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1",
                "CSINodeList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(other: &k8s_api::storage::v1::CSINodeList) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::storage::v1beta1::CSINode::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1beta1",
                "CSINodeList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

// =============================================================================
// CSIStorageCapacity: v1 <-> v1beta1
// =============================================================================

impl Convertible<k8s_api::storage::v1::CSIStorageCapacity>
    for k8s_api::storage::v1beta1::CSIStorageCapacity
{
    fn convert_to(&self) -> Result<k8s_api::storage::v1::CSIStorageCapacity, ConversionError> {
        Ok(k8s_api::storage::v1::CSIStorageCapacity {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1",
                "CSIStorageCapacity",
            ),
            metadata: self.metadata.clone(),
            storage_class_name: self.storage_class_name.clone(),
            node_topology: self.node_topology.clone(),
            capacity: self.capacity.clone(),
            maximum_volume_size: self.maximum_volume_size.clone(),
        })
    }

    fn convert_from(
        other: &k8s_api::storage::v1::CSIStorageCapacity,
    ) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1beta1",
                "CSIStorageCapacity",
            ),
            metadata: other.metadata.clone(),
            storage_class_name: other.storage_class_name.clone(),
            node_topology: other.node_topology.clone(),
            capacity: other.capacity.clone(),
            maximum_volume_size: other.maximum_volume_size.clone(),
        })
    }
}

// =============================================================================
// CSIStorageCapacity: v1 <-> v1alpha1
// =============================================================================

impl Convertible<k8s_api::storage::v1::CSIStorageCapacity>
    for k8s_api::storage::v1alpha1::CSIStorageCapacity
{
    fn convert_to(&self) -> Result<k8s_api::storage::v1::CSIStorageCapacity, ConversionError> {
        Ok(k8s_api::storage::v1::CSIStorageCapacity {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1",
                "CSIStorageCapacity",
            ),
            metadata: self.metadata.clone(),
            storage_class_name: self.storage_class_name.clone(),
            node_topology: self.node_topology.clone(),
            capacity: self.capacity.clone(),
            maximum_volume_size: self.maximum_volume_size.clone(),
        })
    }

    fn convert_from(
        other: &k8s_api::storage::v1::CSIStorageCapacity,
    ) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1alpha1",
                "CSIStorageCapacity",
            ),
            metadata: other.metadata.clone(),
            storage_class_name: other.storage_class_name.clone(),
            node_topology: other.node_topology.clone(),
            capacity: other.capacity.clone(),
            maximum_volume_size: other.maximum_volume_size.clone(),
        })
    }
}

// =============================================================================
// CSIStorageCapacityList: v1 <-> v1beta1/v1alpha1
// =============================================================================

impl Convertible<k8s_api::storage::v1::CSIStorageCapacityList>
    for k8s_api::storage::v1beta1::CSIStorageCapacityList
{
    fn convert_to(&self) -> Result<k8s_api::storage::v1::CSIStorageCapacityList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::storage::v1::CSIStorageCapacityList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1",
                "CSIStorageCapacityList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::storage::v1::CSIStorageCapacityList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::storage::v1beta1::CSIStorageCapacity::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1beta1",
                "CSIStorageCapacityList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

impl Convertible<k8s_api::storage::v1::CSIStorageCapacityList>
    for k8s_api::storage::v1alpha1::CSIStorageCapacityList
{
    fn convert_to(&self) -> Result<k8s_api::storage::v1::CSIStorageCapacityList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::storage::v1::CSIStorageCapacityList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1",
                "CSIStorageCapacityList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::storage::v1::CSIStorageCapacityList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::storage::v1alpha1::CSIStorageCapacity::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1alpha1",
                "CSIStorageCapacityList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

// =============================================================================
// VolumeAttributesClass: v1 <-> v1beta1/v1alpha1
// =============================================================================

impl Convertible<k8s_api::storage::v1::VolumeAttributesClass>
    for k8s_api::storage::v1beta1::VolumeAttributesClass
{
    fn convert_to(&self) -> Result<k8s_api::storage::v1::VolumeAttributesClass, ConversionError> {
        Ok(k8s_api::storage::v1::VolumeAttributesClass {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1",
                "VolumeAttributesClass",
            ),
            metadata: self.metadata.clone(),
            driver_name: self.driver_name.clone(),
            parameters: self
                .parameters
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        })
    }

    fn convert_from(
        other: &k8s_api::storage::v1::VolumeAttributesClass,
    ) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1beta1",
                "VolumeAttributesClass",
            ),
            metadata: other.metadata.clone(),
            driver_name: other.driver_name.clone(),
            parameters: other
                .parameters
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        })
    }
}

impl Convertible<k8s_api::storage::v1::VolumeAttributesClass>
    for k8s_api::storage::v1alpha1::VolumeAttributesClass
{
    fn convert_to(&self) -> Result<k8s_api::storage::v1::VolumeAttributesClass, ConversionError> {
        Ok(k8s_api::storage::v1::VolumeAttributesClass {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1",
                "VolumeAttributesClass",
            ),
            metadata: self.metadata.clone(),
            driver_name: self.driver_name.clone(),
            parameters: self
                .parameters
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        })
    }

    fn convert_from(
        other: &k8s_api::storage::v1::VolumeAttributesClass,
    ) -> Result<Self, ConversionError> {
        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1alpha1",
                "VolumeAttributesClass",
            ),
            metadata: other.metadata.clone(),
            driver_name: other.driver_name.clone(),
            parameters: other
                .parameters
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        })
    }
}

// =============================================================================
// VolumeAttributesClassList: v1 <-> v1beta1/v1alpha1
// =============================================================================

impl Convertible<k8s_api::storage::v1::VolumeAttributesClassList>
    for k8s_api::storage::v1beta1::VolumeAttributesClassList
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::storage::v1::VolumeAttributesClassList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::storage::v1::VolumeAttributesClassList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1",
                "VolumeAttributesClassList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::storage::v1::VolumeAttributesClassList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::storage::v1beta1::VolumeAttributesClass::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1beta1",
                "VolumeAttributesClassList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

impl Convertible<k8s_api::storage::v1::VolumeAttributesClassList>
    for k8s_api::storage::v1alpha1::VolumeAttributesClassList
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::storage::v1::VolumeAttributesClassList, ConversionError> {
        let items = self
            .items
            .iter()
            .map(|item| item.convert_to())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(k8s_api::storage::v1::VolumeAttributesClassList {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1",
                "VolumeAttributesClassList",
            ),
            metadata: self.metadata.clone(),
            items,
        })
    }

    fn convert_from(
        other: &k8s_api::storage::v1::VolumeAttributesClassList,
    ) -> Result<Self, ConversionError> {
        let items = other
            .items
            .iter()
            .map(|item| k8s_api::storage::v1alpha1::VolumeAttributesClass::convert_from(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            type_meta: k8s_apimachinery::apis::meta::v1::TypeMeta::new(
                "storage.k8s.io/v1alpha1",
                "VolumeAttributesClassList",
            ),
            metadata: other.metadata.clone(),
            items,
        })
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;
    use std::collections::HashMap;

    #[test]
    fn test_storage_class_v1beta1_to_v1() {
        let v1beta1 = k8s_api::storage::v1beta1::StorageClass {
            metadata: ObjectMeta::named("fast-storage"),
            provisioner: "kubernetes.io/aws-ebs".to_string(),
            parameters: {
                let mut params = HashMap::new();
                params.insert("type".to_string(), "gp2".to_string());
                params
            },
            reclaim_policy: Some("Delete".to_string()),
            mount_options: vec!["debug".to_string()],
            allow_volume_expansion: Some(true),
            volume_binding_mode: Some("WaitForFirstConsumer".to_string()),
            allowed_topologies: vec![k8s_api::storage::v1beta1::TopologySelectorTerm {
                match_label_expressions: vec![
                    k8s_api::storage::v1beta1::TopologySelectorLabelRequirement {
                        key: "topology.kubernetes.io/zone".to_string(),
                        values: vec!["us-east-1a".to_string(), "us-east-1b".to_string()],
                    },
                ],
            }],
            ..Default::default()
        };

        let v1: k8s_api::storage::v1::StorageClass = v1beta1.convert_to().unwrap();

        assert_eq!(v1.metadata.name, "fast-storage");
        assert_eq!(v1.provisioner, "kubernetes.io/aws-ebs");
        assert_eq!(v1.parameters.get("type"), Some(&"gp2".to_string()));
        assert_eq!(v1.reclaim_policy, Some("Delete".to_string()));
        assert_eq!(v1.mount_options, vec!["debug"]);
        assert_eq!(v1.allow_volume_expansion, Some(true));
        assert_eq!(v1.volume_binding_mode, Some("WaitForFirstConsumer".to_string()));
        assert_eq!(v1.allowed_topologies.len(), 1);
        assert_eq!(
            v1.allowed_topologies[0].match_label_expressions[0].key,
            "topology.kubernetes.io/zone"
        );
    }

    #[test]
    fn test_storage_class_v1_to_v1beta1() {
        let v1 = k8s_api::storage::v1::StorageClass {
            metadata: ObjectMeta::named("standard"),
            provisioner: "kubernetes.io/gce-pd".to_string(),
            reclaim_policy: Some("Retain".to_string()),
            ..Default::default()
        };

        let v1beta1: k8s_api::storage::v1beta1::StorageClass =
            k8s_api::storage::v1beta1::StorageClass::convert_from(&v1).unwrap();

        assert_eq!(v1beta1.metadata.name, "standard");
        assert_eq!(v1beta1.provisioner, "kubernetes.io/gce-pd");
        assert_eq!(v1beta1.reclaim_policy, Some("Retain".to_string()));
    }

    #[test]
    fn test_storage_class_roundtrip() {
        let original = k8s_api::storage::v1beta1::StorageClass {
            metadata: ObjectMeta::named("roundtrip-sc"),
            provisioner: "ebs.csi.aws.com".to_string(),
            parameters: {
                let mut params = HashMap::new();
                params.insert("type".to_string(), "gp3".to_string());
                params.insert("iops".to_string(), "3000".to_string());
                params
            },
            allow_volume_expansion: Some(true),
            ..Default::default()
        };

        let v1: k8s_api::storage::v1::StorageClass = original.convert_to().unwrap();
        let back: k8s_api::storage::v1beta1::StorageClass =
            k8s_api::storage::v1beta1::StorageClass::convert_from(&v1).unwrap();

        assert_eq!(back.metadata.name, "roundtrip-sc");
        assert_eq!(back.provisioner, "ebs.csi.aws.com");
        assert_eq!(back.parameters.get("type"), Some(&"gp3".to_string()));
        assert_eq!(back.parameters.get("iops"), Some(&"3000".to_string()));
        assert_eq!(back.allow_volume_expansion, Some(true));
    }

    #[test]
    fn test_volume_attachment_v1beta1_to_v1() {
        let v1beta1 = k8s_api::storage::v1beta1::VolumeAttachment {
            metadata: ObjectMeta::named("csi-vol-attachment"),
            spec: k8s_api::storage::v1beta1::VolumeAttachmentSpec {
                attacher: "ebs.csi.aws.com".to_string(),
                source: k8s_api::storage::v1beta1::VolumeAttachmentSource {
                    persistent_volume_name: Some("pv-123".to_string()),
                    inline_volume_spec: None,
                },
                node_name: "node-1".to_string(),
            },
            status: Some(k8s_api::storage::v1beta1::VolumeAttachmentStatus {
                attached: true,
                attachment_metadata: {
                    let mut meta = HashMap::new();
                    meta.insert("volumeId".to_string(), "vol-abc123".to_string());
                    meta
                },
                attach_error: None,
                detach_error: None,
            }),
            ..Default::default()
        };

        let v1: k8s_api::storage::v1::VolumeAttachment = v1beta1.convert_to().unwrap();

        assert_eq!(v1.metadata.name, "csi-vol-attachment");
        assert_eq!(v1.spec.attacher, "ebs.csi.aws.com");
        assert_eq!(v1.spec.source.persistent_volume_name, Some("pv-123".to_string()));
        assert_eq!(v1.spec.node_name, "node-1");
        assert!(v1.status.as_ref().unwrap().attached);
        assert_eq!(
            v1.status.as_ref().unwrap().attachment_metadata.get("volumeId"),
            Some(&"vol-abc123".to_string())
        );
    }

    #[test]
    fn test_csi_driver_v1beta1_to_v1() {
        let v1beta1 = k8s_api::storage::v1beta1::CSIDriver {
            metadata: ObjectMeta::named("ebs.csi.aws.com"),
            spec: k8s_api::storage::v1beta1::CSIDriverSpec {
                attach_required: Some(true),
                pod_info_on_mount: Some(false),
                volume_lifecycle_modes: vec!["Persistent".to_string()],
                storage_capacity: Some(true),
                fs_group_policy: Some("File".to_string()),
                token_requests: vec![k8s_api::storage::v1beta1::TokenRequest {
                    audience: "sts.amazonaws.com".to_string(),
                    expiration_seconds: Some(3600),
                }],
                requires_republish: Some(false),
                se_linux_mount: Some(false),
                node_allocatable_update_period_seconds: Some(60),
            },
            ..Default::default()
        };

        let v1: k8s_api::storage::v1::CSIDriver = v1beta1.convert_to().unwrap();

        assert_eq!(v1.metadata.name, "ebs.csi.aws.com");
        assert_eq!(v1.spec.attach_required, Some(true));
        assert_eq!(v1.spec.pod_info_on_mount, Some(false));
        assert_eq!(v1.spec.volume_lifecycle_modes, vec!["Persistent"]);
        assert_eq!(v1.spec.storage_capacity, Some(true));
        assert_eq!(v1.spec.fs_group_policy, Some("File".to_string()));
        assert_eq!(v1.spec.token_requests.len(), 1);
        assert_eq!(v1.spec.token_requests[0].audience, "sts.amazonaws.com");
        assert_eq!(v1.spec.token_requests[0].expiration_seconds, Some(3600));
    }

    #[test]
    fn test_csi_driver_v1_to_v1beta1() {
        let v1 = k8s_api::storage::v1::CSIDriver {
            metadata: ObjectMeta::named("csi.example.com"),
            spec: k8s_api::storage::v1::CSIDriverSpec {
                attach_required: Some(false),
                pod_info_on_mount: Some(true),
                storage_capacity: Some(false),
                ..Default::default()
            },
            ..Default::default()
        };

        let v1beta1: k8s_api::storage::v1beta1::CSIDriver =
            k8s_api::storage::v1beta1::CSIDriver::convert_from(&v1).unwrap();

        assert_eq!(v1beta1.metadata.name, "csi.example.com");
        assert_eq!(v1beta1.spec.attach_required, Some(false));
        assert_eq!(v1beta1.spec.pod_info_on_mount, Some(true));
        // v1 doesn't have node_allocatable_update_period_seconds, so it should be None
        assert_eq!(v1beta1.spec.node_allocatable_update_period_seconds, None);
    }

    #[test]
    fn test_csi_node_v1beta1_to_v1() {
        let v1beta1 = k8s_api::storage::v1beta1::CSINode {
            metadata: ObjectMeta::named("node-1"),
            spec: k8s_api::storage::v1beta1::CSINodeSpec {
                drivers: vec![
                    k8s_api::storage::v1beta1::CSINodeDriver {
                        name: "ebs.csi.aws.com".to_string(),
                        node_id: "i-1234567890abcdef0".to_string(),
                        topology_keys: vec!["topology.ebs.csi.aws.com/zone".to_string()],
                        allocatable: Some(k8s_api::storage::v1beta1::VolumeNodeResources {
                            count: Some(25),
                        }),
                    },
                    k8s_api::storage::v1beta1::CSINodeDriver {
                        name: "efs.csi.aws.com".to_string(),
                        node_id: "i-1234567890abcdef0".to_string(),
                        topology_keys: vec![],
                        allocatable: None,
                    },
                ],
            },
            ..Default::default()
        };

        let v1: k8s_api::storage::v1::CSINode = v1beta1.convert_to().unwrap();

        assert_eq!(v1.metadata.name, "node-1");
        assert_eq!(v1.spec.drivers.len(), 2);
        assert_eq!(v1.spec.drivers[0].name, "ebs.csi.aws.com");
        assert_eq!(v1.spec.drivers[0].node_id, "i-1234567890abcdef0");
        assert_eq!(v1.spec.drivers[0].allocatable.as_ref().unwrap().count, Some(25));
        assert_eq!(v1.spec.drivers[1].name, "efs.csi.aws.com");
        assert!(v1.spec.drivers[1].allocatable.is_none());
    }

    #[test]
    fn test_csi_storage_capacity_v1beta1_to_v1() {
        let v1beta1 = k8s_api::storage::v1beta1::CSIStorageCapacity {
            metadata: ObjectMeta {
                name: "cap-1".to_string(),
                namespace: "kube-system".to_string(),
                ..Default::default()
            },
            storage_class_name: "fast-storage".to_string(),
            node_topology: Some(k8s_apimachinery::apis::meta::v1::LabelSelector {
                match_labels: {
                    let mut labels = std::collections::BTreeMap::new();
                    labels.insert(
                        "topology.kubernetes.io/zone".to_string(),
                        "us-east-1a".to_string(),
                    );
                    labels
                },
                ..Default::default()
            }),
            capacity: Some("100Gi".to_string()),
            maximum_volume_size: Some("16Ti".to_string()),
            ..Default::default()
        };

        let v1: k8s_api::storage::v1::CSIStorageCapacity = v1beta1.convert_to().unwrap();

        assert_eq!(v1.metadata.name, "cap-1");
        assert_eq!(v1.metadata.namespace, "kube-system");
        assert_eq!(v1.storage_class_name, "fast-storage");
        assert!(v1.node_topology.is_some());
        assert_eq!(v1.capacity, Some("100Gi".to_string()));
        assert_eq!(v1.maximum_volume_size, Some("16Ti".to_string()));
    }

    #[test]
    fn test_csi_storage_capacity_roundtrip() {
        let original = k8s_api::storage::v1beta1::CSIStorageCapacity {
            metadata: ObjectMeta::named("roundtrip-cap"),
            storage_class_name: "standard".to_string(),
            capacity: Some("500Gi".to_string()),
            ..Default::default()
        };

        let v1: k8s_api::storage::v1::CSIStorageCapacity = original.convert_to().unwrap();
        let back: k8s_api::storage::v1beta1::CSIStorageCapacity =
            k8s_api::storage::v1beta1::CSIStorageCapacity::convert_from(&v1).unwrap();

        assert_eq!(back.metadata.name, "roundtrip-cap");
        assert_eq!(back.storage_class_name, "standard");
        assert_eq!(back.capacity, Some("500Gi".to_string()));
    }

    #[test]
    fn test_volume_attachment_v1alpha1_to_v1() {
        let v1alpha1 = k8s_api::storage::v1alpha1::VolumeAttachment {
            metadata: ObjectMeta::named("alpha-attachment"),
            spec: k8s_api::storage::v1alpha1::VolumeAttachmentSpec {
                attacher: "csi.example.com".to_string(),
                source: k8s_api::storage::v1alpha1::VolumeAttachmentSource {
                    persistent_volume_name: Some("pv-alpha".to_string()),
                    inline_volume_spec: None,
                },
                node_name: "node-a".to_string(),
            },
            ..Default::default()
        };

        let v1: k8s_api::storage::v1::VolumeAttachment = v1alpha1.convert_to().unwrap();

        assert_eq!(v1.metadata.name, "alpha-attachment");
        assert_eq!(v1.spec.attacher, "csi.example.com");
        assert_eq!(
            v1.spec.source.persistent_volume_name,
            Some("pv-alpha".to_string())
        );
    }

    #[test]
    fn test_volume_attributes_class_v1beta1_to_v1() {
        let v1beta1 = k8s_api::storage::v1beta1::VolumeAttributesClass {
            metadata: ObjectMeta::named("attr-class"),
            driver_name: "csi.example.com".to_string(),
            parameters: {
                let mut params = HashMap::new();
                params.insert("key".to_string(), "value".to_string());
                params
            },
            ..Default::default()
        };

        let v1: k8s_api::storage::v1::VolumeAttributesClass = v1beta1.convert_to().unwrap();
        assert_eq!(v1.metadata.name, "attr-class");
        assert_eq!(v1.driver_name, "csi.example.com");
        assert_eq!(v1.parameters.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_volume_attributes_class_v1alpha1_roundtrip() {
        let v1alpha1 = k8s_api::storage::v1alpha1::VolumeAttributesClass {
            metadata: ObjectMeta::named("alpha-class"),
            driver_name: "csi.alpha.com".to_string(),
            parameters: {
                let mut params = HashMap::new();
                params.insert("mode".to_string(), "fast".to_string());
                params
            },
            ..Default::default()
        };

        let v1: k8s_api::storage::v1::VolumeAttributesClass = v1alpha1.convert_to().unwrap();
        let back: k8s_api::storage::v1alpha1::VolumeAttributesClass =
            k8s_api::storage::v1alpha1::VolumeAttributesClass::convert_from(&v1).unwrap();

        assert_eq!(back.metadata.name, "alpha-class");
        assert_eq!(back.driver_name, "csi.alpha.com");
        assert_eq!(back.parameters.get("mode"), Some(&"fast".to_string()));
    }
}
