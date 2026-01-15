//! ABAC API validation

use crate::{ValidationError, ValidationResult};

pub mod v0 {
    use super::*;
    use k8s_api::abac::v0 as api;

    pub fn validate_policy(policy: &api::Policy) -> ValidationResult {
        let mut errors = Vec::new();

        if policy.user.is_empty() && policy.group.is_empty() {
            errors.push(ValidationError::required(
                "user",
                "user or group is required",
            ));
        }

        if policy.resource.is_empty() {
            errors.push(ValidationError::required(
                "resource",
                "resource is required",
            ));
        }

        errors
    }
}

pub mod v1beta1 {
    use super::*;
    use k8s_api::abac::v1beta1 as api;

    pub fn validate_policy(policy: &api::Policy) -> ValidationResult {
        let mut errors = Vec::new();

        if policy.spec.user.is_empty() && policy.spec.group.is_empty() {
            errors.push(ValidationError::required(
                "spec.user",
                "user or group is required",
            ));
        }

        if policy.spec.resource.is_empty() && policy.spec.non_resource_path.is_empty() {
            errors.push(ValidationError::required(
                "spec.resource",
                "resource or nonResourcePath is required",
            ));
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::v0 as validation_v0;
    use super::v1beta1 as validation_v1beta1;
    use k8s_api::abac::v0 as api_v0;
    use k8s_api::abac::v1beta1 as api_v1beta1;

    #[test]
    fn test_validate_policy_missing_subject() {
        let policy = api_v0::Policy {
            resource: "pods".to_string(),
            ..Default::default()
        };

        let errors = validation_v0::validate_policy(&policy);
        assert!(errors.iter().any(|e| e.field.contains("user")));
    }

    #[test]
    fn test_validate_policy_missing_resource() {
        let policy = api_v1beta1::Policy {
            spec: api_v1beta1::PolicySpec {
                user: "alice".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validation_v1beta1::validate_policy(&policy);
        assert!(errors.iter().any(|e| e.field.contains("spec.resource")));
    }

    #[test]
    fn test_validate_policy_valid() {
        let policy = api_v1beta1::Policy {
            spec: api_v1beta1::PolicySpec {
                user: "alice".to_string(),
                resource: "pods".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validation_v1beta1::validate_policy(&policy);
        assert!(errors.is_empty(), "expected no errors: {:?}", errors);
    }
}
