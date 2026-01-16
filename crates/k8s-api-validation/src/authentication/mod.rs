//! Authentication API validation

use crate::common::validate_object_meta;
use crate::{ValidationError, ValidationResult};

const MIN_TOKEN_AGE_SECONDS: i64 = 10 * 60;
const MAX_TOKEN_AGE_SECONDS: i64 = 1i64 << 32;

fn validate_token_request_expiration(expiration: i64, field: &str) -> ValidationResult {
    let mut errors = Vec::new();

    if expiration < MIN_TOKEN_AGE_SECONDS {
        errors.push(ValidationError::invalid(
            field,
            "may not specify a duration less than 10 minutes",
        ));
    }
    if expiration > MAX_TOKEN_AGE_SECONDS {
        errors.push(ValidationError::invalid(
            field,
            "may not specify a duration larger than 2^32 seconds",
        ));
    }

    errors
}

pub mod v1 {
    use super::*;
    use k8s_api::authentication::v1 as api;

    pub fn validate_token_review(review: &api::TokenReview) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&review.metadata, "metadata", false));

        errors
    }

    pub fn validate_token_request(request: &api::TokenRequest) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&request.metadata, "metadata", false));

        if let Some(expiration) = request.spec.expiration_seconds {
            errors.extend(validate_token_request_expiration(
                expiration,
                "spec.expirationSeconds",
            ));
        }

        errors
    }

    pub fn validate_self_subject_review(review: &api::SelfSubjectReview) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&review.metadata, "metadata", false));

        errors
    }
}

pub mod v1beta1 {
    use super::*;
    use k8s_api::authentication::v1beta1 as api;

    pub fn validate_token_review(review: &api::TokenReview) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&review.metadata, "metadata", false));

        errors
    }
}

pub mod v1alpha1 {
    use super::*;
    use k8s_api::authentication::v1alpha1 as api;

    pub fn validate_self_subject_review(review: &api::SelfSubjectReview) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&review.metadata, "metadata", false));

        errors
    }
}

pub mod internal {
    use super::*;
    use k8s_api::authentication::internal as api;

    pub fn validate_token_review(review: &api::TokenReview) -> ValidationResult {
        crate::internal::validate_with(review, "tokenReview", super::v1::validate_token_review)
    }

    pub fn validate_token_request(request: &api::TokenRequest) -> ValidationResult {
        crate::internal::validate_with(
            request,
            "tokenRequest",
            super::v1::validate_token_request,
        )
    }

    pub fn validate_self_subject_review(review: &api::SelfSubjectReview) -> ValidationResult {
        crate::internal::validate_with(
            review,
            "selfSubjectReview",
            super::v1::validate_self_subject_review,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::v1 as validation_v1;
    use k8s_api::authentication::v1 as api_v1;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_validate_token_request_expiration_too_small() {
        let request = api_v1::TokenRequest {
            metadata: ObjectMeta::default(),
            spec: api_v1::TokenRequestSpec {
                audiences: vec!["api".to_string()],
                expiration_seconds: Some(1),
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validation_v1::validate_token_request(&request);
        assert!(!errors.is_empty());
        assert!(errors
            .iter()
            .any(|error| error.field.contains("expirationSeconds")));
    }

    #[test]
    fn test_validate_token_request_expiration_valid() {
        let request = api_v1::TokenRequest {
            metadata: ObjectMeta::default(),
            spec: api_v1::TokenRequestSpec {
                audiences: vec!["api".to_string()],
                expiration_seconds: Some(600),
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validation_v1::validate_token_request(&request);
        assert!(errors.is_empty(), "expected no errors: {:?}", errors);
    }
}
