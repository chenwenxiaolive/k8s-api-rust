//! ImagePolicy API validation

use crate::common::validate_object_meta;
use crate::{ValidationError, ValidationResult};

pub mod v1alpha1 {
    use super::*;
    use k8s_api::imagepolicy::v1alpha1 as api;

    pub fn validate_image_review(review: &api::ImageReview) -> ValidationResult {
        let mut errors = Vec::new();

        errors.extend(validate_object_meta(&review.metadata, "metadata", false));

        if review.spec.containers.is_empty() {
            errors.push(ValidationError::required(
                "spec.containers",
                "containers must not be empty",
            ));
        } else {
            for (idx, container) in review.spec.containers.iter().enumerate() {
                if container.image.is_empty() {
                    errors.push(ValidationError::required(
                        format!("spec.containers[{}].image", idx),
                        "image is required",
                    ));
                }
            }
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::v1alpha1 as validation_v1alpha1;
    use k8s_api::imagepolicy::v1alpha1 as api_v1alpha1;
    use k8s_apimachinery::apis::meta::v1::ObjectMeta;

    #[test]
    fn test_validate_image_review_empty_containers() {
        let review = api_v1alpha1::ImageReview {
            metadata: ObjectMeta::named("review"),
            spec: api_v1alpha1::ImageReviewSpec::default(),
            ..Default::default()
        };

        let errors = validation_v1alpha1::validate_image_review(&review);
        assert!(errors
            .iter()
            .any(|error| error.field.contains("spec.containers")));
    }

    #[test]
    fn test_validate_image_review_empty_image() {
        let review = api_v1alpha1::ImageReview {
            metadata: ObjectMeta::named("review"),
            spec: api_v1alpha1::ImageReviewSpec {
                containers: vec![api_v1alpha1::ImageReviewContainerSpec::default()],
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validation_v1alpha1::validate_image_review(&review);
        assert!(errors
            .iter()
            .any(|error| error.field.contains("spec.containers[0].image")));
    }

    #[test]
    fn test_validate_image_review_valid() {
        let review = api_v1alpha1::ImageReview {
            metadata: ObjectMeta::named("review"),
            spec: api_v1alpha1::ImageReviewSpec {
                containers: vec![api_v1alpha1::ImageReviewContainerSpec {
                    image: "nginx:latest".to_string(),
                }],
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = validation_v1alpha1::validate_image_review(&review);
        assert!(errors.is_empty(), "expected no errors: {:?}", errors);
    }
}
