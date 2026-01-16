//! Authentication API conversions
//!
//! This module provides conversions between authentication API versions.

use crate::scheme::convert_via_json;
use crate::{ConversionError, Convertible};

// =============================================================================
// TokenReview: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::authentication::v1::TokenReview>
    for k8s_api::authentication::v1beta1::TokenReview
{
    fn convert_to(&self) -> Result<k8s_api::authentication::v1::TokenReview, ConversionError> {
        let mut converted: k8s_api::authentication::v1::TokenReview = convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "authentication.k8s.io/v1",
            "TokenReview",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::authentication::v1::TokenReview,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::authentication::v1beta1::TokenReview =
            convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "authentication.k8s.io/v1beta1",
            "TokenReview",
        );
        Ok(converted)
    }
}

// =============================================================================
// SelfSubjectReview: v1alpha1/v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::authentication::v1::SelfSubjectReview>
    for k8s_api::authentication::v1beta1::SelfSubjectReview
{
    fn convert_to(&self) -> Result<k8s_api::authentication::v1::SelfSubjectReview, ConversionError> {
        let mut converted: k8s_api::authentication::v1::SelfSubjectReview = convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "authentication.k8s.io/v1",
            "SelfSubjectReview",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::authentication::v1::SelfSubjectReview,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::authentication::v1beta1::SelfSubjectReview =
            convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "authentication.k8s.io/v1beta1",
            "SelfSubjectReview",
        );
        Ok(converted)
    }
}

impl Convertible<k8s_api::authentication::v1::SelfSubjectReview>
    for k8s_api::authentication::v1alpha1::SelfSubjectReview
{
    fn convert_to(&self) -> Result<k8s_api::authentication::v1::SelfSubjectReview, ConversionError> {
        let mut converted: k8s_api::authentication::v1::SelfSubjectReview = convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "authentication.k8s.io/v1",
            "SelfSubjectReview",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::authentication::v1::SelfSubjectReview,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::authentication::v1alpha1::SelfSubjectReview =
            convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "authentication.k8s.io/v1alpha1",
            "SelfSubjectReview",
        );
        Ok(converted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_review_roundtrip() {
        let v1beta1 = k8s_api::authentication::v1beta1::TokenReview {
            spec: k8s_api::authentication::v1beta1::TokenReviewSpec {
                token: "token".to_string(),
                audiences: vec!["aud".to_string()],
            },
            ..Default::default()
        };

        let v1: k8s_api::authentication::v1::TokenReview = v1beta1.convert_to().unwrap();
        assert_eq!(v1.spec.token, "token");
        assert_eq!(v1.spec.audiences, vec!["aud".to_string()]);

        let roundtrip: k8s_api::authentication::v1beta1::TokenReview =
            k8s_api::authentication::v1beta1::TokenReview::convert_from(&v1).unwrap();
        assert_eq!(roundtrip.spec.token, "token");
    }

    #[test]
    fn test_self_subject_review_alpha_roundtrip() {
        let v1alpha1 = k8s_api::authentication::v1alpha1::SelfSubjectReview::default();

        let v1: k8s_api::authentication::v1::SelfSubjectReview = v1alpha1.convert_to().unwrap();
        assert_eq!(v1.type_meta.api_version, "authentication.k8s.io/v1");

        let roundtrip: k8s_api::authentication::v1alpha1::SelfSubjectReview =
            k8s_api::authentication::v1alpha1::SelfSubjectReview::convert_from(&v1).unwrap();
        assert!(roundtrip.status.is_none());
    }
}
