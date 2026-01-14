//! Storage API validation
//!
//! This module provides validation for storage API types including:
//! - StorageClass
//! - VolumeAttachment
//! - CSIDriver
//! - CSINode
//! - CSIStorageCapacity

use crate::common::{validate_dns_subdomain_name, validate_object_meta};
use crate::{ValidationError, ValidationResult};
use k8s_api::storage::v1::{
    CSIDriver, CSIDriverSpec, CSINode, CSINodeDriver, CSIStorageCapacity, StorageClass,
    VolumeAttachment, VolumeAttachmentSource, VolumeAttachmentSpec,
};

/// Valid reclaim policies
const VALID_RECLAIM_POLICIES: &[&str] = &["Retain", "Delete", "Recycle"];

/// Valid volume binding modes
const VALID_VOLUME_BINDING_MODES: &[&str] = &["Immediate", "WaitForFirstConsumer"];

/// Valid volume lifecycle modes
const VALID_VOLUME_LIFECYCLE_MODES: &[&str] = &["Persistent", "Ephemeral"];

/// Valid FSGroup policies
const VALID_FS_GROUP_POLICIES: &[&str] = &["File", "None", "ReadWriteOnceWithFSType"];

// =============================================================================
// StorageClass Validation
// =============================================================================

/// Validates a StorageClass resource.
pub fn validate_storage_class(sc: &StorageClass) -> ValidationResult {
    let mut errors = Vec::new();

    errors.extend(validate_object_meta(&sc.metadata, "metadata", true));

    // Provisioner is required
    if sc.provisioner.is_empty() {
        errors.push(ValidationError::required(
            "provisioner",
            "provisioner is required",
        ));
    }

    // Validate reclaimPolicy
    if let Some(policy) = &sc.reclaim_policy {
        if !VALID_RECLAIM_POLICIES.contains(&policy.as_str()) {
            errors.push(ValidationError::not_supported(
                "reclaimPolicy",
                policy,
                VALID_RECLAIM_POLICIES,
            ));
        }
    }

    // Validate volumeBindingMode
    if let Some(mode) = &sc.volume_binding_mode {
        if !VALID_VOLUME_BINDING_MODES.contains(&mode.as_str()) {
            errors.push(ValidationError::not_supported(
                "volumeBindingMode",
                mode,
                VALID_VOLUME_BINDING_MODES,
            ));
        }
    }

    // Validate allowedTopologies
    for (i, topology) in sc.allowed_topologies.iter().enumerate() {
        for (j, expr) in topology.match_label_expressions.iter().enumerate() {
            if expr.key.is_empty() {
                errors.push(ValidationError::required(
                    format!("allowedTopologies[{}].matchLabelExpressions[{}].key", i, j),
                    "key is required",
                ));
            }
            if expr.values.is_empty() {
                errors.push(ValidationError::required(
                    format!("allowedTopologies[{}].matchLabelExpressions[{}].values", i, j),
                    "values is required",
                ));
            }
        }
    }

    errors
}

// =============================================================================
// VolumeAttachment Validation
// =============================================================================

/// Validates a VolumeAttachment resource.
pub fn validate_volume_attachment(va: &VolumeAttachment) -> ValidationResult {
    let mut errors = Vec::new();

    errors.extend(validate_object_meta(&va.metadata, "metadata", true));
    errors.extend(validate_volume_attachment_spec(&va.spec, "spec"));

    errors
}

/// Validates a VolumeAttachmentSpec.
pub fn validate_volume_attachment_spec(spec: &VolumeAttachmentSpec, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Attacher is required
    if spec.attacher.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.attacher", field),
            "attacher is required",
        ));
    }

    // NodeName is required
    if spec.node_name.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.nodeName", field),
            "nodeName is required",
        ));
    } else {
        errors.extend(validate_dns_subdomain_name(
            &spec.node_name,
            &format!("{}.nodeName", field),
        ));
    }

    // Validate source
    errors.extend(validate_volume_attachment_source(
        &spec.source,
        &format!("{}.source", field),
    ));

    errors
}

/// Validates a VolumeAttachmentSource.
pub fn validate_volume_attachment_source(
    source: &VolumeAttachmentSource,
    field: &str,
) -> ValidationResult {
    let mut errors = Vec::new();

    // Must have either persistentVolumeName or inlineVolumeSpec
    let has_pv_name = source.persistent_volume_name.as_ref().map_or(false, |s| !s.is_empty());
    let has_inline = source.inline_volume_spec.is_some();

    if !has_pv_name && !has_inline {
        errors.push(ValidationError::required(
            field,
            "either persistentVolumeName or inlineVolumeSpec is required",
        ));
    }

    if has_pv_name && has_inline {
        errors.push(ValidationError::invalid(
            field,
            "cannot specify both persistentVolumeName and inlineVolumeSpec",
        ));
    }

    // Validate persistentVolumeName if present
    if let Some(pv_name) = &source.persistent_volume_name {
        if !pv_name.is_empty() {
            errors.extend(validate_dns_subdomain_name(
                pv_name,
                &format!("{}.persistentVolumeName", field),
            ));
        }
    }

    errors
}

// =============================================================================
// CSIDriver Validation
// =============================================================================

/// Validates a CSIDriver resource.
pub fn validate_csi_driver(driver: &CSIDriver) -> ValidationResult {
    let mut errors = Vec::new();

    errors.extend(validate_object_meta(&driver.metadata, "metadata", true));
    errors.extend(validate_csi_driver_spec(&driver.spec, "spec"));

    errors
}

/// Validates a CSIDriverSpec.
pub fn validate_csi_driver_spec(spec: &CSIDriverSpec, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate volumeLifecycleModes
    for (i, mode) in spec.volume_lifecycle_modes.iter().enumerate() {
        if !VALID_VOLUME_LIFECYCLE_MODES.contains(&mode.as_str()) {
            errors.push(ValidationError::not_supported(
                format!("{}.volumeLifecycleModes[{}]", field, i),
                mode,
                VALID_VOLUME_LIFECYCLE_MODES,
            ));
        }
    }

    // Validate fsGroupPolicy
    if let Some(policy) = &spec.fs_group_policy {
        if !VALID_FS_GROUP_POLICIES.contains(&policy.as_str()) {
            errors.push(ValidationError::not_supported(
                format!("{}.fsGroupPolicy", field),
                policy,
                VALID_FS_GROUP_POLICIES,
            ));
        }
    }

    // Validate tokenRequests
    for (i, tr) in spec.token_requests.iter().enumerate() {
        if tr.audience.is_empty() {
            errors.push(ValidationError::required(
                format!("{}.tokenRequests[{}].audience", field, i),
                "audience is required",
            ));
        }

        // Validate expirationSeconds if present
        if let Some(exp) = tr.expiration_seconds {
            if exp < 600 {
                errors.push(ValidationError::out_of_range(
                    format!("{}.tokenRequests[{}].expirationSeconds", field, i),
                    600,
                    i64::MAX,
                    exp,
                ));
            }
        }
    }

    errors
}

// =============================================================================
// CSINode Validation
// =============================================================================

/// Validates a CSINode resource.
pub fn validate_csi_node(node: &CSINode) -> ValidationResult {
    let mut errors = Vec::new();

    errors.extend(validate_object_meta(&node.metadata, "metadata", true));

    // Validate drivers
    let mut seen_drivers = std::collections::HashSet::new();
    for (i, driver) in node.spec.drivers.iter().enumerate() {
        errors.extend(validate_csi_node_driver(driver, &format!("spec.drivers[{}]", i)));

        // Check for duplicate drivers
        if !driver.name.is_empty() {
            if !seen_drivers.insert(&driver.name) {
                errors.push(ValidationError::duplicate(
                    format!("spec.drivers[{}].name", i),
                    &driver.name,
                ));
            }
        }
    }

    errors
}

/// Validates a CSINodeDriver.
pub fn validate_csi_node_driver(driver: &CSINodeDriver, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Name is required
    if driver.name.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.name", field),
            "name is required",
        ));
    }

    // NodeID is required
    if driver.node_id.is_empty() {
        errors.push(ValidationError::required(
            format!("{}.nodeID", field),
            "nodeID is required",
        ));
    }

    // Validate allocatable if present
    if let Some(allocatable) = &driver.allocatable {
        if let Some(count) = allocatable.count {
            if count < 0 {
                errors.push(ValidationError::out_of_range(
                    format!("{}.allocatable.count", field),
                    0,
                    i32::MAX as i64,
                    count as i64,
                ));
            }
        }
    }

    errors
}

// =============================================================================
// CSIStorageCapacity Validation
// =============================================================================

/// Validates a CSIStorageCapacity resource.
pub fn validate_csi_storage_capacity(capacity: &CSIStorageCapacity) -> ValidationResult {
    let mut errors = Vec::new();

    errors.extend(validate_object_meta(&capacity.metadata, "metadata", true));

    // StorageClassName is required
    if capacity.storage_class_name.is_empty() {
        errors.push(ValidationError::required(
            "storageClassName",
            "storageClassName is required",
        ));
    } else {
        errors.extend(validate_dns_subdomain_name(
            &capacity.storage_class_name,
            "storageClassName",
        ));
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_api::storage::v1::*;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_validate_storage_class_valid() {
        let sc = StorageClass {
            metadata: ObjectMeta::named("standard"),
            provisioner: "kubernetes.io/gce-pd".to_string(),
            reclaim_policy: Some("Delete".to_string()),
            volume_binding_mode: Some("Immediate".to_string()),
            ..Default::default()
        };

        let errors = validate_storage_class(&sc);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_storage_class_missing_provisioner() {
        let sc = StorageClass {
            metadata: ObjectMeta::named("test"),
            provisioner: "".to_string(),
            ..Default::default()
        };

        let errors = validate_storage_class(&sc);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("provisioner")));
    }

    #[test]
    fn test_validate_storage_class_invalid_reclaim_policy() {
        let sc = StorageClass {
            metadata: ObjectMeta::named("test"),
            provisioner: "test-provisioner".to_string(),
            reclaim_policy: Some("Invalid".to_string()),
            ..Default::default()
        };

        let errors = validate_storage_class(&sc);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("reclaimPolicy")));
    }

    #[test]
    fn test_validate_storage_class_invalid_binding_mode() {
        let sc = StorageClass {
            metadata: ObjectMeta::named("test"),
            provisioner: "test-provisioner".to_string(),
            volume_binding_mode: Some("Invalid".to_string()),
            ..Default::default()
        };

        let errors = validate_storage_class(&sc);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("volumeBindingMode")));
    }

    #[test]
    fn test_validate_storage_class_with_topology() {
        let sc = StorageClass {
            metadata: ObjectMeta::named("topology-aware"),
            provisioner: "test-provisioner".to_string(),
            allowed_topologies: vec![TopologySelectorTerm {
                match_label_expressions: vec![TopologySelectorLabelRequirement {
                    key: "topology.kubernetes.io/zone".to_string(),
                    values: vec!["us-east-1a".to_string(), "us-east-1b".to_string()],
                }],
            }],
            ..Default::default()
        };

        let errors = validate_storage_class(&sc);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_volume_attachment_valid() {
        let va = VolumeAttachment {
            metadata: ObjectMeta::named("test-attachment"),
            spec: VolumeAttachmentSpec {
                attacher: "csi.example.com".to_string(),
                node_name: "node-1".to_string(),
                source: VolumeAttachmentSource {
                    persistent_volume_name: Some("pv-test".to_string()),
                    ..Default::default()
                },
            },
            ..Default::default()
        };

        let errors = validate_volume_attachment(&va);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_volume_attachment_missing_attacher() {
        let va = VolumeAttachment {
            metadata: ObjectMeta::named("test"),
            spec: VolumeAttachmentSpec {
                attacher: "".to_string(),
                node_name: "node-1".to_string(),
                source: VolumeAttachmentSource {
                    persistent_volume_name: Some("pv-test".to_string()),
                    ..Default::default()
                },
            },
            ..Default::default()
        };

        let errors = validate_volume_attachment(&va);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("attacher")));
    }

    #[test]
    fn test_validate_volume_attachment_missing_source() {
        let va = VolumeAttachment {
            metadata: ObjectMeta::named("test"),
            spec: VolumeAttachmentSpec {
                attacher: "csi.example.com".to_string(),
                node_name: "node-1".to_string(),
                source: VolumeAttachmentSource::default(),
            },
            ..Default::default()
        };

        let errors = validate_volume_attachment(&va);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("source")));
    }

    #[test]
    fn test_validate_csi_driver_valid() {
        let driver = CSIDriver {
            metadata: ObjectMeta::named("csi.example.com"),
            spec: CSIDriverSpec {
                attach_required: Some(true),
                pod_info_on_mount: Some(true),
                volume_lifecycle_modes: vec!["Persistent".to_string()],
                fs_group_policy: Some("File".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validate_csi_driver(&driver);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_csi_driver_invalid_lifecycle_mode() {
        let driver = CSIDriver {
            metadata: ObjectMeta::named("csi.example.com"),
            spec: CSIDriverSpec {
                volume_lifecycle_modes: vec!["Invalid".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validate_csi_driver(&driver);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("volumeLifecycleModes")));
    }

    #[test]
    fn test_validate_csi_driver_invalid_fs_group_policy() {
        let driver = CSIDriver {
            metadata: ObjectMeta::named("csi.example.com"),
            spec: CSIDriverSpec {
                fs_group_policy: Some("Invalid".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validate_csi_driver(&driver);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("fsGroupPolicy")));
    }

    #[test]
    fn test_validate_csi_node_valid() {
        let node = CSINode {
            metadata: ObjectMeta::named("node-1"),
            spec: CSINodeSpec {
                drivers: vec![CSINodeDriver {
                    name: "csi.example.com".to_string(),
                    node_id: "node-1-id".to_string(),
                    topology_keys: vec!["topology.kubernetes.io/zone".to_string()],
                    ..Default::default()
                }],
            },
            ..Default::default()
        };

        let errors = validate_csi_node(&node);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_csi_node_missing_driver_name() {
        let node = CSINode {
            metadata: ObjectMeta::named("node-1"),
            spec: CSINodeSpec {
                drivers: vec![CSINodeDriver {
                    name: "".to_string(),
                    node_id: "node-1-id".to_string(),
                    ..Default::default()
                }],
            },
            ..Default::default()
        };

        let errors = validate_csi_node(&node);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("name")));
    }

    #[test]
    fn test_validate_csi_node_duplicate_drivers() {
        let node = CSINode {
            metadata: ObjectMeta::named("node-1"),
            spec: CSINodeSpec {
                drivers: vec![
                    CSINodeDriver {
                        name: "csi.example.com".to_string(),
                        node_id: "id-1".to_string(),
                        ..Default::default()
                    },
                    CSINodeDriver {
                        name: "csi.example.com".to_string(), // Duplicate
                        node_id: "id-2".to_string(),
                        ..Default::default()
                    },
                ],
            },
            ..Default::default()
        };

        let errors = validate_csi_node(&node);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.message.contains("duplicate")));
    }

    #[test]
    fn test_validate_csi_storage_capacity_valid() {
        let capacity = CSIStorageCapacity {
            metadata: ObjectMeta::named("test-capacity"),
            storage_class_name: "standard".to_string(),
            capacity: Some("100Gi".to_string()),
            ..Default::default()
        };

        let errors = validate_csi_storage_capacity(&capacity);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_csi_storage_capacity_missing_storage_class() {
        let capacity = CSIStorageCapacity {
            metadata: ObjectMeta::named("test-capacity"),
            storage_class_name: "".to_string(),
            ..Default::default()
        };

        let errors = validate_csi_storage_capacity(&capacity);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("storageClassName")));
    }

    #[test]
    fn test_validate_csi_driver_with_token_requests() {
        let driver = CSIDriver {
            metadata: ObjectMeta::named("csi.example.com"),
            spec: CSIDriverSpec {
                token_requests: vec![TokenRequest {
                    audience: "api".to_string(),
                    expiration_seconds: Some(3600),
                }],
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validate_csi_driver(&driver);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_csi_driver_token_request_invalid_expiration() {
        let driver = CSIDriver {
            metadata: ObjectMeta::named("csi.example.com"),
            spec: CSIDriverSpec {
                token_requests: vec![TokenRequest {
                    audience: "api".to_string(),
                    expiration_seconds: Some(300), // Less than 600 minimum
                }],
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validate_csi_driver(&driver);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("expirationSeconds")));
    }
}
