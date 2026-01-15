//! Authorization API conversions
//!
//! This module provides conversions between authorization API versions.

use crate::scheme::convert_via_json;
use crate::{ConversionError, Convertible};

// =============================================================================
// SubjectAccessReview: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::authorization::v1::SubjectAccessReview>
    for k8s_api::authorization::v1beta1::SubjectAccessReview
{
    fn convert_to(&self) -> Result<k8s_api::authorization::v1::SubjectAccessReview, ConversionError> {
        let mut converted: k8s_api::authorization::v1::SubjectAccessReview =
            convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "authorization.k8s.io/v1",
            "SubjectAccessReview",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::authorization::v1::SubjectAccessReview,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::authorization::v1beta1::SubjectAccessReview =
            convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "authorization.k8s.io/v1beta1",
            "SubjectAccessReview",
        );
        Ok(converted)
    }
}

// =============================================================================
// SelfSubjectAccessReview: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::authorization::v1::SelfSubjectAccessReview>
    for k8s_api::authorization::v1beta1::SelfSubjectAccessReview
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::authorization::v1::SelfSubjectAccessReview, ConversionError> {
        let mut converted: k8s_api::authorization::v1::SelfSubjectAccessReview =
            convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "authorization.k8s.io/v1",
            "SelfSubjectAccessReview",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::authorization::v1::SelfSubjectAccessReview,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::authorization::v1beta1::SelfSubjectAccessReview =
            convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "authorization.k8s.io/v1beta1",
            "SelfSubjectAccessReview",
        );
        Ok(converted)
    }
}

// =============================================================================
// LocalSubjectAccessReview: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::authorization::v1::LocalSubjectAccessReview>
    for k8s_api::authorization::v1beta1::LocalSubjectAccessReview
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::authorization::v1::LocalSubjectAccessReview, ConversionError> {
        let mut converted: k8s_api::authorization::v1::LocalSubjectAccessReview =
            convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "authorization.k8s.io/v1",
            "LocalSubjectAccessReview",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::authorization::v1::LocalSubjectAccessReview,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::authorization::v1beta1::LocalSubjectAccessReview =
            convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "authorization.k8s.io/v1beta1",
            "LocalSubjectAccessReview",
        );
        Ok(converted)
    }
}

// =============================================================================
// SelfSubjectRulesReview: v1beta1 <-> v1
// =============================================================================

impl Convertible<k8s_api::authorization::v1::SelfSubjectRulesReview>
    for k8s_api::authorization::v1beta1::SelfSubjectRulesReview
{
    fn convert_to(
        &self,
    ) -> Result<k8s_api::authorization::v1::SelfSubjectRulesReview, ConversionError> {
        let mut converted: k8s_api::authorization::v1::SelfSubjectRulesReview =
            convert_via_json(self)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "authorization.k8s.io/v1",
            "SelfSubjectRulesReview",
        );
        Ok(converted)
    }

    fn convert_from(
        other: &k8s_api::authorization::v1::SelfSubjectRulesReview,
    ) -> Result<Self, ConversionError> {
        let mut converted: k8s_api::authorization::v1beta1::SelfSubjectRulesReview =
            convert_via_json(other)?;
        converted.type_meta = k8s_apimachinery::apis::meta::v1::TypeMeta::new(
            "authorization.k8s.io/v1beta1",
            "SelfSubjectRulesReview",
        );
        Ok(converted)
    }
}
