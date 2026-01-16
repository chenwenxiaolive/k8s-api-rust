//! Admission API validation

use crate::{ValidationError, ValidationResult};

const VALID_OPERATIONS: &[&str] = &["CREATE", "UPDATE", "DELETE", "CONNECT"];
const VALID_PATCH_TYPES: &[&str] = &["JSONPatch"];

fn validate_operation(operation: &str, field: &str) -> ValidationResult {
    if operation.is_empty() {
        vec![ValidationError::required(field, "operation is required")]
    } else if !VALID_OPERATIONS.contains(&operation) {
        vec![ValidationError::not_supported(field, operation, VALID_OPERATIONS)]
    } else {
        Vec::new()
    }
}

fn validate_patch(patch: &Option<String>, patch_type: &Option<String>, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    match (patch, patch_type) {
        (Some(_), None) => errors.push(ValidationError::required(
            &format!("{}.patchType", field),
            "patchType is required when patch is set",
        )),
        (None, Some(_)) => errors.push(ValidationError::required(
            &format!("{}.patch", field),
            "patch is required when patchType is set",
        )),
        (Some(_), Some(patch_type)) => {
            if !VALID_PATCH_TYPES.contains(&patch_type.as_str()) {
                errors.push(ValidationError::not_supported(
                    &format!("{}.patchType", field),
                    patch_type,
                    VALID_PATCH_TYPES,
                ));
            }
        }
        _ => {}
    }

    errors
}

fn validate_group_version_kind(
    version: &str,
    kind: &str,
    field: &str,
) -> ValidationResult {
    let mut errors = Vec::new();

    if version.is_empty() {
        errors.push(ValidationError::required(
            &format!("{}.version", field),
            "version is required",
        ));
    }
    if kind.is_empty() {
        errors.push(ValidationError::required(
            &format!("{}.kind", field),
            "kind is required",
        ));
    }

    errors
}

fn validate_group_version_resource(
    version: &str,
    resource: &str,
    field: &str,
) -> ValidationResult {
    let mut errors = Vec::new();

    if version.is_empty() {
        errors.push(ValidationError::required(
            &format!("{}.version", field),
            "version is required",
        ));
    }
    if resource.is_empty() {
        errors.push(ValidationError::required(
            &format!("{}.resource", field),
            "resource is required",
        ));
    }

    errors
}

pub mod v1 {
    use super::*;
    use k8s_api::admission::v1 as api;

    pub fn validate_admission_review(review: &api::AdmissionReview) -> ValidationResult {
        let mut errors = Vec::new();

        if review.request.is_none() && review.response.is_none() {
            errors.push(ValidationError::required(
                "request",
                "request or response is required",
            ));
        }

        if let Some(request) = &review.request {
            errors.extend(validate_admission_request(request, "request"));
        }

        if let Some(response) = &review.response {
            errors.extend(validate_admission_response(response, "response"));
        }

        errors
    }

    fn validate_admission_request(
        request: &api::AdmissionRequest,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        if request.uid.is_empty() {
            errors.push(ValidationError::required(
                &format!("{}.uid", field),
                "uid is required",
            ));
        }

        errors.extend(validate_group_version_kind(
            &request.kind.version,
            &request.kind.kind,
            &format!("{}.kind", field),
        ));
        errors.extend(validate_group_version_resource(
            &request.resource.version,
            &request.resource.resource,
            &format!("{}.resource", field),
        ));
        errors.extend(validate_operation(&request.operation, &format!("{}.operation", field)));

        if let Some(kind) = &request.request_kind {
            errors.extend(validate_group_version_kind(
                &kind.version,
                &kind.kind,
                &format!("{}.requestKind", field),
            ));
        }

        if let Some(resource) = &request.request_resource {
            errors.extend(validate_group_version_resource(
                &resource.version,
                &resource.resource,
                &format!("{}.requestResource", field),
            ));
        }

        errors
    }

    fn validate_admission_response(
        response: &api::AdmissionResponse,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        if response.uid.is_empty() {
            errors.push(ValidationError::required(
                &format!("{}.uid", field),
                "uid is required",
            ));
        }

        errors.extend(validate_patch(
            &response.patch,
            &response.patch_type,
            field,
        ));

        errors
    }
}

pub mod v1beta1 {
    use super::*;
    use k8s_api::admission::v1beta1 as api;

    pub fn validate_admission_review(review: &api::AdmissionReview) -> ValidationResult {
        let mut errors = Vec::new();

        if review.request.is_none() && review.response.is_none() {
            errors.push(ValidationError::required(
                "request",
                "request or response is required",
            ));
        }

        if let Some(request) = &review.request {
            errors.extend(validate_admission_request(request, "request"));
        }

        if let Some(response) = &review.response {
            errors.extend(validate_admission_response(response, "response"));
        }

        errors
    }

    fn validate_admission_request(
        request: &api::AdmissionRequest,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        if request.uid.is_empty() {
            errors.push(ValidationError::required(
                &format!("{}.uid", field),
                "uid is required",
            ));
        }

        errors.extend(validate_group_version_kind(
            &request.kind.version,
            &request.kind.kind,
            &format!("{}.kind", field),
        ));
        errors.extend(validate_group_version_resource(
            &request.resource.version,
            &request.resource.resource,
            &format!("{}.resource", field),
        ));
        errors.extend(validate_operation(&request.operation, &format!("{}.operation", field)));

        if let Some(kind) = &request.request_kind {
            errors.extend(validate_group_version_kind(
                &kind.version,
                &kind.kind,
                &format!("{}.requestKind", field),
            ));
        }

        if let Some(resource) = &request.request_resource {
            errors.extend(validate_group_version_resource(
                &resource.version,
                &resource.resource,
                &format!("{}.requestResource", field),
            ));
        }

        errors
    }

    fn validate_admission_response(
        response: &api::AdmissionResponse,
        field: &str,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        if response.uid.is_empty() {
            errors.push(ValidationError::required(
                &format!("{}.uid", field),
                "uid is required",
            ));
        }

        errors.extend(validate_patch(
            &response.patch,
            &response.patch_type,
            field,
        ));

        errors
    }
}

pub mod internal {
    use super::*;
    use k8s_api::admission::internal as api;

    pub fn validate_admission_review(review: &api::AdmissionReview) -> ValidationResult {
        crate::internal::validate_with(
            review,
            "admissionReview",
            super::v1::validate_admission_review,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::v1 as validation_v1;
    use super::v1beta1 as validation_v1beta1;
    use k8s_api::admission::v1 as api_v1;
    use k8s_api::admission::v1beta1 as api_v1beta1;

    #[test]
    fn test_validate_admission_review_missing_request() {
        let review = api_v1::AdmissionReview::default();
        let errors = validation_v1::validate_admission_review(&review);
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_validate_admission_review_invalid_operation() {
        let review = api_v1::AdmissionReview {
            request: Some(api_v1::AdmissionRequest {
                uid: "123".to_string(),
                kind: api_v1::GroupVersionKind {
                    group: "".to_string(),
                    version: "v1".to_string(),
                    kind: "Pod".to_string(),
                },
                resource: api_v1::GroupVersionResource {
                    group: "".to_string(),
                    version: "v1".to_string(),
                    resource: "pods".to_string(),
                },
                operation: "INVALID".to_string(),
                user_info: api_v1::UserInfo::default(),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validation_v1::validate_admission_review(&review);
        assert!(errors.iter().any(|e| e.field.contains("operation")));
    }

    #[test]
    fn test_validate_admission_response_patch_missing_type() {
        let review = api_v1::AdmissionReview {
            response: Some(api_v1::AdmissionResponse {
                uid: "123".to_string(),
                allowed: true,
                patch: Some("[]".to_string()),
                patch_type: None,
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validation_v1::validate_admission_review(&review);
        assert!(errors.iter().any(|e| e.field.contains("patchType")));
    }

    #[test]
    fn test_validate_admission_review_v1beta1_valid() {
        let review = api_v1beta1::AdmissionReview {
            request: Some(api_v1beta1::AdmissionRequest {
                uid: "abc".to_string(),
                kind: api_v1beta1::GroupVersionKind {
                    group: "".to_string(),
                    version: "v1".to_string(),
                    kind: "Pod".to_string(),
                },
                resource: api_v1beta1::GroupVersionResource {
                    group: "".to_string(),
                    version: "v1".to_string(),
                    resource: "pods".to_string(),
                },
                operation: "CREATE".to_string(),
                user_info: api_v1beta1::UserInfo::default(),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errors = validation_v1beta1::validate_admission_review(&review);
        assert!(errors.is_empty(), "expected no errors: {:?}", errors);
    }
}
