//! Node API validation
//!
//! This module provides validation for node API types including:
//! - RuntimeClass

use crate::common::{validate_labels, validate_object_meta, validate_quantity};
use crate::{ValidationError, ValidationResult};
use k8s_api::node::v1::{Overhead, RuntimeClass, Scheduling, Toleration};

/// Valid toleration operators
const VALID_TOLERATION_OPERATORS: &[&str] = &["Exists", "Equal"];

/// Valid toleration effects
const VALID_TOLERATION_EFFECTS: &[&str] = &["NoSchedule", "PreferNoSchedule", "NoExecute", ""];

/// Maximum length for handler name
const MAX_HANDLER_LENGTH: usize = 253;

// =============================================================================
// RuntimeClass Validation
// =============================================================================

/// Validates a RuntimeClass resource.
pub fn validate_runtime_class(rc: &RuntimeClass) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate metadata
    errors.extend(validate_object_meta(&rc.metadata, "metadata", true));

    // Handler is required
    if rc.handler.is_empty() {
        errors.push(ValidationError::required(
            "handler",
            "handler is required",
        ));
    } else {
        if rc.handler.len() > MAX_HANDLER_LENGTH {
            errors.push(ValidationError::too_long(
                "handler",
                MAX_HANDLER_LENGTH,
                rc.handler.len(),
            ));
        }
        // Handler should be a valid DNS label-like name
        if !is_valid_handler_name(&rc.handler) {
            errors.push(ValidationError::invalid(
                "handler",
                "must consist of alphanumeric characters, '-', '_', or '.', and start with an alphanumeric character",
            ));
        }
    }

    // Validate overhead if present
    if let Some(overhead) = &rc.overhead {
        errors.extend(validate_overhead(overhead, "overhead"));
    }

    // Validate scheduling if present
    if let Some(scheduling) = &rc.scheduling {
        errors.extend(validate_scheduling(scheduling, "scheduling"));
    }

    errors
}

/// Validates Overhead.
fn validate_overhead(overhead: &Overhead, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate pod_fixed resources
    for (resource_name, quantity) in &overhead.pod_fixed {
        let quantity_field = format!("{}.podFixed[{}]", field, resource_name);
        errors.extend(validate_quantity(quantity, &quantity_field));

        // Resource quantities should not be negative
        if quantity.starts_with('-') {
            errors.push(ValidationError::invalid(
                &quantity_field,
                "resource overhead cannot be negative",
            ));
        }
    }

    errors
}

/// Validates Scheduling.
fn validate_scheduling(scheduling: &Scheduling, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate node_selector
    errors.extend(validate_labels(
        &scheduling.node_selector,
        &format!("{}.nodeSelector", field),
    ));

    // Validate tolerations
    for (i, toleration) in scheduling.tolerations.iter().enumerate() {
        errors.extend(validate_toleration(
            toleration,
            &format!("{}.tolerations[{}]", field, i),
        ));
    }

    errors
}

/// Validates a Toleration.
fn validate_toleration(toleration: &Toleration, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate operator
    if !toleration.operator.is_empty()
        && !VALID_TOLERATION_OPERATORS.contains(&toleration.operator.as_str())
    {
        errors.push(ValidationError::not_supported(
            format!("{}.operator", field),
            &toleration.operator,
            VALID_TOLERATION_OPERATORS,
        ));
    }

    // Validate effect
    if !VALID_TOLERATION_EFFECTS.contains(&toleration.effect.as_str()) {
        errors.push(ValidationError::not_supported(
            format!("{}.effect", field),
            &toleration.effect,
            VALID_TOLERATION_EFFECTS,
        ));
    }

    // If operator is Exists, value should be empty
    if toleration.operator == "Exists" && !toleration.value.is_empty() {
        errors.push(ValidationError::invalid(
            format!("{}.value", field),
            "value must be empty when operator is Exists",
        ));
    }

    // If operator is Equal or empty (defaults to Equal), key must be set
    if (toleration.operator.is_empty() || toleration.operator == "Equal")
        && toleration.key.is_empty()
        && !toleration.value.is_empty()
    {
        errors.push(ValidationError::required(
            format!("{}.key", field),
            "key is required when value is set",
        ));
    }

    // Validate toleration_seconds (only valid for NoExecute effect)
    if let Some(seconds) = toleration.toleration_seconds {
        if toleration.effect != "NoExecute" && !toleration.effect.is_empty() {
            errors.push(ValidationError::invalid(
                format!("{}.tolerationSeconds", field),
                "tolerationSeconds is only valid for NoExecute effect",
            ));
        }
        if seconds < 0 {
            errors.push(ValidationError::invalid(
                format!("{}.tolerationSeconds", field),
                "tolerationSeconds cannot be negative",
            ));
        }
    }

    errors
}

/// Checks if a handler name is valid.
fn is_valid_handler_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    // Must start with alphanumeric
    let first_char = name.chars().next().unwrap();
    if !first_char.is_ascii_alphanumeric() {
        return false;
    }

    // Can contain alphanumeric, '-', '_', '.'
    name.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;
    use std::collections::BTreeMap;

    #[test]
    fn test_validate_runtime_class_valid() {
        let rc = RuntimeClass {
            metadata: ObjectMeta {
                name: "gvisor".to_string(),
                ..Default::default()
            },
            handler: "runsc".to_string(),
            ..Default::default()
        };

        let errors = validate_runtime_class(&rc);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_runtime_class_with_overhead() {
        let mut pod_fixed = BTreeMap::new();
        pod_fixed.insert("memory".to_string(), "100Mi".to_string());
        pod_fixed.insert("cpu".to_string(), "100m".to_string());

        let rc = RuntimeClass {
            metadata: ObjectMeta {
                name: "kata-containers".to_string(),
                ..Default::default()
            },
            handler: "kata".to_string(),
            overhead: Some(Overhead { pod_fixed }),
            ..Default::default()
        };

        let errors = validate_runtime_class(&rc);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_runtime_class_with_scheduling() {
        let mut node_selector = BTreeMap::new();
        node_selector.insert(
            "kubernetes.io/os".to_string(),
            "linux".to_string(),
        );
        node_selector.insert(
            "runtime".to_string(),
            "kata".to_string(),
        );

        let rc = RuntimeClass {
            metadata: ObjectMeta {
                name: "kata".to_string(),
                ..Default::default()
            },
            handler: "kata-qemu".to_string(),
            scheduling: Some(Scheduling {
                node_selector,
                tolerations: vec![Toleration {
                    key: "node.kubernetes.io/kata".to_string(),
                    operator: "Exists".to_string(),
                    effect: "NoSchedule".to_string(),
                    ..Default::default()
                }],
            }),
            ..Default::default()
        };

        let errors = validate_runtime_class(&rc);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_runtime_class_missing_handler() {
        let rc = RuntimeClass {
            metadata: ObjectMeta {
                name: "test-runtime".to_string(),
                ..Default::default()
            },
            handler: String::new(),
            ..Default::default()
        };

        let errors = validate_runtime_class(&rc);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("handler")));
    }

    #[test]
    fn test_validate_runtime_class_invalid_handler() {
        let rc = RuntimeClass {
            metadata: ObjectMeta {
                name: "test-runtime".to_string(),
                ..Default::default()
            },
            handler: "-invalid-handler".to_string(),
            ..Default::default()
        };

        let errors = validate_runtime_class(&rc);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("handler")));
    }

    #[test]
    fn test_validate_runtime_class_invalid_overhead() {
        let mut pod_fixed = BTreeMap::new();
        pod_fixed.insert("memory".to_string(), "-100Mi".to_string());

        let rc = RuntimeClass {
            metadata: ObjectMeta {
                name: "test-runtime".to_string(),
                ..Default::default()
            },
            handler: "test-handler".to_string(),
            overhead: Some(Overhead { pod_fixed }),
            ..Default::default()
        };

        let errors = validate_runtime_class(&rc);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("overhead")));
    }

    #[test]
    fn test_validate_toleration_exists_with_value() {
        let toleration = Toleration {
            key: "node.kubernetes.io/unreachable".to_string(),
            operator: "Exists".to_string(),
            value: "should-be-empty".to_string(),
            effect: "NoExecute".to_string(),
            ..Default::default()
        };

        let errors = validate_toleration(&toleration, "test");
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("value")));
    }

    #[test]
    fn test_validate_toleration_invalid_operator() {
        let toleration = Toleration {
            key: "key".to_string(),
            operator: "Invalid".to_string(),
            effect: "NoSchedule".to_string(),
            ..Default::default()
        };

        let errors = validate_toleration(&toleration, "test");
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("operator")));
    }

    #[test]
    fn test_validate_toleration_invalid_effect() {
        let toleration = Toleration {
            key: "key".to_string(),
            operator: "Equal".to_string(),
            value: "value".to_string(),
            effect: "InvalidEffect".to_string(),
            ..Default::default()
        };

        let errors = validate_toleration(&toleration, "test");
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("effect")));
    }

    #[test]
    fn test_validate_toleration_seconds_wrong_effect() {
        let toleration = Toleration {
            key: "key".to_string(),
            operator: "Equal".to_string(),
            value: "value".to_string(),
            effect: "NoSchedule".to_string(),
            toleration_seconds: Some(300),
        };

        let errors = validate_toleration(&toleration, "test");
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field.contains("tolerationSeconds")));
    }

    #[test]
    fn test_validate_toleration_valid_no_execute() {
        let toleration = Toleration {
            key: "node.kubernetes.io/not-ready".to_string(),
            operator: "Exists".to_string(),
            effect: "NoExecute".to_string(),
            toleration_seconds: Some(300),
            ..Default::default()
        };

        let errors = validate_toleration(&toleration, "test");
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_is_valid_handler_name() {
        assert!(is_valid_handler_name("runc"));
        assert!(is_valid_handler_name("runsc"));
        assert!(is_valid_handler_name("kata-qemu"));
        assert!(is_valid_handler_name("handler_v1.0"));

        assert!(!is_valid_handler_name(""));
        assert!(!is_valid_handler_name("-invalid"));
        assert!(!is_valid_handler_name("handler with space"));
        assert!(!is_valid_handler_name(".dotstart"));
    }
}
