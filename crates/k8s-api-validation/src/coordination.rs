//! Coordination API validation
//!
//! This module provides validation for coordination API types including:
//! - Lease

use crate::common::validate_object_meta;
use crate::{ValidationError, ValidationResult};
use k8s_api::coordination::v1::{Lease, LeaseSpec};

/// Maximum length for holder identity
const MAX_HOLDER_IDENTITY_LENGTH: usize = 253;

/// Maximum lease duration in seconds (about 1 year)
const MAX_LEASE_DURATION_SECONDS: i32 = 365 * 24 * 60 * 60;

// =============================================================================
// Lease Validation
// =============================================================================

/// Validates a Lease resource.
pub fn validate_lease(lease: &Lease) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate metadata
    errors.extend(validate_object_meta(&lease.metadata, "metadata", true));

    // Validate spec if present
    if let Some(spec) = &lease.spec {
        errors.extend(validate_lease_spec(spec, "spec"));
    }

    errors
}

/// Validates LeaseSpec.
fn validate_lease_spec(spec: &LeaseSpec, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate holder_identity length if present
    if let Some(holder) = &spec.holder_identity {
        if holder.len() > MAX_HOLDER_IDENTITY_LENGTH {
            errors.push(ValidationError::too_long(
                format!("{}.holderIdentity", field),
                MAX_HOLDER_IDENTITY_LENGTH,
                holder.len(),
            ));
        }
    }

    // Validate lease_duration_seconds if present
    if let Some(duration) = spec.lease_duration_seconds {
        if duration <= 0 {
            errors.push(ValidationError::invalid(
                format!("{}.leaseDurationSeconds", field),
                "leaseDurationSeconds must be positive",
            ));
        }
        if duration > MAX_LEASE_DURATION_SECONDS {
            errors.push(ValidationError::out_of_range(
                format!("{}.leaseDurationSeconds", field),
                1,
                MAX_LEASE_DURATION_SECONDS as i64,
                duration as i64,
            ));
        }
    }

    // Validate lease_transitions if present
    if let Some(transitions) = spec.lease_transitions {
        if transitions < 0 {
            errors.push(ValidationError::invalid(
                format!("{}.leaseTransitions", field),
                "leaseTransitions cannot be negative",
            ));
        }
    }

    errors
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_apimachinery::apis::meta::v1::{MicroTime, ObjectMeta};

    #[test]
    fn test_validate_lease_valid() {
        let lease = Lease {
            metadata: ObjectMeta {
                name: "my-lease".to_string(),
                namespace: "default".to_string(),
                ..Default::default()
            },
            spec: Some(LeaseSpec {
                holder_identity: Some("node-1".to_string()),
                lease_duration_seconds: Some(15),
                lease_transitions: Some(3),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_lease(&lease);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_lease_without_spec() {
        let lease = Lease {
            metadata: ObjectMeta {
                name: "empty-lease".to_string(),
                namespace: "kube-system".to_string(),
                ..Default::default()
            },
            spec: None,
            ..Default::default()
        };

        let errors = validate_lease(&lease);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_lease_missing_name() {
        let lease = Lease {
            metadata: ObjectMeta {
                ..Default::default()
            },
            spec: Some(LeaseSpec::default()),
            ..Default::default()
        };

        let errors = validate_lease(&lease);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("name")));
    }

    #[test]
    fn test_validate_lease_invalid_duration() {
        let lease = Lease {
            metadata: ObjectMeta {
                name: "my-lease".to_string(),
                ..Default::default()
            },
            spec: Some(LeaseSpec {
                lease_duration_seconds: Some(0),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_lease(&lease);
        assert!(!errors.is_empty());
        assert!(errors
            .iter()
            .any(|e| e.field.contains("leaseDurationSeconds")));
    }

    #[test]
    fn test_validate_lease_negative_duration() {
        let lease = Lease {
            metadata: ObjectMeta {
                name: "my-lease".to_string(),
                ..Default::default()
            },
            spec: Some(LeaseSpec {
                lease_duration_seconds: Some(-5),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_lease(&lease);
        assert!(!errors.is_empty());
        assert!(errors
            .iter()
            .any(|e| e.field.contains("leaseDurationSeconds")));
    }

    #[test]
    fn test_validate_lease_negative_transitions() {
        let lease = Lease {
            metadata: ObjectMeta {
                name: "my-lease".to_string(),
                ..Default::default()
            },
            spec: Some(LeaseSpec {
                lease_transitions: Some(-1),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_lease(&lease);
        assert!(!errors.is_empty());
        assert!(errors
            .iter()
            .any(|e| e.field.contains("leaseTransitions")));
    }

    #[test]
    fn test_validate_lease_holder_identity_too_long() {
        let lease = Lease {
            metadata: ObjectMeta {
                name: "my-lease".to_string(),
                ..Default::default()
            },
            spec: Some(LeaseSpec {
                holder_identity: Some("a".repeat(300)),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_lease(&lease);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("holderIdentity")));
    }

    #[test]
    fn test_validate_lease_kubelet_style() {
        // This mimics how kubelet creates leases for node heartbeats
        let lease = Lease {
            metadata: ObjectMeta {
                name: "worker-node-1".to_string(),
                namespace: "kube-node-lease".to_string(),
                ..Default::default()
            },
            spec: Some(LeaseSpec {
                holder_identity: Some("worker-node-1".to_string()),
                lease_duration_seconds: Some(40),
                lease_transitions: Some(0),
                acquire_time: Some(MicroTime::now()),
                renew_time: Some(MicroTime::now()),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_lease(&lease);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_lease_leader_election_style() {
        // This mimics how leader election uses leases
        let lease = Lease {
            metadata: ObjectMeta {
                name: "kube-controller-manager".to_string(),
                namespace: "kube-system".to_string(),
                ..Default::default()
            },
            spec: Some(LeaseSpec {
                holder_identity: Some(
                    "kube-controller-manager_master-1_abc12345-6789-0123-4567-890abcdef012"
                        .to_string(),
                ),
                lease_duration_seconds: Some(15),
                lease_transitions: Some(5),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_lease(&lease);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_lease_duration_too_large() {
        let lease = Lease {
            metadata: ObjectMeta {
                name: "my-lease".to_string(),
                ..Default::default()
            },
            spec: Some(LeaseSpec {
                lease_duration_seconds: Some(MAX_LEASE_DURATION_SECONDS + 1),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_lease(&lease);
        assert!(!errors.is_empty());
        assert!(errors
            .iter()
            .any(|e| e.field.contains("leaseDurationSeconds")));
    }
}
