//! Policy API validation
//!
//! This module provides validation for policy API types including:
//! - PodDisruptionBudget
//! - Eviction

use crate::common::validate_object_meta;
use crate::{ValidationError, ValidationResult};
use k8s_api::policy::v1::{Eviction, PodDisruptionBudget, PodDisruptionBudgetSpec};

/// Valid unhealthy pod eviction policies
const VALID_UNHEALTHY_POD_EVICTION_POLICIES: &[&str] = &["IfHealthyBudget", "AlwaysAllow"];

// =============================================================================
// PodDisruptionBudget Validation
// =============================================================================

/// Validates a PodDisruptionBudget resource.
pub fn validate_pdb(pdb: &PodDisruptionBudget) -> ValidationResult {
    let mut errors = Vec::new();

    errors.extend(validate_object_meta(&pdb.metadata, "metadata", true));

    if let Some(spec) = &pdb.spec {
        errors.extend(validate_pdb_spec(spec, "spec"));
    }

    errors
}

/// Validates a PodDisruptionBudgetSpec.
pub fn validate_pdb_spec(spec: &PodDisruptionBudgetSpec, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    let has_min_available = spec.min_available.is_some();
    let has_max_unavailable = spec.max_unavailable.is_some();

    // Cannot have both minAvailable and maxUnavailable
    if has_min_available && has_max_unavailable {
        errors.push(ValidationError::invalid(
            field,
            "cannot specify both minAvailable and maxUnavailable",
        ));
    }

    // Should have at least one of minAvailable or maxUnavailable
    if !has_min_available && !has_max_unavailable {
        errors.push(ValidationError::required(
            field,
            "one of minAvailable or maxUnavailable is required",
        ));
    }

    // Validate minAvailable
    if let Some(min_available) = &spec.min_available {
        errors.extend(validate_int_or_percent(
            min_available,
            &format!("{}.minAvailable", field),
        ));
    }

    // Validate maxUnavailable
    if let Some(max_unavailable) = &spec.max_unavailable {
        errors.extend(validate_int_or_percent(
            max_unavailable,
            &format!("{}.maxUnavailable", field),
        ));
    }

    // Validate unhealthyPodEvictionPolicy
    if let Some(policy) = &spec.unhealthy_pod_eviction_policy {
        if !VALID_UNHEALTHY_POD_EVICTION_POLICIES.contains(&policy.as_str()) {
            errors.push(ValidationError::not_supported(
                format!("{}.unhealthyPodEvictionPolicy", field),
                policy,
                VALID_UNHEALTHY_POD_EVICTION_POLICIES,
            ));
        }
    }

    errors
}

/// Validates an integer or percentage value (used for minAvailable/maxUnavailable).
fn validate_int_or_percent(value: &serde_json::Value, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    match value {
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                if i < 0 {
                    errors.push(ValidationError::out_of_range(field, 0, i64::MAX, i));
                }
            } else if let Some(f) = n.as_f64() {
                if f < 0.0 {
                    errors.push(ValidationError::invalid(
                        field,
                        "value must be a non-negative integer or percentage",
                    ));
                }
            }
        }
        serde_json::Value::String(s) => {
            if s.ends_with('%') {
                // Parse percentage
                let percent_str = &s[..s.len() - 1];
                match percent_str.parse::<i32>() {
                    Ok(percent) => {
                        if percent < 0 || percent > 100 {
                            errors.push(ValidationError::out_of_range(field, 0, 100, percent as i64));
                        }
                    }
                    Err(_) => {
                        errors.push(ValidationError::invalid(
                            field,
                            format!("invalid percentage value: {}", s),
                        ));
                    }
                }
            } else {
                // Try to parse as integer
                match s.parse::<i32>() {
                    Ok(i) => {
                        if i < 0 {
                            errors.push(ValidationError::out_of_range(field, 0, i32::MAX as i64, i as i64));
                        }
                    }
                    Err(_) => {
                        errors.push(ValidationError::invalid(
                            field,
                            format!("must be an integer or percentage, got: {}", s),
                        ));
                    }
                }
            }
        }
        _ => {
            errors.push(ValidationError::invalid(
                field,
                "must be an integer or percentage string",
            ));
        }
    }

    errors
}

// =============================================================================
// Eviction Validation
// =============================================================================

/// Validates an Eviction resource.
pub fn validate_eviction(eviction: &Eviction) -> ValidationResult {
    let mut errors = Vec::new();

    // Eviction requires both name (pod name) and namespace
    if eviction.metadata.name.is_empty() {
        errors.push(ValidationError::required(
            "metadata.name",
            "pod name is required for eviction",
        ));
    }

    if eviction.metadata.namespace.is_empty() {
        errors.push(ValidationError::required(
            "metadata.namespace",
            "namespace is required for eviction",
        ));
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_api::policy::v1::*;
    use k8s_apimachinery::apis::meta::v1::{LabelSelector, ObjectMeta};
    use std::collections::BTreeMap;

    #[test]
    fn test_validate_pdb_valid_min_available_int() {
        let pdb = PodDisruptionBudget {
            metadata: ObjectMeta::named("test-pdb"),
            spec: Some(PodDisruptionBudgetSpec {
                min_available: Some(serde_json::json!(2)),
                selector: Some(LabelSelector {
                    match_labels: {
                        let mut labels = BTreeMap::new();
                        labels.insert("app".to_string(), "nginx".to_string());
                        labels
                    },
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_pdb(&pdb);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_pdb_valid_min_available_percent() {
        let pdb = PodDisruptionBudget {
            metadata: ObjectMeta::named("test-pdb"),
            spec: Some(PodDisruptionBudgetSpec {
                min_available: Some(serde_json::json!("50%")),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_pdb(&pdb);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_pdb_valid_max_unavailable() {
        let pdb = PodDisruptionBudget {
            metadata: ObjectMeta::named("test-pdb"),
            spec: Some(PodDisruptionBudgetSpec {
                max_unavailable: Some(serde_json::json!(1)),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_pdb(&pdb);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_pdb_both_min_and_max() {
        let pdb = PodDisruptionBudget {
            metadata: ObjectMeta::named("test-pdb"),
            spec: Some(PodDisruptionBudgetSpec {
                min_available: Some(serde_json::json!(2)),
                max_unavailable: Some(serde_json::json!(1)),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_pdb(&pdb);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.message.contains("cannot specify both")));
    }

    #[test]
    fn test_validate_pdb_neither_min_nor_max() {
        let pdb = PodDisruptionBudget {
            metadata: ObjectMeta::named("test-pdb"),
            spec: Some(PodDisruptionBudgetSpec::default()),
            ..Default::default()
        };

        let errors = validate_pdb(&pdb);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.message.contains("one of minAvailable or maxUnavailable")));
    }

    #[test]
    fn test_validate_pdb_negative_min_available() {
        let pdb = PodDisruptionBudget {
            metadata: ObjectMeta::named("test-pdb"),
            spec: Some(PodDisruptionBudgetSpec {
                min_available: Some(serde_json::json!(-1)),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_pdb(&pdb);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("minAvailable")));
    }

    #[test]
    fn test_validate_pdb_invalid_percent() {
        let pdb = PodDisruptionBudget {
            metadata: ObjectMeta::named("test-pdb"),
            spec: Some(PodDisruptionBudgetSpec {
                min_available: Some(serde_json::json!("150%")), // > 100%
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_pdb(&pdb);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("minAvailable")));
    }

    #[test]
    fn test_validate_pdb_invalid_percent_format() {
        let pdb = PodDisruptionBudget {
            metadata: ObjectMeta::named("test-pdb"),
            spec: Some(PodDisruptionBudgetSpec {
                min_available: Some(serde_json::json!("abc%")),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_pdb(&pdb);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("minAvailable")));
    }

    #[test]
    fn test_validate_pdb_unhealthy_policy_valid() {
        let pdb = PodDisruptionBudget {
            metadata: ObjectMeta::named("test-pdb"),
            spec: Some(PodDisruptionBudgetSpec {
                min_available: Some(serde_json::json!(1)),
                unhealthy_pod_eviction_policy: Some("IfHealthyBudget".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_pdb(&pdb);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_pdb_unhealthy_policy_always_allow() {
        let pdb = PodDisruptionBudget {
            metadata: ObjectMeta::named("test-pdb"),
            spec: Some(PodDisruptionBudgetSpec {
                min_available: Some(serde_json::json!(1)),
                unhealthy_pod_eviction_policy: Some("AlwaysAllow".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_pdb(&pdb);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_pdb_unhealthy_policy_invalid() {
        let pdb = PodDisruptionBudget {
            metadata: ObjectMeta::named("test-pdb"),
            spec: Some(PodDisruptionBudgetSpec {
                min_available: Some(serde_json::json!(1)),
                unhealthy_pod_eviction_policy: Some("Invalid".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validate_pdb(&pdb);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("unhealthyPodEvictionPolicy")));
    }

    #[test]
    fn test_validate_eviction_valid() {
        let eviction = Eviction {
            metadata: ObjectMeta {
                name: "my-pod".to_string(),
                namespace: "default".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validate_eviction(&eviction);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_eviction_missing_name() {
        let eviction = Eviction {
            metadata: ObjectMeta {
                namespace: "default".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validate_eviction(&eviction);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("name")));
    }

    #[test]
    fn test_validate_eviction_missing_namespace() {
        let eviction = Eviction {
            metadata: ObjectMeta {
                name: "my-pod".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validate_eviction(&eviction);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("namespace")));
    }

    #[test]
    fn test_validate_int_or_percent_zero() {
        // Zero is valid
        let errors = validate_int_or_percent(&serde_json::json!(0), "test");
        assert!(errors.is_empty());

        let errors = validate_int_or_percent(&serde_json::json!("0%"), "test");
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_int_or_percent_string_number() {
        // String that represents a number (without %)
        let errors = validate_int_or_percent(&serde_json::json!("5"), "test");
        assert!(errors.is_empty());
    }
}
