//! Scheduling API validation
//!
//! This module provides validation for scheduling API types including:
//! - PriorityClass (v1, v1beta1, v1alpha1)

use crate::common::validate_object_meta;
use crate::{ValidationError, ValidationResult};

const VALID_PREEMPTION_POLICIES: &[&str] = &["Never", "PreemptLowerPriority"];

fn validate_preemption_policy(value: &str, field: &str) -> ValidationResult {
    if value.is_empty() {
        vec![ValidationError::required(field, "preemptionPolicy is required")]
    } else if !VALID_PREEMPTION_POLICIES.contains(&value) {
        vec![ValidationError::not_supported(
            field,
            value,
            VALID_PREEMPTION_POLICIES,
        )]
    } else {
        Vec::new()
    }
}

pub mod v1 {
    use super::*;
    use k8s_api::scheduling::v1 as api;

    /// Validates a PriorityClass resource.
    pub fn validate_priority_class(priority_class: &api::PriorityClass) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&priority_class.metadata, "metadata", true));

        if let Some(policy) = &priority_class.preemption_policy {
            errors.extend(validate_preemption_policy(policy, "preemptionPolicy"));
        }

        errors
    }
}

pub mod v1beta1 {
    use super::*;
    use k8s_api::scheduling::v1beta1 as api;

    /// Validates a PriorityClass resource.
    pub fn validate_priority_class(priority_class: &api::PriorityClass) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&priority_class.metadata, "metadata", true));

        if let Some(policy) = &priority_class.preemption_policy {
            errors.extend(validate_preemption_policy(policy, "preemptionPolicy"));
        }

        errors
    }
}

pub mod v1alpha1 {
    use super::*;
    use k8s_api::scheduling::v1alpha1 as api;

    /// Validates a PriorityClass resource.
    pub fn validate_priority_class(priority_class: &api::PriorityClass) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&priority_class.metadata, "metadata", true));

        if let Some(policy) = &priority_class.preemption_policy {
            errors.extend(validate_preemption_policy(policy, "preemptionPolicy"));
        }

        errors
    }
}

// =============================================================================
// Tests
// =============================================================================

pub mod internal {
    use super::*;
    use k8s_api::scheduling::internal as api;

    pub fn validate_priority_class(priority_class: &api::PriorityClass) -> ValidationResult {
        crate::internal::validate_with(
            priority_class,
            "priorityClass",
            super::v1::validate_priority_class,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_validate_priority_class_valid_v1() {
        let pc = k8s_api::scheduling::v1::PriorityClass {
            metadata: ObjectMeta {
                name: "high-priority".to_string(),
                ..Default::default()
            },
            value: 1000,
            global_default: Some(false),
            description: "test".to_string(),
            preemption_policy: Some("Never".to_string()),
            ..Default::default()
        };

        let errors = v1::validate_priority_class(&pc);
        assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
    }

    #[test]
    fn test_validate_priority_class_invalid_policy_v1beta1() {
        let pc = k8s_api::scheduling::v1beta1::PriorityClass {
            metadata: ObjectMeta {
                name: "invalid-policy".to_string(),
                ..Default::default()
            },
            value: 0,
            global_default: Some(false),
            description: String::new(),
            preemption_policy: Some("Always".to_string()),
            ..Default::default()
        };

        let errors = v1beta1::validate_priority_class(&pc);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|err| err.field == "preemptionPolicy"));
    }

    #[test]
    fn test_validate_priority_class_empty_policy_v1alpha1() {
        let pc = k8s_api::scheduling::v1alpha1::PriorityClass {
            metadata: ObjectMeta {
                name: "empty-policy".to_string(),
                ..Default::default()
            },
            value: 5,
            global_default: Some(false),
            description: String::new(),
            preemption_policy: Some(String::new()),
            ..Default::default()
        };

        let errors = v1alpha1::validate_priority_class(&pc);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|err| err.field == "preemptionPolicy"));
    }
}
