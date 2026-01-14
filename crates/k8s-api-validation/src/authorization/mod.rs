//! Authorization API validation

use crate::common::{validate_dns_label, validate_object_meta};
use crate::{ValidationError, ValidationResult};
use k8s_apimachinery::apis::meta::v1::ObjectMeta;

fn metadata_is_empty(meta: &ObjectMeta, allow_namespace: bool) -> bool {
    let mut copy = meta.clone();
    copy.managed_fields.clear();
    if allow_namespace {
        copy.namespace.clear();
    }
    copy == ObjectMeta::default()
}

fn validate_metadata_empty(meta: &ObjectMeta, field: &str) -> ValidationResult {
    if metadata_is_empty(meta, false) {
        Vec::new()
    } else {
        vec![ValidationError::invalid(field, "must be empty")]
    }
}

fn validate_metadata_empty_except_namespace(meta: &ObjectMeta, field: &str) -> ValidationResult {
    let mut errors = Vec::new();
    if !metadata_is_empty(meta, true) {
        errors.push(ValidationError::invalid(
            field,
            "must be empty except for namespace",
        ));
    }
    if !meta.namespace.is_empty() {
        errors.extend(validate_dns_label(
            &meta.namespace,
            &format!("{}.namespace", field),
        ));
    }
    errors
}

fn validate_namespace_field(namespace: &str, field: &str) -> ValidationResult {
    if namespace.is_empty() {
        vec![ValidationError::required(field, "namespace is required")]
    } else {
        validate_dns_label(namespace, field)
    }
}

fn validate_subject_access_review_spec(
    has_resource: bool,
    has_non_resource: bool,
    user: &str,
    groups: &[String],
    field: &str,
) -> ValidationResult {
    let mut errors = Vec::new();

    if has_resource && has_non_resource {
        errors.push(ValidationError::invalid(
            format!("{}.nonResourceAttributes", field),
            "cannot be specified in combination with resourceAttributes",
        ));
    }
    if !has_resource && !has_non_resource {
        errors.push(ValidationError::invalid(
            format!("{}.resourceAttributes", field),
            "exactly one of nonResourceAttributes or resourceAttributes must be specified",
        ));
    }
    if user.is_empty() && groups.is_empty() {
        errors.push(ValidationError::invalid(
            format!("{}.user", field),
            "at least one of user or group must be specified",
        ));
    }

    errors
}

fn validate_self_subject_access_review_spec(
    has_resource: bool,
    has_non_resource: bool,
    field: &str,
) -> ValidationResult {
    let mut errors = Vec::new();

    if has_resource && has_non_resource {
        errors.push(ValidationError::invalid(
            format!("{}.nonResourceAttributes", field),
            "cannot be specified in combination with resourceAttributes",
        ));
    }
    if !has_resource && !has_non_resource {
        errors.push(ValidationError::invalid(
            format!("{}.resourceAttributes", field),
            "exactly one of nonResourceAttributes or resourceAttributes must be specified",
        ));
    }

    errors
}

pub mod v1 {
    use super::*;
    use k8s_api::authorization::v1 as api;

    pub fn validate_subject_access_review(review: &api::SubjectAccessReview) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_subject_access_review_spec(
            review.spec.resource_attributes.is_some(),
            review.spec.non_resource_attributes.is_some(),
            &review.spec.user,
            &review.spec.groups,
            "spec",
        ));
        errors.extend(validate_metadata_empty(&review.metadata, "metadata"));

        errors
    }

    pub fn validate_self_subject_access_review(
        review: &api::SelfSubjectAccessReview,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_self_subject_access_review_spec(
            review.spec.resource_attributes.is_some(),
            review.spec.non_resource_attributes.is_some(),
            "spec",
        ));
        errors.extend(validate_metadata_empty(&review.metadata, "metadata"));

        errors
    }

    pub fn validate_local_subject_access_review(
        review: &api::LocalSubjectAccessReview,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_subject_access_review_spec(
            review.spec.resource_attributes.is_some(),
            review.spec.non_resource_attributes.is_some(),
            &review.spec.user,
            &review.spec.groups,
            "spec",
        ));
        errors.extend(validate_metadata_empty_except_namespace(
            &review.metadata,
            "metadata",
        ));

        if let Some(resource_attributes) = &review.spec.resource_attributes {
            if resource_attributes.namespace != review.metadata.namespace {
                errors.push(ValidationError::invalid(
                    "spec.resourceAttributes.namespace",
                    "must match metadata.namespace",
                ));
            }
        }

        if review.spec.non_resource_attributes.is_some() {
            errors.push(ValidationError::invalid(
                "spec.nonResourceAttributes",
                "disallowed on this kind of request",
            ));
        }

        errors
    }

    pub fn validate_self_subject_rules_review(
        review: &api::SelfSubjectRulesReview,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&review.metadata, "metadata", false));
        errors.extend(validate_namespace_field(
            &review.spec.namespace,
            "spec.namespace",
        ));

        errors
    }
}

pub mod v1beta1 {
    use super::*;
    use k8s_api::authorization::v1beta1 as api;

    pub fn validate_subject_access_review(review: &api::SubjectAccessReview) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_subject_access_review_spec(
            review.spec.resource_attributes.is_some(),
            review.spec.non_resource_attributes.is_some(),
            &review.spec.user,
            &review.spec.groups,
            "spec",
        ));
        errors.extend(validate_metadata_empty(&review.metadata, "metadata"));

        errors
    }

    pub fn validate_self_subject_access_review(
        review: &api::SelfSubjectAccessReview,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_self_subject_access_review_spec(
            review.spec.resource_attributes.is_some(),
            review.spec.non_resource_attributes.is_some(),
            "spec",
        ));
        errors.extend(validate_metadata_empty(&review.metadata, "metadata"));

        errors
    }

    pub fn validate_local_subject_access_review(
        review: &api::LocalSubjectAccessReview,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_subject_access_review_spec(
            review.spec.resource_attributes.is_some(),
            review.spec.non_resource_attributes.is_some(),
            &review.spec.user,
            &review.spec.groups,
            "spec",
        ));
        errors.extend(validate_metadata_empty_except_namespace(
            &review.metadata,
            "metadata",
        ));

        if let Some(resource_attributes) = &review.spec.resource_attributes {
            if resource_attributes.namespace != review.metadata.namespace {
                errors.push(ValidationError::invalid(
                    "spec.resourceAttributes.namespace",
                    "must match metadata.namespace",
                ));
            }
        }

        if review.spec.non_resource_attributes.is_some() {
            errors.push(ValidationError::invalid(
                "spec.nonResourceAttributes",
                "disallowed on this kind of request",
            ));
        }

        errors
    }

    pub fn validate_self_subject_rules_review(
        review: &api::SelfSubjectRulesReview,
    ) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&review.metadata, "metadata", false));
        errors.extend(validate_namespace_field(
            &review.spec.namespace,
            "spec.namespace",
        ));

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::v1 as validation_v1;
    use super::v1beta1 as validation_v1beta1;
    use k8s_api::authorization::v1 as api_v1;
    use k8s_api::authorization::v1beta1 as api_v1beta1;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_validate_subject_access_review_requires_user_or_group() {
        let review = api_v1::SubjectAccessReview {
            metadata: ObjectMeta::default(),
            spec: api_v1::SubjectAccessReviewSpec {
                resource_attributes: Some(api_v1::ResourceAttributes::default()),
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validation_v1::validate_subject_access_review(&review);
        assert!(errors.iter().any(|error| error.field.contains("spec.user")));
    }

    #[test]
    fn test_validate_subject_access_review_metadata_empty() {
        let review = api_v1::SubjectAccessReview {
            metadata: ObjectMeta::named("sar"),
            spec: api_v1::SubjectAccessReviewSpec {
                resource_attributes: Some(api_v1::ResourceAttributes::default()),
                user: "alice".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validation_v1::validate_subject_access_review(&review);
        assert!(errors.iter().any(|error| error.field == "metadata"));
    }

    #[test]
    fn test_validate_local_subject_access_review_namespace_mismatch() {
        let review = api_v1::LocalSubjectAccessReview {
            metadata: ObjectMeta {
                namespace: "team-a".to_string(),
                ..Default::default()
            },
            spec: api_v1::SubjectAccessReviewSpec {
                resource_attributes: Some(api_v1::ResourceAttributes {
                    namespace: "team-b".to_string(),
                    ..Default::default()
                }),
                user: "alice".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validation_v1::validate_local_subject_access_review(&review);
        assert!(errors
            .iter()
            .any(|error| error.field.contains("spec.resourceAttributes.namespace")));
    }

    #[test]
    fn test_validate_local_subject_access_review_non_resource_disallowed() {
        let review = api_v1beta1::LocalSubjectAccessReview {
            metadata: ObjectMeta::default(),
            spec: api_v1beta1::SubjectAccessReviewSpec {
                non_resource_attributes: Some(api_v1beta1::NonResourceAttributes {
                    path: "/healthz".to_string(),
                    verb: "get".to_string(),
                }),
                user: "bob".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validation_v1beta1::validate_local_subject_access_review(&review);
        assert!(errors
            .iter()
            .any(|error| error.field.contains("spec.nonResourceAttributes")));
    }
}
