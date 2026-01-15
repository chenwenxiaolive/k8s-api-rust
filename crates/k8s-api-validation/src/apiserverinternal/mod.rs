//! API Server Internal validation

use crate::common::validate_object_meta;
use crate::{ValidationError, ValidationResult};

const VALID_CONDITION_STATUS: &[&str] = &["True", "False", "Unknown"];

pub mod v1alpha1 {
    use super::*;
    use k8s_api::apiserverinternal::v1alpha1 as api;

    pub fn validate_storage_version(version: &api::StorageVersion) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&version.metadata, "metadata", false));

        for (idx, condition) in version.status.conditions.iter().enumerate() {
            if condition.type_.is_empty() {
                errors.push(ValidationError::required(
                    format!("status.conditions[{}].type", idx),
                    "type is required",
                ));
            }
            if condition.status.is_empty() {
                errors.push(ValidationError::required(
                    format!("status.conditions[{}].status", idx),
                    "status is required",
                ));
            } else if !VALID_CONDITION_STATUS.contains(&condition.status.as_str()) {
                errors.push(ValidationError::not_supported(
                    format!("status.conditions[{}].status", idx),
                    &condition.status,
                    VALID_CONDITION_STATUS,
                ));
            }
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::v1alpha1 as validation_v1alpha1;
    use k8s_api::apiserverinternal::v1alpha1 as api_v1alpha1;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_validate_storage_version_invalid_condition() {
        let version = api_v1alpha1::StorageVersion {
            metadata: ObjectMeta::named("sv"),
            status: api_v1alpha1::StorageVersionStatus {
                conditions: vec![api_v1alpha1::StorageVersionCondition {
                    type_: "".to_string(),
                    status: "Bad".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validation_v1alpha1::validate_storage_version(&version);
        assert!(errors.iter().any(|e| e.field.contains("status.conditions[0]")));
    }

    #[test]
    fn test_validate_storage_version_valid() {
        let version = api_v1alpha1::StorageVersion {
            metadata: ObjectMeta::named("sv"),
            status: api_v1alpha1::StorageVersionStatus {
                conditions: vec![api_v1alpha1::StorageVersionCondition {
                    type_: "AllEncodingVersionsEqual".to_string(),
                    status: "True".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validation_v1alpha1::validate_storage_version(&version);
        assert!(errors.is_empty(), "expected no errors: {:?}", errors);
    }
}
