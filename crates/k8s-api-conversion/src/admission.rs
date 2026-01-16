//! Admission API conversions
//!
//! This module provides conversions between admission API versions.

use crate::scheme::convert_via_json;
use crate::{ConversionError, Convertible};

// =============================================================================
// AdmissionReview: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::admission::v1::AdmissionReview>
    for k8s_api::admission::v1beta1::AdmissionReview
{
    fn convert_to(&self) -> Result<k8s_api::admission::v1::AdmissionReview, ConversionError> {
        let mut converted: k8s_api::admission::v1::AdmissionReview = convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "admission.k8s.io/v1",
            "AdmissionReview",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::admission::v1::AdmissionReview,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::admission::v1beta1::AdmissionReview =
            convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "admission.k8s.io/v1beta1",
            "AdmissionReview",
        );
        Ok(converted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admission_review_empty_roundtrip() {
        let v1beta1 = k8s_api::admission::v1beta1::AdmissionReview::default();
        let v1: k8s_api::admission::v1::AdmissionReview = v1beta1.convert_to().unwrap();

        assert!(v1.request.is_none());
        assert!(v1.response.is_none());
        assert_eq!(v1.type_meta.api_version, "admission.k8s.io/v1");

        let roundtrip: k8s_api::admission::v1beta1::AdmissionReview =
            k8s_api::admission::v1beta1::AdmissionReview::convert_from(&v1).unwrap();
        assert!(roundtrip.request.is_none());
    }
}
