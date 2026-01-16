//! API Registration validation

use crate::common::validate_object_meta;
use crate::{ValidationError, ValidationResult};

pub mod v1 {
    use super::*;
    use k8s_api::apiregistration::v1 as api;

    pub fn validate_api_service(service: &api::APIService) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&service.metadata, "metadata", true));

        let Some(spec) = &service.spec else {
            errors.push(ValidationError::required("spec", "spec is required"));
            return errors;
        };

        if spec.group.is_empty() {
            errors.push(ValidationError::required(
                "spec.group",
                "group is required",
            ));
        }
        if spec.version.is_empty() {
            errors.push(ValidationError::required(
                "spec.version",
                "version is required",
            ));
        }
        if spec.group_priority_minimum <= 0 {
            errors.push(ValidationError::invalid(
                "spec.groupPriorityMinimum",
                "must be positive",
            ));
        }
        if spec.version_priority <= 0 {
            errors.push(ValidationError::invalid(
                "spec.versionPriority",
                "must be positive",
            ));
        }

        if let Some(service_ref) = &spec.service {
            if service_ref.namespace.is_empty() {
                errors.push(ValidationError::required(
                    "spec.service.namespace",
                    "namespace is required",
                ));
            }
            if service_ref.name.is_empty() {
                errors.push(ValidationError::required(
                    "spec.service.name",
                    "name is required",
                ));
            }
        }

        errors
    }
}

pub mod internal {
    use super::*;
    use k8s_api::apiregistration::internal as api;

    pub fn validate_api_service(service: &api::APIService) -> ValidationResult {
        crate::internal::validate_with(
            service,
            "apiService",
            super::v1::validate_api_service,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::v1 as validation_v1;
    use k8s_api::apiregistration::v1 as api_v1;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_validate_api_service_missing_spec() {
        let service = api_v1::APIService {
            metadata: ObjectMeta::named("v1.foo"),
            spec: None,
            ..Default::default()
        };

        let errors = validation_v1::validate_api_service(&service);
        assert!(errors.iter().any(|e| e.field == "spec"));
    }

    #[test]
    fn test_validate_api_service_missing_group() {
        let service = api_v1::APIService {
            metadata: ObjectMeta::named("v1.foo"),
            spec: Some(api_v1::APIServiceSpec {
                version: "v1".to_string(),
                group_priority_minimum: 10,
                version_priority: 10,
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validation_v1::validate_api_service(&service);
        assert!(errors.iter().any(|e| e.field.contains("spec.group")));
    }

    #[test]
    fn test_validate_api_service_valid() {
        let service = api_v1::APIService {
            metadata: ObjectMeta::named("v1.foo"),
            spec: Some(api_v1::APIServiceSpec {
                group: "example.com".to_string(),
                version: "v1".to_string(),
                group_priority_minimum: 10,
                version_priority: 20,
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validation_v1::validate_api_service(&service);
        assert!(errors.is_empty(), "expected no errors: {:?}", errors);
    }
}
